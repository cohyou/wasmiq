use super::*;

impl<'a> Thread<'a> {
    pub fn execute_drop(&mut self) -> ExecResult {
        self.stack.pop();
        ExecResult::Vals(vec![])
    }

    pub fn execute_select(&mut self) -> ExecResult {
        let c = self.stack.pop().unwrap();
        let v2 = self.stack.pop().unwrap();
        let v1 = self.stack.pop().unwrap();
        if let StackEntry::Value(Val::I32Const(c)) = c {
            if c != 0 {
                if let StackEntry::Value(v1) = v1 {
                    ExecResult::Vals(vec![v1])
                } else {
                    unreachable!()
                }
            } else {
                if let StackEntry::Value(v2) = v2 {
                    ExecResult::Vals(vec![v2])
                } else {
                    unreachable!()
                }
            }
        } else {
            unreachable!()
        }
    }
}