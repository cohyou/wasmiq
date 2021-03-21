use crate::{
    ValType as ValTypeOriginal,
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
    vt_rev,
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
    pub fn validate_instr_sequence(context: &Context, instrs: &Vec<Instr>) -> Result<FuncType, Error> {
        if instrs.is_empty() { return instr_tp!(Ellipsis -> Ellipsis); }

        let mut ret = Err(Error::Invalid);
        let mut rets: ResultType; 

        let mut instr_resolved: Option<Instr> = None;

        for instr_pair in instrs.windows(2) {
            let first_functype = {
                if let Some(instr) = instr_resolved {
                    instr_resolved = None;
                    instr.validate(context)?
                } else {
                    instr_pair[0].validate(context)?
                }
            };
            let instr_second = &instr_pair[1];
            let second_functype = instr_second.validate(context)?;
            let mut second_functype_args = second_functype.0;
            let mut second_functype_rets = second_functype.1;


            // resolve valtype for value-polymorphic instrs
            // TODO: better algorithm...
            match instr_second {
                &Instr::Drop(None) => {
                    let valtype = first_functype.1.last().ok_or(Error::Invalid)?;
                    second_functype_args = ResultType(vec![valtype.clone()]);
                    instr_resolved = Some(Instr::Drop(Some(vt_rev(valtype))));
                },
                &Instr::Select(None) => {
                    let valtype = first_functype.1.last2().ok_or(Error::Invalid)?;
                    second_functype_args = ResultType(vec![valtype.clone(), valtype.clone(), ValType::I32]);
                    second_functype_rets = ResultType(vec![valtype.clone()]);
                    instr_resolved = Some(Instr::Select(Some(vt_rev(valtype))));
                },
                _ => (),
            }


            // compare types
            rets = first_functype.1.strip_suffix(&second_functype_args).ok_or(Error::Invalid)?;
            

            let ret_args = first_functype.0;
            let ret_rets = {
                if ResultType::is_stack_polymorphic(&second_functype_rets) {
                    second_functype_rets
                } else {
                    rets.0.extend(second_functype_rets.0);
                    rets.clone()
                }
            };
            ret = Ok((ret_args, ret_rets));
        }
        
        ret
    }
}