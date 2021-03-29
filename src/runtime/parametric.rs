use super::*;

impl<'a> Thread<'a> {
    pub fn execute_drop(&mut self) -> Result {
        self.stack.pop();
        Result::Vals(vec![])
    }

    pub fn execute_select(&mut self) -> Result {
        let c = self.stack.pop().unwrap();
        let v2 = self.stack.pop().unwrap();
        let v1 = self.stack.pop().unwrap();
        if let StackEntry::Value(Val::I32Const(c)) = c {
            if c != 0 {
                if let StackEntry::Value(v1) = v1 {
                    Result::Vals(vec![v1])
                } else {
                    unreachable!()
                }
            } else {
                if let StackEntry::Value(v2) = v2 {
                    Result::Vals(vec![v2])
                } else {
                    unreachable!()
                }
            }
        } else {
            unreachable!()
        }
    }
}