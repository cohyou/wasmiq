use crate::{
    Store,
    Module,
    ExternVal,
    ModuleInst,
    Error,
    Frame,

    FuncInst,
    ExternType,
    Thread,
    StackEntry,
    Val,
    Instr,
    Start,
    FuncAddr,
    GlobalType,
    GlobalAddr,
    GlobalInst,
    Func,
    FuncType,
    TableInst,
    Limits,
    TableType,
    TableAddr,
    MemAddr,
    MemType,
    MemInst,
    Export,
    ExportDesc,
    ExportInst,
};

use std::collections::VecDeque;

pub fn module_instanciate<'a >(store: &'a mut Store, module: Module, externvals: Vec<ExternVal>) -> (&'a mut Store, Result<ModuleInst, Error>) {
    let (store, thread) = module.instanciate(store, externvals);
    if thread.instrs.is_empty() {
        (store, Ok(thread.frame.module))
    } else {
        (store, Err(Error::Invalid))
    }
}

impl Module {
    fn instanciate<'a>(&self, store: &'a mut Store, externvals: Vec<ExternVal>) -> (&'a mut Store, Thread) {
        let frame = Frame::default();
        let thread_trap = Thread::trap_with_frame(frame);
        let externtypes = match self.validate() {
            Err(_error) => {
                return (store, thread_trap);
            },
            Ok(externtypes) => externtypes,
        };
        let externtypes_imp = externtypes.0;
        if externtypes_imp.len() != externvals.len() { 
            return (store, thread_trap);
        }
        let mut globaladdrs = vec![];
        for (ext_val, ext_type) in externvals.iter().zip(externtypes_imp) {
            match ext_val {
                ExternVal::Func(funcaddr) => {
                    let functype = match store.funcs.get(funcaddr.clone()) {
                        None => return (store, thread_trap),
                        Some(FuncInst::User(funcinst)) => funcinst.tp.clone(),
                        Some(FuncInst::Host(funcinst)) => funcinst.tp.clone(),
                    };
                    if let ExternType::Func(ft) = ext_type {
                        if functype != ft {
                            return (store, thread_trap);
                        }
                    } else {
                        return (store, thread_trap);
                    }
                },
                ExternVal::Table(_) => unimplemented!(),
                ExternVal::Mem(_) => unimplemented!(),
                ExternVal::Global(globaladdr) => {
                    globaladdrs.push(globaladdr.clone());
                },
            }
        }

        let frame = Frame::default();
        let mut thread = Thread::trap_with_frame(frame);

        let mut moduleinst_g = ModuleInst::default();
        moduleinst_g.globaladdrs = globaladdrs;
        let frame_g = Frame { module: moduleinst_g, locals: vec![] };
        thread.stack.push(StackEntry::Activation(0, frame_g));
        let mut vals = vec![];
        for _global in &self.globals {
            vals.push(Self::evaluate_global());
        }
        thread.stack.pop();

        let moduleinst = self.alloc_module(store, vec![], vals);

        let mut instr_init_elem_list: VecDeque<Instr> = VecDeque::from(vec![]);
        for elem in &self.elem {
            let eo = if let Val::I32Const(eo) = Self::evaluate_elem() {
                eo
            } else {
                return (store, thread_trap);
            };
            let tableidx = elem.table;
            let tableaddr = moduleinst.tableaddrs[tableidx as usize];
            let tableinst = &store.tables[tableaddr];
            let eend = eo as usize + elem.init.len();

            if eend > tableinst.elem.len() {
                return (store, thread_trap);
            }
            instr_init_elem_list.push_back(Instr::InitElem(tableaddr, eo, elem.init.clone()));
        }

        let mut instr_init_data_list: VecDeque<Instr> = VecDeque::from(vec![]);
        for data in &self.data {
            let data_o = if let Val::I32Const(data_o) = Self::evaluate_data() {
                data_o
            } else {
                return (store, thread_trap);
            };
            let memidx = data.data;
            let memaddr = moduleinst.memaddrs[memidx as usize];
            let meminst = &store.mems[memaddr];
            let dend = data_o as usize + data.init.len();

            if dend > meminst.data.len() {
                return (store, thread_trap);
            }
            instr_init_data_list.push_back(Instr::InitData(memaddr, data_o, data.init.clone()))
        }

        let frame = Frame { module: moduleinst, locals: vec![] };
        thread.stack.push(StackEntry::Activation(0, frame));
        let frame = if let Some(StackEntry::Activation(0, frame)) = thread.stack.pop() {
            frame
        } else {
            return (store, thread_trap);
        };

        let mut instrs = VecDeque::new();

        while let Some(init_elem) = instr_init_elem_list.pop_front() {
            instrs.push_back(init_elem);
        }
        while let Some(init_data) = instr_init_data_list.pop_front() {
            instrs.push_back(init_data);
        }

        if let Some(Start(funcidx)) = &self.start {
            let funcaddr = frame.module.funcaddrs[funcidx.clone() as usize];
            instrs.push_back(Instr::Invoke(funcaddr.clone()));
        }

        thread.spawn(store, &mut instrs);

        unimplemented!()
    }

    pub fn invoke<'a>(_store: &'a mut Store, _funcaddr: FuncAddr, _vals: Vec<Val>) -> (&'a mut Store, Thread) {
        unimplemented!();
    }

    fn alloc_module<'a>(&self, store: &'a mut Store, externvals: Vec<ExternVal>, vals: Vec<Val>) -> ModuleInst {
        let mut moduleinst = ModuleInst::default();

        fn hostfunc() {}
        alloc_hostfunc(store, (vec![], vec![]), hostfunc);

        let mut funcaddrs = vec![];
        for func in &self.funcs {
            let (_, funcaddr) = alloc_func(store, func, &moduleinst);
            funcaddrs.push(funcaddr);
        }

        let mut tableaddrs = vec![];
        for table in &self.tables {
            let (_, tableaddr) = alloc_table(store, table.0.clone());
            tableaddrs.push(tableaddr);
        }

        let mut memaddrs = vec![];
        for mem in &self.mems {
            let (_, memaddr) = alloc_mem(store, mem.0.clone());
            memaddrs.push(memaddr);
        }

        let mut globaladdrs = vec![];
        for (i, global) in self.globals.iter().enumerate() {
            let (_, globaladdr) = alloc_global(store, global.tp.clone(), vals[i]);
            globaladdrs.push(globaladdr);
        }


        let mut funcaddrs_ext = vec![];
        let mut tableaddrs_ext = vec![];
        let mut memaddrs_ext = vec![];
        let mut globaladdrs_ext = vec![];
        for externval in externvals {
            match externval {
                ExternVal::Func(func) => funcaddrs_ext.push(func),
                ExternVal::Global(global) => globaladdrs_ext.push(global),
                ExternVal::Mem(mem) => memaddrs_ext.push(mem),
                ExternVal::Table(table) => tableaddrs_ext.push(table),
            }
        }

        funcaddrs_ext.extend(funcaddrs);
        tableaddrs_ext.extend(tableaddrs);
        memaddrs_ext.extend(memaddrs);
        globaladdrs_ext.extend(globaladdrs);

        
        let mut exportinsts = vec![];
        for export in &self.exports {
            let Export{ name: _, desc} = export;
            let externval = match desc {
                ExportDesc::Func(funcidx) => ExternVal::Func(funcaddrs_ext[funcidx.clone() as usize]),
                ExportDesc::Table(tableidx) => ExternVal::Table(tableaddrs_ext[tableidx.clone() as usize]),
                ExportDesc::Mem(memidx) => ExternVal::Mem(memaddrs_ext[memidx.clone() as usize]),
                ExportDesc::Global(globalidx) => ExternVal::Global(globaladdrs_ext[globalidx.clone() as usize]),
            };
            let exportinst = ExportInst{ name: export.name.clone(), value: externval };
            exportinsts.push(exportinst);
        }

        moduleinst.types = self.types.clone();
        moduleinst.funcaddrs = funcaddrs_ext;
        moduleinst.tableaddrs = tableaddrs_ext;
        moduleinst.memaddrs = memaddrs_ext;
        moduleinst.globaladdrs = globaladdrs_ext;
        moduleinst.exports = exportinsts;

        moduleinst
    }

    fn evaluate_global() -> Val {
        unimplemented!();
    }

    fn evaluate_elem() -> Val {
        unimplemented!();
    }

    fn evaluate_data() -> Val {
        unimplemented!();
    }
}

fn alloc_func<'a>(store: &'a mut Store, func: &Func, moduleinst: &ModuleInst) -> (&'a mut Store, FuncAddr) {
    let addr = store.funcs.len();
    let functype = &moduleinst.types[func.tp as usize];
    let funcinst = FuncInst::user(functype.clone(), moduleinst.clone(), func.clone());
    store.funcs.push(funcinst);
    (store, addr)
}

fn alloc_hostfunc<'a>(store: &'a mut Store, functype: FuncType, hostfunc: fn()) -> (&'a mut Store, FuncAddr) {
    let addr = store.funcs.len();
    let funcinst = FuncInst::host(functype, hostfunc);
    store.funcs.push(funcinst);
    (store, addr)
}

fn alloc_table<'a>(store: &'a mut Store, tabletype: TableType) -> (&'a mut Store, TableAddr) {
    let addr = store.tables.len();
    let TableType(Limits{ min: n, max: m }, _) = tabletype;
    let mut elem = vec![];
    for _ in 0..n { elem.push(None) }
    let tableinst = TableInst{ elem: elem, max: m };
    store.tables.push(tableinst);
    (store, addr)
}

fn alloc_mem<'a>(store: &'a mut Store, memtype: MemType) -> (&'a mut Store, MemAddr) {
    let addr = store.mems.len();
    let MemType(Limits{ min: n, max: m }) = memtype;
    let data = Vec::with_capacity((n * 64) as usize);
    let meminst = MemInst{ data: data, max: m };
    store.mems.push(meminst);
    (store, addr)
}

fn alloc_global<'a>(store: &'a mut Store, globaltype: GlobalType, val: Val) -> (&'a mut Store, GlobalAddr) {
    let addr = store.globals.len();
    let globalinst = GlobalInst{ value: val, mutability: globaltype.1 };
    store.globals.push(globalinst);
    (store, addr)
}

// fn grow_table() {}

// fn grow_mem() {}
