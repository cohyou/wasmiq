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

    fn alloc_module<'a>(&self, store: &'a mut Store, _ext: Vec<ExternVal>, _vals: Vec<Val>) -> ModuleInst {
        let moduleinst = ModuleInst::default();

        fn hostfunc() {}
        alloc_hostfunc(store, (vec![], vec![]), hostfunc);


        use crate::{Expr,};
        let func = Func{ tp: 1, locals: vec![], body: Expr::default() };
        alloc_func(store, func, moduleinst);


        use crate::{ValType, Mut};
        let globaltype = GlobalType(ValType::I32, Mut::Const);
        alloc_global(store, globaltype, Val::I32Const(0));

        // moduleinst
        unimplemented!();
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

fn alloc_func<'a>(store: &'a mut Store, func: Func, moduleinst: ModuleInst) -> (&'a mut Store, FuncAddr) {
    let addr = store.funcs.len();
    let functype = &moduleinst.types[func.tp as usize];
    let funcinst = FuncInst::user(functype.clone(), moduleinst, func);
    store.funcs.push(funcinst);
    (store, addr)
}

fn alloc_hostfunc<'a>(store: &'a mut Store, functype: FuncType, hostfunc: fn()) -> (&'a mut Store, FuncAddr) {
    let addr = store.funcs.len();
    let funcinst = FuncInst::host(functype, hostfunc);
    store.funcs.push(funcinst);
    (store, addr)
}

fn alloc_global<'a>(store: &'a mut Store, globaltype: GlobalType, val: Val) -> (&'a mut Store, GlobalAddr) {
    let addr = store.globals.len();
    let globalinst = GlobalInst{ value: val, mutability: globaltype.1 };
    store.globals.push(globalinst);
    (store, addr)
}
