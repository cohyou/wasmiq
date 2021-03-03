use crate::{
    Error,
    Context,
};

use super::{
    Instr,
    ValType,
    FuncType,
};

impl Instr {
    fn validate_instr_sequence(context: &Context, instrs: Vec<Instr>) -> Result<FuncType, Error> {
        if let Some(instr) = instrs.first() {
            let args = instr.validate(context)?.0;
            for instr_pair in instrs.windows(2) {
                let first_functype = instr_pair[0].validate(context)?;
                let second_functype = instr_pair[1].validate(context)?;

                // compare types
                let _ = Instr::suffixes(first_functype.1, second_functype.0)?;
            }    
        } else {
            unimplemented!()
        }
        unimplemented!()
    }

    fn suffixes(first_rets: Vec<ValType>, second_args: Vec<ValType>) -> Result<(), Error> {
        for (i, arg) in second_args.iter().rev().enumerate() {
            if arg != &first_rets[first_rets.len() - i - 1] {
                return Err(Error::Invalid);
            }
        }
        Ok(())
    }
}