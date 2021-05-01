use crate::{
    Module,
    Store,
    FuncAddr,
    Val,
    ExecResult,
    Error,
    ValType,
    FuncInst,
    Frame,
    ModuleInst,
    Thread,
    StackEntry,
};

impl Module {
    pub fn invoke(store: &mut Store, funcaddr: FuncAddr, vals: Vec<Val>) -> ExecResult {
        let funcinst = if let Some(funcinst) = store.funcs.get(funcaddr) {
            funcinst
        } else {
            let message = format!("can not get func with addr:{:?} from store.funcs:{:?} (Module::invoke)", funcaddr, store.funcs);
            let error = Error::Invalid(message);
            return ExecResult::Trap(error);
        };
        let (argtypes, returntypes) = match funcinst {
            FuncInst::User(user) => user.tp.clone(),
            FuncInst::Host(host) => host.tp.clone(),
        };
        if vals.len() != argtypes.len() {
            let message = format!("Module invoke vals.len{:?} != argtypes.len{:?}", vals, argtypes);
            return ExecResult::Trap(Error::Invalid(message));
        }

        for (argtype, val) in argtypes.iter().zip(vals.clone()) {
            let matches = match val {
                Val::I32Const(_) => argtype == &ValType::I32,
                Val::I64Const(_) => argtype == &ValType::I64,
                Val::F32Const(_) => argtype == &ValType::F32,
                Val::F64Const(_) => argtype == &ValType::F64,
            };
            if !matches { return ExecResult::Trap(Error::Invalid("Module::invoke !matches".to_owned())); }
        }

        let dummy_frame = Frame{ module: ModuleInst::default(), locals: vec![] };
        let mut thread = Thread::new(store);
        thread.stack.push(StackEntry::Activation(0, dummy_frame.clone()));
        let vals: Vec<StackEntry> = vals.clone().iter().map(|v| StackEntry::Value(v.clone())).collect();
        thread.stack.extend(vals);

        let mut thread = Thread::new(store);
        let mut returnvals = vec![];
        match thread.execute_invoke(&funcaddr) {
            ExecResult::Vals(mut vals) => {
                for _ in 0..returntypes.len() {
                    if let Some(v) = vals.pop() {
                        returnvals.push(v);
                    }
                }
                ExecResult::Vals(returnvals)
            },
            ExecResult::Trap(err) => ExecResult::Trap(err),
            ExecResult::Return(mut vals) => {
                for _ in 0..returntypes.len() {
                    if let Some(v) = vals.pop() {
                        returnvals.push(v);
                    }
                }
                ExecResult::Vals(returnvals)
            }
        }

        
    }
}
