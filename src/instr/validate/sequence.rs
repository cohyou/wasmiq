use crate::{
    // ValType as ValTypeOriginal,
    Expr,
    Error,
    Context,
    Mut,
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

impl Expr {
    pub fn validate(&self, context: &Context) -> Result<ResultType, Error> {
        let functype = Instr::validate_instr_sequence(context, &self.0)?;
        if functype.0.0.len() > 0 {
            return Err(Error::Invalid("Expr::validate functype.0.0.len() > 0".to_owned()));
        }
        Ok(ResultType(functype.1.0))
    }

    pub fn is_constant(&self, context: &Context) -> bool {
        self.0.iter().all(|instr| instr.is_constant(context))
    }
}

impl Instr {
    pub fn is_constant(&self, context: &Context) -> bool {
        match self {
            Instr::I32Const(_) | 
            Instr::I64Const(_) |
            Instr::F32Const(_) |
            Instr::F64Const(_) => {
                true
            },
            Instr::GlobalGet(x) => {
                if let Some(globaltype) = context.global(x.clone()) {
                    globaltype.1 == Mut::Const
                } else {
                    false
                }
            },
            _ => false,
        }
    }
}

impl ResultType {
    pub fn strip_suffix<'a>(&'a self, suffix: &ResultType) -> Result<ResultType, Error> {
        let mut res = ResultType(self.0.clone());

        if ResultType::is_stack_polymorphic(&self)
        || ResultType::is_stack_polymorphic(suffix) {
            return Ok(ResultType(vec![]));
        }

        if self.0.len() < suffix.0.len() {
            let message = format!("ResultType::strip_suffix different resulttype length: {:?} {:?}", self.0, suffix.0);
            return Err(Error::Invalid(message));
        }

        let max_idx = &self.0.len() - 1;
        for (idx, valtype) in suffix.iter().rev().enumerate() {
            let valtype1 = &self.0.get(max_idx - idx).unwrap();
            if &valtype == valtype1 {
                // res.0.insert(0, valtype.clone());
                res.0.pop();
            } else {
                return Err(Error::Invalid("ResultType::strip_suffix &valtype == valtype1".to_owned()));
            }
        }

        Ok(res)
    }

    pub fn is_stack_polymorphic(rt: &ResultType) -> bool {
        rt.0.starts_with(&[ValType::Ellipsis])
    }
}

impl Instr {
    pub fn validate_instr_sequence(context: &Context, instrs: &Vec<Instr>) -> Result<FuncType, Error> {
        if instrs.is_empty() { return instr_tp!(Ellipsis -> Ellipsis); }
        if instrs.len() ==  1 {
            return instrs[0].validate(context);
        }

        let mut ret: Result<FuncType, Error> = Err(Error::Invalid("Instr::validate_instr_sequence default".to_owned()));
        let mut rets: ResultType; 

        let mut instr_resolved: Option<Instr> = None;

        for instr_pair in instrs.windows(2) {
            let first_functype = {
                match ret {
                    Ok(ft) => ft,
                    _ => {
                        if let Some(instr) = instr_resolved {
                            instr_resolved = None;
                            instr.validate(context)?
                        } else {
                            instr_pair[0].validate(context)?
                        }
                    }
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
                    let valtype = first_functype.1.last()
                        .ok_or(Error::Invalid("validate_instr_sequence instr_second Drop first_functype.1.last()".to_owned()))?;
                    second_functype_args = ResultType(vec![valtype.clone()]);
                    instr_resolved = Some(Instr::Drop(Some(vt_rev(valtype))));
                },
                &Instr::Select(None) => {
                    let valtype = first_functype.1.last2()
                        .ok_or(Error::Invalid("validate_instr_sequence instr_second Select first_functype.1.last2()".to_owned()))?;
                    second_functype_args = ResultType(vec![valtype.clone(), valtype.clone(), ValType::I32]);
                    second_functype_rets = ResultType(vec![valtype.clone()]);
                    instr_resolved = Some(Instr::Select(Some(vt_rev(valtype))));
                },
                _ => (),
            }


            // compare types
            rets = first_functype.1.strip_suffix(&second_functype_args)?;

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


#[test]
fn test_validate_instrs() {
    let instrs = vec![
        Instr::I32Const(42),
    ];
    test_validate(instrs, (ResultType(vec![]), ResultType(vec![ValType::I32])));
    
    let instrs = vec![
        Instr::I32Const(42),
        Instr::I32Const(42),
    ];
    test_validate(instrs, (ResultType(vec![]), ResultType(vec![ValType::I32, ValType::I32])));

    use crate::{ValSize, IBinOp};
    let instrs = vec![
        Instr::I32Const(1), 
        Instr::I32Const(2),
        Instr::IBinOp(ValSize::V32, IBinOp::Add),
    ];
    test_validate(instrs, (ResultType(vec![]), ResultType(vec![ValType::I32])));
}

#[allow(dead_code)]
fn test_validate(instrs: Vec<Instr>, functype: FuncType) {
    let context = Context::default();
    let validation = Instr::validate_instr_sequence(&context, &instrs);
    assert_eq!(validation, Ok(functype));
}