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

    // Instr,
    // IUnOp,
    // ValSize,
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
        let externtypes = match self.validate() {
            Err(_error) => {
                return (store, Thread::trap_with_frame(frame));
            },
            Ok(externtypes) => externtypes,
        };
        let externtypes_imp = externtypes.0;
        if externtypes_imp.len() != externvals.len() { 
            return (store, Thread::trap_with_frame(frame));
        }
        for (ext_val, ext_type) in externvals.iter().zip(externtypes_imp) {
            match ext_val {
                ExternVal::Func(a) => {
                    let functype = match store.funcs.get(a.clone()) {
                        None => return (store, Thread::trap_with_frame(frame)),
                        Some(FuncInst::User(funcinst)) => funcinst.tp.clone(),
                        Some(FuncInst::Host(funcinst)) => funcinst.tp.clone(),
                    };
                    if let ExternType::Func(ft) = ext_type {
                        if functype != ft {
                            return (store, Thread::trap_with_frame(frame));
                        }
                    } else {
                        return (store, Thread::trap_with_frame(frame));
                    }
                },
                ExternVal::Table(_) => unimplemented!(),
                ExternVal::Mem(_) => unimplemented!(),
                ExternVal::Global(_) => unimplemented!(),
            }
        }

        let mut thread = Thread::trap_with_frame(frame);
        thread.spawn(&mut VecDeque::from(vec![]));

        unimplemented!()
    }
}
