use crate::{
    Store,
    Module,
    ExternVal,
    ModuleInst,
    Error,
    Frame,
    Instr,
    FuncInst,
    ExternType,
    Thread,
};

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
        let externtypes = match self.validate() {
            Err(error) => {
                return (store, Thread { frame: frame, instrs: vec![Instr::Trap] });
            },
            Ok(externtypes) => externtypes,
        };
        let externtypes_imp = externtypes.0;
        if externtypes_imp.len() != externvals.len() { 
            return (store, Thread { frame: frame, instrs: vec![Instr::Trap] });
        }
        for (ext_val, ext_type) in externvals.iter().zip(externtypes_imp) {
            match ext_val {
                ExternVal::Func(a) => {
                    let functype = match store.funcs.get(a.clone()) {
                        None => return (store, Thread { frame: frame, instrs: vec![Instr::Trap] }),
                        Some(FuncInst::User(funcinst)) => funcinst.tp.clone(),
                        Some(FuncInst::Host(funcinst)) => funcinst.tp.clone(),
                    };
                    if let ExternType::Func(ft) = ext_type {
                        if functype != ft {
                            return (store, Thread { frame: frame, instrs: vec![Instr::Trap] });
                        }
                    } else {
                        return (store, Thread { frame: frame, instrs: vec![Instr::Trap] });
                    }
                },
                ExternVal::Table(a) => unimplemented!(),
                ExternVal::Mem(a) => unimplemented!(),
                ExternVal::Global(a) => unimplemented!(),
            }
        }

        unimplemented!()
    }
}