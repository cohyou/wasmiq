use super::*;

impl<'a> Thread<'a> {
    /* integer */

    pub fn execute_ieqz32(&mut self) -> Result { self.execute_itestop32(ieqz32) }

    fn execute_itestop32(&mut self, func: fn(u32) -> u32) -> Result {
        // assert!(vals.len() >= 1);
        if let Some(StackEntry::Value(Val::I32Const(c))) = self.stack.pop() {
            Result::Vals(vec![Val::I32Const(func(c))])
        } else {
            Result::Trap
        }
    }


    /* floating-point number */

    pub fn execute_ieqz64(&mut self) -> Result { self.execute_itestop64(ieqz64) }

    fn execute_itestop64(&mut self, func: fn(u64) -> u64) -> Result {
        // assert!(vals.len() >= 1);
        if let Some(StackEntry::Value(Val::I64Const(c))) = self.stack.pop() {
            Result::Vals(vec![Val::I64Const(func(c))])
        } else {
            Result::Trap
        }
    }
}