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
}

// macro_rules! trunc_op {
//     ($this:ident, $vt:pat) => {
//         if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
//             if v.is_nan() || v.is_infinite() { return Result::Trap; }
//             if v < u32::MIN as f32 || v > u32::MAX as f32 { return Result::Trap; }
//             let vi = unsafe { v.to_int_unchecked::<u32>() };
//             Result::Vals(vec![Val::I32Const(vi)])
//         } else {
//             Result::Trap
//         }
//     };
// }

impl<'a> Thread<'a> {
    pub fn execute_i32trunc_f32_u(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return Result::Trap; }
            if v < u32::MIN as f32 || v > u32::MAX as f32 { return Result::Trap; }
            let vi = unsafe { v.to_int_unchecked::<u32>() };
            Result::Vals(vec![Val::I32Const(vi)])
        } else {
            Result::Trap
        }
    }

    pub fn execute_i32trunc_f32_s(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return Result::Trap; }
            if v < i32::MIN as f32 || v > i32::MAX as f32 { return Result::Trap; }
            let vi = unsafe { v.to_int_unchecked::<i32>() };
            Result::Vals(vec![Val::I32Const(unsigned32(vi))])
        } else {
            Result::Trap
        }
    }

    pub fn execute_i32trunc_f64_u(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return Result::Trap; }
            if v < u32::MIN as f64 || v > u32::MAX as f64 { return Result::Trap; }
            let vi = unsafe { v.to_int_unchecked::<u32>() };
            Result::Vals(vec![Val::I32Const(vi)])
        } else {
            Result::Trap
        }
    }

    pub fn execute_i32trunc_f64_s(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return Result::Trap; }
            if v < i32::MIN as f64 || v > i32::MAX as f64 { return Result::Trap; }
            let vi = unsafe { v.to_int_unchecked::<i32>() };
            Result::Vals(vec![Val::I32Const(unsigned32(vi))])
        } else {
            Result::Trap
        }
    }

    pub fn execute_i64trunc_f32_u(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return Result::Trap; }
            if v < u32::MIN as f32 || v > u32::MAX as f32 { return Result::Trap; }
            let vi = unsafe { v.to_int_unchecked::<u64>() };
            Result::Vals(vec![Val::I64Const(vi)])
        } else {
            Result::Trap
        }
    }

    pub fn execute_i64trunc_f32_s(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return Result::Trap; }
            if v < i32::MIN as f32 || v > i32::MAX as f32 { return Result::Trap; }
            let vi = unsafe { v.to_int_unchecked::<i64>() };
            Result::Vals(vec![Val::I64Const(unsigned64(vi))])
        } else {
            Result::Trap
        }
    }

    pub fn execute_i64trunc_f64_u(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return Result::Trap; }
            if v < u32::MIN as f64 || v > u32::MAX as f64 { return Result::Trap; }
            let vi = unsafe { v.to_int_unchecked::<u64>() };
            Result::Vals(vec![Val::I64Const(vi)])
        } else {
            Result::Trap
        }
    }

    pub fn execute_i64trunc_f64_s(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return Result::Trap; }
            if v < i32::MIN as f64 || v > i32::MAX as f64 { return Result::Trap; }
            let vi = unsafe { v.to_int_unchecked::<i64>() };
            Result::Vals(vec![Val::I64Const(unsigned64(vi))])
        } else {
            Result::Trap
        }
    }
}

macro_rules! convert_op {
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
        convert_op!(self, v, Val::F32Const(v), Val::I32Const(v as u32))
    }
    pub fn execute_i32trunc_sat_f32_s(&mut self) -> Result {
        convert_op!(self, v, Val::F32Const(v), Val::I32Const(unsigned32(v as i32)))
    }
    pub fn execute_i32trunc_sat_f64_u(&mut self) -> Result {
        convert_op!(self, v, Val::F64Const(v), Val::I32Const(v as u32))
    }
    pub fn execute_i32trunc_sat_f64_s(&mut self) -> Result {
        convert_op!(self, v, Val::F64Const(v), Val::I32Const(unsigned32(v as i32)))
    }
    pub fn execute_i64trunc_sat_f32_u(&mut self) -> Result {
        convert_op!(self, v, Val::F32Const(v), Val::I64Const(v as u64))
    }
    pub fn execute_i64trunc_sat_f32_s(&mut self) -> Result {
        convert_op!(self, v, Val::F32Const(v), Val::I64Const(unsigned64(v as i64)))
    }
    pub fn execute_i64trunc_sat_f64_u(&mut self) -> Result {
        convert_op!(self, v, Val::F64Const(v), Val::I64Const(v as u64))
    }
    pub fn execute_i64trunc_sat_f64_s(&mut self) -> Result {
        convert_op!(self, v, Val::F64Const(v), Val::I64Const(unsigned64(v as i64)))
    }
}

impl<'a> Thread<'a> {
    pub fn execute_demote(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F64Const(_v))) = self.stack.pop() {
            unimplemented!()
        } else {
            Result::Trap
        }
    }
    pub fn execute_promote(&mut self) -> Result {
        if let Some(StackEntry::Value(Val::F32Const(_v))) = self.stack.pop() {
            unimplemented!()
        } else {
            Result::Trap
        }
    }
}

impl<'a> Thread<'a> {
    pub fn execute_f32convert_i32_u(&mut self) -> Result {
        convert_op!(self, v, Val::I32Const(v), Val::F32Const(v as f32))
    }
    pub fn execute_f32convert_i32_s(&mut self) -> Result {
        convert_op!(self, v, Val::I32Const(v), Val::F32Const(signed32(v) as f32))
    }
    pub fn execute_f32convert_i64_u(&mut self) -> Result {
        convert_op!(self, v, Val::I64Const(v), Val::F32Const(v as f32))
    }
    pub fn execute_f32convert_i64_s(&mut self) -> Result {
        convert_op!(self, v, Val::I64Const(v), Val::F32Const(signed64(v) as f32))
    }
    pub fn execute_f64convert_i32_u(&mut self) -> Result {
        convert_op!(self, v, Val::I32Const(v), Val::F64Const(v as f64))
    }
    pub fn execute_f64convert_i32_s(&mut self) -> Result {
        convert_op!(self, v, Val::I32Const(v), Val::F64Const(signed32(v) as f64))
    }
    pub fn execute_f64convert_i64_u(&mut self) -> Result {
        convert_op!(self, v, Val::I64Const(v), Val::F64Const(v as f64))
    }
    pub fn execute_f64convert_i64_s(&mut self) -> Result {
        convert_op!(self, v, Val::I64Const(v), Val::F64Const(signed64(v) as f64))
    }
}

impl<'a> Thread<'a> {
    pub fn execute_i32reinterpret_f32(&mut self) -> Result {
        fn reinterpret(n: f32) -> u32 { u32::from_le_bytes(n.to_le_bytes()) }
        convert_op!(self, v, Val::F32Const(v), Val::I32Const(reinterpret(v)))
    }
    pub fn execute_i64reinterpret_f64(&mut self) -> Result {
        fn reinterpret(n: f64) -> u64 { u64::from_le_bytes(n.to_le_bytes()) }
        convert_op!(self, v, Val::F64Const(v), Val::I64Const(reinterpret(v)))
    }
    pub fn execute_f32reinterpret_i32(&mut self) -> Result {
        fn reinterpret(n: u32) -> f32 { f32::from_le_bytes(n.to_le_bytes()) }
        convert_op!(self, v, Val::I32Const(v), Val::F32Const(reinterpret(v)))
    }
    pub fn execute_f64reinterpret_f64(&mut self) -> Result {
        fn reinterpret(n: u64) -> f64 { f64::from_le_bytes(n.to_le_bytes()) }
        convert_op!(self, v, Val::I64Const(v), Val::F64Const(reinterpret(v)))
    }
}