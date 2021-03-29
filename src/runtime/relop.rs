use super::*;

impl<'a> Thread<'a> {
    /* integer */

    pub fn execute_ieq32(&mut self) -> Result { self.execute_irelop32(ieq32) }
    pub fn execute_ine32(&mut self) -> Result { self.execute_irelop32(ine32) }
    pub fn execute_ilt_u32(&mut self) -> Result { self.execute_irelop32(ilt_u32) }
    pub fn execute_ilt_s32(&mut self) -> Result { self.execute_irelop32(ilt_s32) }
    pub fn execute_igt_u32(&mut self) -> Result { self.execute_irelop32(igt_u32) }
    pub fn execute_igt_s32(&mut self) -> Result { self.execute_irelop32(igt_s32) }
    pub fn execute_ile_u32(&mut self) -> Result { self.execute_irelop32(ile_u32) }
    pub fn execute_ile_s32(&mut self) -> Result { self.execute_irelop32(ile_s32) }
    pub fn execute_ige_u32(&mut self) -> Result { self.execute_irelop32(ige_u32) }
    pub fn execute_ige_s32(&mut self) -> Result { self.execute_irelop32(ige_s32) }

    fn execute_irelop32(&mut self, func: fn(u32, u32) -> u32) -> Result {
        // assert!(self.stack.len() >= 2);
        if let Some(StackEntry::Value(Val::I32Const(c2))) = self.stack.pop() {
            if let Some(StackEntry::Value(Val::I32Const(c1))) = self.stack.pop() {
                Result::Vals(vec![Val::I32Const(func(c1, c2))])
            } else {
                Result::Trap
            }
        } else {
            Result::Trap
        }
    }


    pub fn execute_ieq64(&mut self) -> Result { self.execute_irelop64(ieq64) }
    pub fn execute_ine64(&mut self) -> Result { self.execute_irelop64(ine64) }
    pub fn execute_ilt_u64(&mut self) -> Result { self.execute_irelop64(ilt_u64) }
    pub fn execute_ilt_s64(&mut self) -> Result { self.execute_irelop64(ilt_s64) }
    pub fn execute_igt_u64(&mut self) -> Result { self.execute_irelop64(igt_u64) }
    pub fn execute_igt_s64(&mut self) -> Result { self.execute_irelop64(igt_s64) }
    pub fn execute_ile_u64(&mut self) -> Result { self.execute_irelop64(ile_u64) }
    pub fn execute_ile_s64(&mut self) -> Result { self.execute_irelop64(ile_s64) }
    pub fn execute_ige_u64(&mut self) -> Result { self.execute_irelop64(ige_u64) }
    pub fn execute_ige_s64(&mut self) -> Result { self.execute_irelop64(ige_s64) }

    fn execute_irelop64(&mut self, func: fn(u64, u64) -> u64) -> Result {
        // assert!(self.stack.len() >= 2);
        if let Some(StackEntry::Value(Val::I64Const(c2))) = self.stack.pop() {
            if let Some(StackEntry::Value(Val::I64Const(c1))) = self.stack.pop() {
                Result::Vals(vec![Val::I64Const(func(c1, c2))])
            } else {
                Result::Trap
            }
        } else {
            Result::Trap
        }
    }

    /* floating-point number */

    pub fn execute_feq32(&mut self) -> Result { self.execute_frelop32(feq32) }
    pub fn execute_fne32(&mut self) -> Result { self.execute_frelop32(fne32) }
    pub fn execute_flt32(&mut self) -> Result { self.execute_frelop32(flt32) }
    pub fn execute_fgt32(&mut self) -> Result { self.execute_frelop32(fgt32) }
    pub fn execute_fle32(&mut self) -> Result { self.execute_frelop32(fle32) }
    pub fn execute_fge32(&mut self) -> Result { self.execute_frelop32(fge32) }

    fn execute_frelop32(&mut self, func: fn(f32, f32) -> f32) -> Result {
        // assert!(self.stack.len() >= 2);
        if let Some(StackEntry::Value(Val::F32Const(c2))) = self.stack.pop() {
            if let Some(StackEntry::Value(Val::F32Const(c1))) = self.stack.pop() {
                Result::Vals(vec![Val::F32Const(func(c1, c2))])
            } else {
                Result::Trap
            }
        } else {
            Result::Trap
        }
    }

    pub fn execute_feq64(&mut self) -> Result { self.execute_frelop64(feq64) }
    pub fn execute_fne64(&mut self) -> Result { self.execute_frelop64(fne64) }
    pub fn execute_flt64(&mut self) -> Result { self.execute_frelop64(flt64) }
    pub fn execute_fgt64(&mut self) -> Result { self.execute_frelop64(fgt64) }
    pub fn execute_fle64(&mut self) -> Result { self.execute_frelop64(fle64) }
    pub fn execute_fge64(&mut self) -> Result { self.execute_frelop64(fge64) }

    fn execute_frelop64(&mut self, func: fn(f64, f64) -> f64) -> Result {
        // assert!(self.stack.len() >= 2);
        if let Some(StackEntry::Value(Val::F64Const(c2))) = self.stack.pop() {
            if let Some(StackEntry::Value(Val::F64Const(c1))) = self.stack.pop() {
                Result::Vals(vec![Val::F64Const(func(c1, c2))])
            } else {
                Result::Trap
            }
        } else {
            Result::Trap
        }
    }
}