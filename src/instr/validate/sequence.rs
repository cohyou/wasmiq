use crate::{
    Error,
    Context,
    ft,
    instr_tp,
};

use super::{
    Instr,
    ValType,
    ResultType,
    FuncType,
};


impl ResultType {
    pub fn strip_suffix<'a>(&'a self, suffix: &ResultType) -> Option<ResultType> {
        let mut res = ResultType(vec![]);

        if ResultType::is_stack_polymorphic(&self)
        || ResultType::is_stack_polymorphic(suffix) {
            return Some(res);
        }

        if self.0.len() < suffix.0.len() { return None; }

        let max_idx = &self.0.len() - 1;
        for (idx, valtype) in suffix.iter().rev().enumerate() {
            let valtype1 = &self.0.get(max_idx - idx).unwrap();
            if &valtype == valtype1 {
                res.0.insert(0, valtype.clone());
            } else {
                return None;
            }
        }

        Some(res)
    }

    pub fn is_stack_polymorphic(rt: &ResultType) -> bool {
        rt.0.starts_with(&[ValType::Ellipsis])
    }
}

impl Instr {
    fn validate_instr_sequence(context: &Context, instrs: Vec<Instr>) -> Result<FuncType, Error> {
        if let Some(instr) = instrs.first() {
            let args = instr.validate(context)?.0;
            let mut ret = Err(Error::Invalid);
            let mut rets: ResultType; 

            for instr_pair in instrs.windows(2) {
                let first_functype = instr_pair[0].validate(context)?;
                let second_functype = instr_pair[1].validate(context)?;

                // compare types
                rets = first_functype.1.strip_suffix(&second_functype.0).ok_or(Error::Invalid)?;
                let ret_args = first_functype.0;
                let ret_rets = {
                    if ResultType::is_stack_polymorphic(&second_functype.1) {
                        second_functype.1
                    } else {
                        rets.0.extend(second_functype.1.0);
                        rets.clone()
                    }
                };
                ret = Ok((ret_args, ret_rets));
            }
            ret
        } else {
            instr_tp!(Ellipsis -> Ellipsis)
        }
    }
}