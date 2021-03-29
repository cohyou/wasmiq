use crate::{
    Thread,
};
use super::*;

impl<'a> Thread<'a> {
    /* integer */

    pub fn execute_iclz32(&mut self) -> Result { self.execute_iunop32(iclz32) }
    pub fn execute_ictz32(&mut self) -> Result { self.execute_iunop32(ictz32) }
    pub fn execute_ipopcnt32(&mut self) -> Result { self.execute_iunop32(ipopcnt32) }

    fn execute_iunop32(&mut self, func: fn(u32) -> u32) -> Result {
        // assert!(vals.len() >= 1);
        if let Some(StackEntry::Value(Val::I32Const(i))) = self.stack.pop() {
            Result::Vals(vec![Val::I32Const(func(i))])
        } else {
            Result::Trap
        }
    }

    pub fn execute_iclz64(&mut self) -> Result { self.execute_iunop64(iclz64) }
    pub fn execute_ictz64(&mut self) -> Result { self.execute_iunop64(ictz64) }
    pub fn execute_ipopcnt64(&mut self) -> Result { self.execute_iunop64(ipopcnt64) }

    fn execute_iunop64(&mut self, func: fn(u64) -> u64) -> Result {
        // assert!(vals.len() >= 1);
        if let Some(StackEntry::Value(Val::I64Const(i))) = self.stack.pop() {
            Result::Vals(vec![Val::I64Const(func(i))])
        } else {
            Result::Trap
        }
    }


    /* floating-point number */

    pub fn execute_fabs32(&mut self) -> Result { self.execute_funop32(fabs32) }
    pub fn execute_fneg32(&mut self) -> Result { self.execute_funop32(fneg32) }
    pub fn execute_fsqrt32(&mut self) -> Result { self.execute_funop32(fsqrt32) }
    pub fn execute_fceil32(&mut self) -> Result { self.execute_funop32(fceil32) }
    pub fn execute_ffloor32(&mut self) -> Result { self.execute_funop32(ffloor32) }
    pub fn execute_ftrunc32(&mut self) -> Result { self.execute_funop32(ftrunc32) }
    pub fn execute_fnearest32(&mut self) -> Result { self.execute_funop32(fnearest32) }

    pub fn execute_funop32(&mut self, func:fn(f32) -> f32) -> Result {
        // assert!(vals.len() >= 1);
        if let Some(StackEntry::Value(Val::F32Const(f))) = self.stack.pop() {
            Result::Vals(vec![Val::F32Const(func(f))])
        } else {
            Result::Trap
        }
    }

    pub fn execute_fabs64(&mut self) -> Result { self.execute_funop64(fabs64) }
    pub fn execute_fneg64(&mut self) -> Result { self.execute_funop64(fneg64) }
    pub fn execute_fsqrt64(&mut self) -> Result { self.execute_funop64(fsqrt64) }
    pub fn execute_fceil64(&mut self) -> Result { self.execute_funop64(fceil64) }
    pub fn execute_ffloor64(&mut self) -> Result { self.execute_funop64(ffloor64) }
    pub fn execute_ftrunc64(&mut self) -> Result { self.execute_funop64(ftrunc64) }
    pub fn execute_fnearest64(&mut self) -> Result { self.execute_funop64(fnearest64) }

    pub fn execute_funop64(&mut self, func:fn(f64) -> f64) -> Result {
        // assert!(vals.len() >= 1);
        if let Some(StackEntry::Value(Val::F64Const(f))) = self.stack.pop() {
            Result::Vals(vec![Val::F64Const(func(f))])
        } else {
            Result::Trap
        }
    }
}
