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
    ElemType,
    ValType,
    Expr,
};

use std::collections::VecDeque;

pub fn module_instanciate(store: &mut Store, module: Module, externvals: Vec<ExternVal>) -> Result<ModuleInst, Error> {
    let (frame, instrs) = module.instanciate(store, externvals);
    if instrs.is_empty() {
        Ok(frame.module)
    } else {
        Err(Error::Invalid)
    }
}

impl Module {
    fn instanciate(&self, store: &mut Store, externvals: Vec<ExternVal>) -> (Frame, Vec<Instr>) {
        let frame = Frame::default();
        let trap = vec![Instr::Trap];

        let externtypes = match self.validate() {
            Err(_error) => {
                return (frame, trap);
            },
            Ok(externtypes) => externtypes,
        };
        let externtypes_imp = externtypes.0;
        if externtypes_imp.len() != externvals.len() { 
            return (frame, trap);
        }
        let mut globaladdrs = vec![];
        for (ext_val, ext_type) in externvals.iter().zip(externtypes_imp) {
            match ext_val {
                ExternVal::Func(funcaddr) => {
                    let functype = match store.funcs.get(funcaddr.clone()) {
                        None => return (frame, trap),
                        Some(FuncInst::User(funcinst)) => funcinst.tp.clone(),
                        Some(FuncInst::Host(funcinst)) => funcinst.tp.clone(),
                    };
                    if let ExternType::Func(ft) = ext_type {
                        if Module::match_functype(functype, ft) { return (frame, trap); }
                    } else {
                        return (frame, trap);
                    }
                },
                ExternVal::Table(tableaddr) => {
                    let tabletype = match store.tables.get(tableaddr.clone()) {
                        None => return (frame, trap),
                        Some(TableInst{elem, max: m}) => {
                            TableType(Limits{min: elem.len() as u32, max: m.clone()}, ElemType::FuncRef)
                        }
                    };
                    if let ExternType::Table(tt) = ext_type {
                        if Module::match_tabletype(tabletype, tt) { return (frame, trap); }
                    } else {
                        return (frame, trap);
                    }
                },
                ExternVal::Mem(memaddr) => {
                    let memtype = match store.mems.get(memaddr.clone()) {
                        None => return (frame, trap),
                        Some(MemInst{data, max}) => {
                            MemType(Limits{min: (data.len()/64) as u32, max: max.clone()})
                        }
                    };
                    if let ExternType::Mem(mt) = ext_type {
                        if Module::match_memtype(memtype, mt) { return (frame, trap); }
                    } else {
                        return (frame, trap);
                    }
                },
                ExternVal::Global(globaladdr) => {
                    let globaltype = match store.globals.get(globaladdr.clone()) {
                        None => return (frame, trap),
                        Some(GlobalInst{value: val, mutability: mt}) => {
                            let vt = match val {
                                Val::I32Const(_) => ValType::I32,
                                Val::I64Const(_) => ValType::I64,
                                Val::F32Const(_) => ValType::F32,
                                Val::F64Const(_) => ValType::F64,
                            };
                            GlobalType(vt, mt.clone())
                        },
                    };
                    if let ExternType::Global(gt) = ext_type {
                        if Module::match_globaltype(globaltype, gt) { return (frame, trap); }
                    } else {
                        return (frame, trap);
                    }
                    globaladdrs.push(globaladdr.clone());
                },
            }
        }

        let frame = Frame::default();
        let mut thread = Thread::trap_with_frame(store, frame);

        let mut moduleinst_g = ModuleInst::default();
        moduleinst_g.globaladdrs = globaladdrs;
        let frame_g = Frame { module: moduleinst_g, locals: vec![] };
        thread.stack.push(StackEntry::Activation(0, frame_g));
        let mut vals = vec![];
        for global in &self.globals {
            let moduleinst = ModuleInst::default();
            let frame = Frame { module: moduleinst, locals: vec![] };
            vals.push(Self::evaluate_expr(thread.store, frame, global.init.clone()));
        }
        thread.stack.pop();

        let moduleinst = self.alloc_module(thread.store, vec![], vals);

        let mut instr_init_elem_list: VecDeque<Instr> = VecDeque::from(vec![]);
        for elem in &self.elem {
            let frame = Frame{module: moduleinst.clone(), locals: vec![]};
            let eo = if let Val::I32Const(eo) = Self::evaluate_expr(thread.store, frame, elem.offset.clone()) {
                eo
            } else {
                return (thread.frame, trap);
            };
            let tableidx = elem.table;
            let tableaddr = moduleinst.tableaddrs[tableidx as usize];
            let tableinst = &thread.store.tables[tableaddr];
            let eend = eo as usize + elem.init.len();

            if eend > tableinst.elem.len() {
                return (thread.frame, trap);
            }
            instr_init_elem_list.push_back(Instr::InitElem(tableaddr, eo, elem.init.clone()));
        }

        let mut instr_init_data_list: VecDeque<Instr> = VecDeque::from(vec![]);
        for data in &self.data {
            let frame = Frame{module: moduleinst.clone(), locals: vec![]};
            let data_o = if let Val::I32Const(data_o) = Self::evaluate_expr(thread.store, frame, data.offset.clone()) {
                data_o
            } else {
                return (thread.frame, trap);
            };
            let memidx = data.data;
            let memaddr = moduleinst.memaddrs[memidx as usize];
            let meminst = &thread.store.mems[memaddr];
            let dend = data_o as usize + data.init.len();

            if dend > meminst.data.len() {
                return (thread.frame, trap);
            }
            instr_init_data_list.push_back(Instr::InitData(memaddr, data_o, data.init.clone()))
        }

        let frame = Frame { module: moduleinst, locals: vec![] };
        thread.stack.push(StackEntry::Activation(0, frame));
        let frame = if let Some(StackEntry::Activation(0, frame)) = thread.stack.pop() {
            frame
        } else {
            return (thread.frame, trap);
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

        thread.spawn(&mut instrs);

        (thread.frame, vec![])
    }

    pub fn invoke(store: &mut Store, funcaddr: FuncAddr, vals: Vec<Val>) -> (Vec<Val>, Vec<Instr>) {
        let funcinst = if let Some(funcinst) = store.funcs.get(funcaddr) {
            funcinst
        } else {
            return (vec![], vec![Instr::Trap]);
        };
        let (argtypes, returntypes) = match funcinst {
            FuncInst::User(user) => user.tp.clone(),
            FuncInst::Host(host) => host.tp.clone(),
        };
        if vals.len() != argtypes.len() {
            return (vec![], vec![Instr::Trap]);
        }
        for (argtype, val) in argtypes.iter().zip(vals.clone()) {
            let matches = match val {
                Val::I32Const(_) => argtype == &ValType::I32,
                Val::I64Const(_) => argtype == &ValType::I64,
                Val::F32Const(_) => argtype == &ValType::F32,
                Val::F64Const(_) => argtype == &ValType::F64,
            };
            if !matches { return (vec![], vec![Instr::Trap]); }
        }

        let frame = Frame{ module: ModuleInst::default(), locals: vec![] };
        let mut thread = Thread::new_with_frame(store, frame.clone());
        thread.stack.push(StackEntry::Activation(0, frame.clone()));
        let vals: Vec<StackEntry> = vals.clone().iter().map(|v| StackEntry::Value(v.clone())).collect();
        thread.stack.extend(vals);

        let instrs = vec![Instr::Invoke(funcaddr)];
        let mut instrs = VecDeque::from(instrs);
        thread.spawn(&mut instrs);

        let mut returnvals = vec![];
        for _ in 0..returntypes.len() {
            if let Some(StackEntry::Value(v)) = thread.stack.pop() {
                returnvals.push(v);
            }
        }

        (returnvals, vec![])
    }

    fn alloc_module(&self, store: &mut Store, externvals: Vec<ExternVal>, vals: Vec<Val>) -> ModuleInst {
        let mut moduleinst = ModuleInst::default();

        fn hostfunc() {}
        alloc_hostfunc(store, (vec![], vec![]), hostfunc);

        let mut funcaddrs = vec![];
        for func in &self.funcs {
            let funcaddr = alloc_func(store, func, &moduleinst);
            funcaddrs.push(funcaddr);
        }

        let mut tableaddrs = vec![];
        for table in &self.tables {
            let tableaddr = alloc_table(store, table.0.clone());
            tableaddrs.push(tableaddr);
        }

        let mut memaddrs = vec![];
        for mem in &self.mems {
            let memaddr = alloc_mem(store, mem.0.clone());
            memaddrs.push(memaddr);
        }

        let mut globaladdrs = vec![];
        for (i, global) in self.globals.iter().enumerate() {
            let globaladdr = alloc_global(store, global.tp.clone(), vals[i]);
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

    fn evaluate_expr(store: &mut Store, frame: Frame, expr: Expr) -> Val {
        let mut thread = Thread::new_with_frame(store, frame);
        let mut instrs = VecDeque::from(expr.0);
        thread.spawn(&mut instrs);
        if let Some(StackEntry::Value(val)) = thread.stack.pop() {
            val
        } else {
            panic!("evaluate_offset");
        }
    }

    fn match_functype(ft1: FuncType, ft2: FuncType) -> bool {
        // TODO: update match algorithm
        ft1 != ft2
    }

    fn match_tabletype(tt1: TableType, tt2: TableType) -> bool {
        // TODO: update match algorithm
        tt1 != tt2
    }

    fn match_memtype(mt1: MemType, mt2: MemType) -> bool {
        // TODO: update match algorithm
        mt1 != mt2
    }

    fn match_globaltype(gt1: GlobalType, gt2: GlobalType) -> bool {
        // TODO: update match algorithm
        gt1 != gt2
    }
}

fn alloc_func(store: &mut Store, func: &Func, moduleinst: &ModuleInst) -> FuncAddr {
    let addr = store.funcs.len();
    let functype = &moduleinst.types[func.tp as usize];
    let funcinst = FuncInst::user(functype.clone(), moduleinst.clone(), func.clone());
    store.funcs.push(funcinst);
    addr
}

fn alloc_hostfunc(store: &mut Store, functype: FuncType, hostfunc: fn()) -> FuncAddr {
    let addr = store.funcs.len();
    let funcinst = FuncInst::host(functype, hostfunc);
    store.funcs.push(funcinst);
    addr
}

fn alloc_table<'a>(store: &'a mut Store, tabletype: TableType) -> TableAddr {
    let addr = store.tables.len();
    let TableType(Limits{ min: n, max: m }, _) = tabletype;
    let mut elem = vec![];
    for _ in 0..n { elem.push(None) }
    let tableinst = TableInst{ elem: elem, max: m };
    store.tables.push(tableinst);
    addr
}

fn alloc_mem<'a>(store: &'a mut Store, memtype: MemType) -> MemAddr {
    let addr = store.mems.len();
    let MemType(Limits{ min: n, max: m }) = memtype;
    let data = Vec::with_capacity((n * 64) as usize);
    let meminst = MemInst{ data: data, max: m };
    store.mems.push(meminst);
    addr
}

fn alloc_global<'a>(store: &'a mut Store, globaltype: GlobalType, val: Val) -> GlobalAddr {
    let addr = store.globals.len();
    let globalinst = GlobalInst{ value: val, mutability: globaltype.1 };
    store.globals.push(globalinst);
    addr
}

// fn grow_table() {}

// fn grow_mem() {}
