use super::*;

impl<'a> Thread<'a> {
    /* integer */

    pub fn execute_iadd32(&mut self) -> ExecResult { self.execute_ibinop32(iadd32) }
    pub fn execute_isub32(&mut self) -> ExecResult { self.execute_ibinop32(isub32) }
    pub fn execute_imul32(&mut self) -> ExecResult { self.execute_ibinop32(imul32) }
    pub fn execute_idiv_u32(&mut self) -> ExecResult { self.execute_ibinop32(idiv_u32) }
    pub fn execute_idiv_s32(&mut self) -> ExecResult { self.execute_ibinop32(idiv_s32) }
    pub fn execute_irem_u32(&mut self) -> ExecResult { self.execute_ibinop32(irem_u32) }
    pub fn execute_irem_s32(&mut self) -> ExecResult { self.execute_ibinop32(irem_s32) }
    pub fn execute_iand32(&mut self) -> ExecResult { self.execute_ibinop32(iand32) }
    pub fn execute_ior32(&mut self) -> ExecResult { self.execute_ibinop32(ior32) }
    pub fn execute_ixor32(&mut self) -> ExecResult { self.execute_ibinop32(ixor32) }
    pub fn execute_ishl32(&mut self) -> ExecResult { self.execute_ibinop32(ishl32) }
    pub fn execute_ishr_u32(&mut self) -> ExecResult { self.execute_ibinop32(ishr_u32) }
    pub fn execute_ishr_s32(&mut self) -> ExecResult { self.execute_ibinop32(ishr_s32) }
    pub fn execute_irotl32(&mut self) -> ExecResult { self.execute_ibinop32(irotl32) }
    pub fn execute_irotr32(&mut self) -> ExecResult { self.execute_ibinop32(irotr32) }

    fn execute_ibinop32(&mut self, func: fn(u32, u32) -> u32) -> ExecResult {
        // assert!(self.stack.len() >= 2);
        if let Some(StackEntry::Value(Val::I32Const(c2))) = self.stack.pop() {
            if let Some(StackEntry::Value(Val::I32Const(c1))) = self.stack.pop() {
                ExecResult::Vals(vec![Val::I32Const(func(c1, c2))])
            } else {
                ExecResult::Trap(Error::Invalid("Thread::execute_ibinop32 Some(StackEntry::Value(Val::I32Const(c1))) = self.stack.pop()".to_owned()))
            }
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_ibinop32 Some(StackEntry::Value(Val::I32Const(c2))) = self.stack.pop()".to_owned()))
        }
    }

    pub fn execute_iadd64(&mut self) -> ExecResult { self.execute_ibinop64(iadd64) }
    pub fn execute_isub64(&mut self) -> ExecResult { self.execute_ibinop64(isub64) }
    pub fn execute_imul64(&mut self) -> ExecResult { self.execute_ibinop64(imul64) }
    pub fn execute_idiv_u64(&mut self) -> ExecResult { self.execute_ibinop64(idiv_u64) }
    pub fn execute_idiv_s64(&mut self) -> ExecResult { self.execute_ibinop64(idiv_s64) }
    pub fn execute_irem_u64(&mut self) -> ExecResult { self.execute_ibinop64(irem_u64) }
    pub fn execute_irem_s64(&mut self) -> ExecResult { self.execute_ibinop64(irem_s64) }
    pub fn execute_iand64(&mut self) -> ExecResult { self.execute_ibinop64(iand64) }
    pub fn execute_ior64(&mut self) -> ExecResult { self.execute_ibinop64(ior64) }
    pub fn execute_ixor64(&mut self) -> ExecResult { self.execute_ibinop64(ixor64) }
    pub fn execute_ishl64(&mut self) -> ExecResult { self.execute_ibinop64(ishl64) }
    pub fn execute_ishr_u64(&mut self) -> ExecResult { self.execute_ibinop64(ishr_u64) }
    pub fn execute_ishr_s64(&mut self) -> ExecResult { self.execute_ibinop64(ishr_s64) }
    pub fn execute_irotl64(&mut self) -> ExecResult { self.execute_ibinop64(irotl64) }
    pub fn execute_irotr64(&mut self) -> ExecResult { self.execute_ibinop64(irotr64) }

    fn execute_ibinop64(&mut self, func: fn(u64, u64) -> u64) -> ExecResult {
        // assert!(self.stack.len() >= 2);
        if let Some(StackEntry::Value(Val::I64Const(c2))) = self.stack.pop() {
            if let Some(StackEntry::Value(Val::I64Const(c1))) = self.stack.pop() {
                ExecResult::Vals(vec![Val::I64Const(func(c1, c2))])
            } else {
                ExecResult::Trap(Error::Invalid("Thread::execute_ibinop64 Some(StackEntry::Value(Val::I64Const(c1))) = self.stack.pop()".to_owned()))
            }
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_ibinop64 Some(StackEntry::Value(Val::I64Const(c2))) = self.stack.pop()".to_owned()))
        }
    }


    /* floating-point number */

    pub fn execute_fadd32(&mut self) -> ExecResult { self.execute_fbinop32(fadd32) }
    pub fn execute_fsub32(&mut self) -> ExecResult { self.execute_fbinop32(fsub32) }
    pub fn execute_fmul32(&mut self) -> ExecResult { self.execute_fbinop32(fmul32) }
    pub fn execute_fdiv32(&mut self) -> ExecResult { self.execute_fbinop32(fdiv32) }
    pub fn execute_fmin32(&mut self) -> ExecResult { self.execute_fbinop32(fmin32) }
    pub fn execute_fmax32(&mut self) -> ExecResult { self.execute_fbinop32(fmax32) }
    pub fn execute_fcopysign32(&mut self) -> ExecResult { self.execute_fbinop32(fcopysign32) }

    fn execute_fbinop32(&mut self, func: fn(f32, f32) -> f32) -> ExecResult {
        // assert!(self.stack.len() >= 2);
        if let Some(StackEntry::Value(Val::F32Const(c2))) = self.stack.pop() {
            if let Some(StackEntry::Value(Val::F32Const(c1))) = self.stack.pop() {
                ExecResult::Vals(vec![Val::F32Const(func(c1, c2))])
            } else {
                ExecResult::Trap(Error::Invalid("Thread::execute_fbinop32 Some(StackEntry::Value(Val::F32Const(c1))) = self.stack.pop()".to_owned()))
            }
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_fbinop32 Some(StackEntry::Value(Val::F32Const(c2))) = self.stack.pop()".to_owned()))
        }
    }

    pub fn execute_fadd64(&mut self) -> ExecResult { self.execute_fbinop64(fadd64) }
    pub fn execute_fsub64(&mut self) -> ExecResult { self.execute_fbinop64(fsub64) }
    pub fn execute_fmul64(&mut self) -> ExecResult { self.execute_fbinop64(fmul64) }
    pub fn execute_fdiv64(&mut self) -> ExecResult { self.execute_fbinop64(fdiv64) }
    pub fn execute_fmin64(&mut self) -> ExecResult { self.execute_fbinop64(fmin64) }
    pub fn execute_fmax64(&mut self) -> ExecResult { self.execute_fbinop64(fmax64) }
    pub fn execute_fcopysign64(&mut self) -> ExecResult { self.execute_fbinop64(fcopysign64) }

    fn execute_fbinop64(&mut self, func: fn(f64, f64) -> f64) -> ExecResult {
        // assert!(self.stack.len() >= 2);
        if let Some(StackEntry::Value(Val::F64Const(c2))) = self.stack.pop() {
            if let Some(StackEntry::Value(Val::F64Const(c1))) = self.stack.pop() {
                ExecResult::Vals(vec![Val::F64Const(func(c1, c2))])
            } else {
                ExecResult::Trap(Error::Invalid("Thread::execute_fbinop64 Some(StackEntry::Value(Val::F64Const(c1))) = self.stack.pop()".to_owned()))
            }
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_fbinop64 Some(StackEntry::Value(Val::F64Const(c2))) = self.stack.pop()".to_owned()))
        }
    }
}