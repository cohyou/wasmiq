use crate::{
    ValSize,
    ValSign,
};

use super::*;

macro_rules! val {
    (i, 32, $m:ident) => { Val::I32Const($m) };
    (i, 64, $m:ident) => { Val::I64Const($m) };
}

macro_rules! extendN_op {
    ($this:ident, $m:ident, $mp:pat, $mr:expr, $sz:ident) => {
        match $this.stack.pop() {
            Some(StackEntry::Value($mp)) => {
                if $m > $sz::MAX.into() { return Result::Trap; }
                Result::Vals(vec![$mr])
            },
            _ => Result::Trap,
        }
    };
}

impl<'a> Thread<'a> {
    pub fn execute_i32extend8s(&mut self) -> Result {
        extendN_op!(self, v, val!(i, 32, v), val!(i, 32, v), u8)
    }
    pub fn execute_i64extend8s(&mut self) -> Result {
        extendN_op!(self, v, val!(i, 64, v), val!(i, 64, v), u8)
    }
    pub fn execute_i32extend16s(&mut self) -> Result {
        extendN_op!(self, v, val!(i, 32, v), val!(i, 32, v), u16)
    }
    pub fn execute_i64extend16s(&mut self) -> Result {
        extendN_op!(self, v, val!(i, 64, v), val!(i, 64, v), u16)
    }
    pub fn execute_i64extend32s(&mut self) -> Result {
        extendN_op!(self, v, val!(i, 64, v), val!(i, 64, v), u32)
    }
}

impl<'a> Thread<'a> {
    pub fn execute_i32wrap_i64(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::I64Const(v))) = self.stack.pop() {
            let r = v % 2u64.pow(32);
            Result::Vals(vec![Val::I32Const(r as u32)])
        } else {
            Result::Trap
        }
    }
    pub fn execute_i64wrap_i32_u(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::I32Const(v))) = self.stack.pop() {
            Result::Vals(vec![Val::I64Const(v as u64)])
        } else {
            Result::Trap
        }
    }
    pub fn execute_i64wrap_i32_s(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::I32Const(v))) = self.stack.pop() {
            if v > u32::MAX.into() { return Result::Trap; }
            Result::Vals(vec![Val::I64Const(v as u64)])
        } else {
            Result::Trap
        }
    }
    pub fn execute_i32trunc_f32_u(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return Result::Trap; }
            let vf = v.trunc().round() as u32;
            // TODO: update algorithm
            Result::Vals(vec![Val::I32Const(vf)])
        } else {
            Result::Trap
        }
    }
    pub fn execute_i32trunc_f32_s(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return Result::Trap; }
            // let vf = v.trunc().round() as i32;
            // TODO: update algorithm
            // Result::Vals(vec![Val::I32Const(vf)])
            unimplemented!()
        } else {
            Result::Trap
        }
    }

    // fn execute_cvtop<T1, T2>(&mut self, func: fn(T1) -> T2) -> Result {

    // }
}

macro_rules! trunc_sat_op {
    ($this:ident, $v:ident, $vp:pat, $vr:expr) => {
        if let Some(StackEntry::Value($vp)) = $this.stack.pop() {
            Result::Vals(vec![$vr])
        } else {
            Result::Trap
        }
    };
}
impl<'a> Thread<'a> {
    pub fn execute_i32trunc_sat_f32_u(&mut self) -> Result {
        trunc_sat_op!(self, v, Val::F32Const(v), Val::I32Const(v as u32))
    }
    pub fn execute_i32trunc_sat_f64_u(&mut self) -> Result {
        trunc_sat_op!(self, v, Val::F64Const(v), Val::I32Const(v as u32))
    }
    pub fn execute_i64trunc_sat_f32_u(&mut self) -> Result {
        trunc_sat_op!(self, v, Val::F32Const(v), Val::I64Const(v as u64))
    }
    pub fn execute_i64trunc_sat_f64_u(&mut self) -> Result {
        trunc_sat_op!(self, v, Val::F64Const(v), Val::I64Const(v as u64))
    }
}

// TODO: check algorithm correct
impl<'a> Thread<'a> {
    pub fn execute_f32convert_i32_u(&mut self) -> Result {
        trunc_sat_op!(self, v, Val::I32Const(v), Val::F32Const(v as f32))
    }
    pub fn execute_f32convert_i64_u(&mut self) -> Result {
        trunc_sat_op!(self, v, Val::I64Const(v), Val::F32Const(v as f32))
    }
    pub fn execute_f64convert_i32_u(&mut self) -> Result {
        trunc_sat_op!(self, v, Val::I32Const(v), Val::F64Const(v as f64))
    }
    pub fn execute_f64convert_i64_u(&mut self) -> Result {
        trunc_sat_op!(self, v, Val::I64Const(v), Val::F64Const(v as f64))
    }
}