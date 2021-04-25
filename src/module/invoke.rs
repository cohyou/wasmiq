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
            return ExecResult::Trap(Error::Invalid("Module::invoke store.funcs.get(funcaddr) is None".to_owned()));
        };
        let (argtypes, returntypes) = match funcinst {
            FuncInst::User(user) => user.tp.clone(),
            FuncInst::Host(host) => host.tp.clone(),
        };
        if vals.len() != argtypes.len() {
            return ExecResult::Trap(Error::Invalid("Module::invoke vals.len() != argtypes.len()".to_owned()));
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
        if let ExecResult::Vals(mut vals) = thread.execute_invoke(&funcaddr) {
            for _ in 0..returntypes.len() {
                if let Some(v) = vals.pop() {
                    returnvals.push(v);
                }
            }    
        }

        ExecResult::Vals(returnvals)
    }
}
