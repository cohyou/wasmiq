use super::*;

impl<'a> Thread<'a> {
    /* integer */

    pub fn execute_ieqz32(&mut self) -> ExecResult { self.execute_itestop32(ieqz32) }

    fn execute_itestop32(&mut self, func: fn(u32) -> u32) -> ExecResult {
        // assert!(vals.len() >= 1);
        if let Some(StackEntry::Value(Val::I32Const(c))) = self.stack.pop() {
            ExecResult::Vals(vec![Val::I32Const(func(c))])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_itestop32 Some(StackEntry::Value(Val::I32Const(c))) = self.stack.pop()".to_owned()))
        }
    }


    /* floating-point number */

    pub fn execute_ieqz64(&mut self) -> ExecResult { self.execute_itestop64(ieqz64) }

    fn execute_itestop64(&mut self, func: fn(u64) -> u64) -> ExecResult {
        // assert!(vals.len() >= 1);
        if let Some(StackEntry::Value(Val::I64Const(c))) = self.stack.pop() {
            ExecResult::Vals(vec![Val::I64Const(func(c))])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_itestop64 Some(StackEntry::Value(Val::I64Const(c))) = self.stack.pop()".to_owned()))
        }
    }
}