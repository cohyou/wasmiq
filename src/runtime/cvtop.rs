use super::*;

macro_rules! val {
    (i, 32, $m:ident) => { Val::I32Const($m) };
    (i, 64, $m:ident) => { Val::I64Const($m) };
}

macro_rules! extendN_op {
    ($this:ident, $m:ident, $mp:pat, $mr:expr, $sz:ident) => {
        match $this.stack.pop() {
            Some(StackEntry::Value($mp)) => {
                if $m > $sz::MAX.into() { return ExecResult::Trap(Error::Invalid("extendN_op $m > $sz::MAX.into()".to_owned())); }
                ExecResult::Vals(vec![$mr])
            },
            _ => ExecResult::Trap(Error::Invalid("extendN_op $this.stack.pop() is None".to_owned())),
        }
    };
}

impl<'a> Thread<'a> {
    pub fn execute_i32extend8s(&mut self) -> ExecResult {
        extendN_op!(self, v, val!(i, 32, v), val!(i, 32, v), u8)
    }
    pub fn execute_i64extend8s(&mut self) -> ExecResult {
        extendN_op!(self, v, val!(i, 64, v), val!(i, 64, v), u8)
    }
    pub fn execute_i32extend16s(&mut self) -> ExecResult {
        extendN_op!(self, v, val!(i, 32, v), val!(i, 32, v), u16)
    }
    pub fn execute_i64extend16s(&mut self) -> ExecResult {
        extendN_op!(self, v, val!(i, 64, v), val!(i, 64, v), u16)
    }
    pub fn execute_i64extend32s(&mut self) -> ExecResult {
        extendN_op!(self, v, val!(i, 64, v), val!(i, 64, v), u32)
    }
}

impl<'a> Thread<'a> {
    pub fn execute_i32wrap_i64(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::I64Const(v))) = self.stack.pop() {
            let r = v % 2u64.pow(32);
            ExecResult::Vals(vec![Val::I32Const(r as u32)])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_i32wrap_i64 Some(StackEntry::Value(Val::I64Const(v))) = self.stack.pop()".to_owned()))
        }
    }
    pub fn execute_i64wrap_i32_u(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::I32Const(v))) = self.stack.pop() {
            ExecResult::Vals(vec![Val::I64Const(v as u64)])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_i64wrap_i32_u Some(StackEntry::Value(Val::I32Const(v))) = self.stack.pop()".to_owned()))
        }
    }
    pub fn execute_i64wrap_i32_s(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::I32Const(v))) = self.stack.pop() {
            if v > u32::MAX.into() { return ExecResult::Trap(Error::Invalid("Thread::execute_i64wrap_i32_s v > u32::MAX.into()".to_owned())); }
            ExecResult::Vals(vec![Val::I64Const(v as u64)])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_i64wrap_i32_s Some(StackEntry::Value(Val::I32Const(v))) = self.stack.pop()".to_owned()))
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
    pub fn execute_i32trunc_f32_u(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f32_u v.is_nan() || v.is_infinite()".to_owned())); }
            if v < u32::MIN as f32 || v > u32::MAX as f32 { return ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f32_u v < u32::MIN as f32 || v > u32::MAX as f32".to_owned())); }
            let vi = unsafe { v.to_int_unchecked::<u32>() };
            ExecResult::Vals(vec![Val::I32Const(vi)])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f32_u Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop()".to_owned()))
        }
    }

    pub fn execute_i32trunc_f32_s(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f32_s v.is_nan() || v.is_infinite() ".to_owned())); }
            if v < i32::MIN as f32 || v > i32::MAX as f32 { return ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f32_s v < i32::MIN as f32 || v > i32::MAX as f32".to_owned())); }
            let vi = unsafe { v.to_int_unchecked::<i32>() };
            ExecResult::Vals(vec![Val::I32Const(unsigned32(vi))])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f32_s Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop()".to_owned()))
        }
    }

    pub fn execute_i32trunc_f64_u(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f64_u v.is_nan() || v.is_infinite()".to_owned())); }
            if v < u32::MIN as f64 || v > u32::MAX as f64 { return ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f64_u v < u32::MIN as f64 || v > u32::MAX as f64".to_owned())); }
            let vi = unsafe { v.to_int_unchecked::<u32>() };
            ExecResult::Vals(vec![Val::I32Const(vi)])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f64_u Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop()".to_owned()))
        }
    }

    pub fn execute_i32trunc_f64_s(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f64_s v.is_nan() || v.is_infinite()".to_owned())); }
            if v < i32::MIN as f64 || v > i32::MAX as f64 { return ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f64_s v < i32::MIN as f64 || v > i32::MAX as f64 ".to_owned())); }
            let vi = unsafe { v.to_int_unchecked::<i32>() };
            ExecResult::Vals(vec![Val::I32Const(unsigned32(vi))])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_i32trunc_f64_s Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop()".to_owned()))
        }
    }

    pub fn execute_i64trunc_f32_u(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f32_u v.is_nan() || v.is_infinite()".to_owned())); }
            if v < u32::MIN as f32 || v > u32::MAX as f32 { return ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f32_u v < u32::MIN as f32 || v > u32::MAX as f32".to_owned())); }
            let vi = unsafe { v.to_int_unchecked::<u64>() };
            ExecResult::Vals(vec![Val::I64Const(vi)])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f32_u Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop()".to_owned()))
        }
    }

    pub fn execute_i64trunc_f32_s(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f32_s v.is_nan() || v.is_infinite()".to_owned())); }
            if v < i32::MIN as f32 || v > i32::MAX as f32 { return ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f32_s v < i32::MIN as f32 || v > i32::MAX as f32".to_owned())); }
            let vi = unsafe { v.to_int_unchecked::<i64>() };
            ExecResult::Vals(vec![Val::I64Const(unsigned64(vi))])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f32_s Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop()".to_owned()))
        }
    }

    pub fn execute_i64trunc_f64_u(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f64_u v.is_nan() || v.is_infinite() ".to_owned())); }
            if v < u32::MIN as f64 || v > u32::MAX as f64 { return ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f64_u v < u32::MIN as f64 || v > u32::MAX as f64".to_owned())); }
            let vi = unsafe { v.to_int_unchecked::<u64>() };
            ExecResult::Vals(vec![Val::I64Const(vi)])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f64_u Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop()".to_owned()))
        }
    }

    pub fn execute_i64trunc_f64_s(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop() {
            if v.is_nan() || v.is_infinite() { return ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f64_s v.is_nan() || v.is_infinite()".to_owned())); }
            if v < i32::MIN as f64 || v > i32::MAX as f64 { return ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f64_s v < i32::MIN as f64 || v > i32::MAX as f64".to_owned())); }
            let vi = unsafe { v.to_int_unchecked::<i64>() };
            ExecResult::Vals(vec![Val::I64Const(unsigned64(vi))])
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_i64trunc_f64_s Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop()".to_owned()))
        }
    }
}

macro_rules! convert_op {
    ($this:ident, $v:ident, $vp:pat, $vr:expr) => {
        if let Some(StackEntry::Value($vp)) = $this.stack.pop() {
            ExecResult::Vals(vec![$vr])
        } else {
            ExecResult::Trap(Error::Invalid("convert_op Some(StackEntry::Value($vp)) = $this.stack.pop()".to_owned()))
        }
    };
}

impl<'a> Thread<'a> {
    pub fn execute_i32trunc_sat_f32_u(&mut self) -> ExecResult {
        convert_op!(self, v, Val::F32Const(v), Val::I32Const(v as u32))
    }
    pub fn execute_i32trunc_sat_f32_s(&mut self) -> ExecResult {
        convert_op!(self, v, Val::F32Const(v), Val::I32Const(unsigned32(v as i32)))
    }
    pub fn execute_i32trunc_sat_f64_u(&mut self) -> ExecResult {
        convert_op!(self, v, Val::F64Const(v), Val::I32Const(v as u32))
    }
    pub fn execute_i32trunc_sat_f64_s(&mut self) -> ExecResult {
        convert_op!(self, v, Val::F64Const(v), Val::I32Const(unsigned32(v as i32)))
    }
    pub fn execute_i64trunc_sat_f32_u(&mut self) -> ExecResult {
        convert_op!(self, v, Val::F32Const(v), Val::I64Const(v as u64))
    }
    pub fn execute_i64trunc_sat_f32_s(&mut self) -> ExecResult {
        convert_op!(self, v, Val::F32Const(v), Val::I64Const(unsigned64(v as i64)))
    }
    pub fn execute_i64trunc_sat_f64_u(&mut self) -> ExecResult {
        convert_op!(self, v, Val::F64Const(v), Val::I64Const(v as u64))
    }
    pub fn execute_i64trunc_sat_f64_s(&mut self) -> ExecResult {
        convert_op!(self, v, Val::F64Const(v), Val::I64Const(unsigned64(v as i64)))
    }
}

impl<'a> Thread<'a> {
    pub fn execute_demote(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop() {
            if v.is_nan() {
                if v == f64::NAN {
                    return ExecResult::f32val(f32::NAN);
                } else {
                    unimplemented!()
                }
            }
            if v.is_infinite() || v == 0.0 { return ExecResult::f32val(v as f32); }
            if v < f32::MIN as f64 || v > f32::MAX as f64 { return ExecResult::Trap(Error::Invalid("".to_owned())); }
            ExecResult::f32val(v as f32)
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_demote Some(StackEntry::Value(Val::F64Const(v))) = self.stack.pop()".to_owned()))
        }
    }
    
    pub fn execute_promote(&mut self) -> ExecResult {
        if let Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop() {
            if v.is_nan() {
                if v == f32::NAN {
                    return ExecResult::f64val(f64::NAN);
                } else {
                    unimplemented!()
                }
            }
            ExecResult::f64val(v as f64)
        } else {
            ExecResult::Trap(Error::Invalid("Thread::execute_promote Some(StackEntry::Value(Val::F32Const(v))) = self.stack.pop()".to_owned()))
        }
    }
}

impl<'a> Thread<'a> {
    pub fn execute_f32convert_i32_u(&mut self) -> ExecResult {
        convert_op!(self, v, Val::I32Const(v), Val::F32Const(v as f32))
    }
    pub fn execute_f32convert_i32_s(&mut self) -> ExecResult {
        convert_op!(self, v, Val::I32Const(v), Val::F32Const(signed32(v) as f32))
    }
    pub fn execute_f32convert_i64_u(&mut self) -> ExecResult {
        convert_op!(self, v, Val::I64Const(v), Val::F32Const(v as f32))
    }
    pub fn execute_f32convert_i64_s(&mut self) -> ExecResult {
        convert_op!(self, v, Val::I64Const(v), Val::F32Const(signed64(v) as f32))
    }
    pub fn execute_f64convert_i32_u(&mut self) -> ExecResult {
        convert_op!(self, v, Val::I32Const(v), Val::F64Const(v as f64))
    }
    pub fn execute_f64convert_i32_s(&mut self) -> ExecResult {
        convert_op!(self, v, Val::I32Const(v), Val::F64Const(signed32(v) as f64))
    }
    pub fn execute_f64convert_i64_u(&mut self) -> ExecResult {
        convert_op!(self, v, Val::I64Const(v), Val::F64Const(v as f64))
    }
    pub fn execute_f64convert_i64_s(&mut self) -> ExecResult {
        convert_op!(self, v, Val::I64Const(v), Val::F64Const(signed64(v) as f64))
    }
}

impl<'a> Thread<'a> {
    pub fn execute_i32reinterpret_f32(&mut self) -> ExecResult {
        fn reinterpret(n: f32) -> u32 { u32::from_le_bytes(n.to_le_bytes()) }
        convert_op!(self, v, Val::F32Const(v), Val::I32Const(reinterpret(v)))
    }
    pub fn execute_i64reinterpret_f64(&mut self) -> ExecResult {
        fn reinterpret(n: f64) -> u64 { u64::from_le_bytes(n.to_le_bytes()) }
        convert_op!(self, v, Val::F64Const(v), Val::I64Const(reinterpret(v)))
    }
    pub fn execute_f32reinterpret_i32(&mut self) -> ExecResult {
        fn reinterpret(n: u32) -> f32 { f32::from_le_bytes(n.to_le_bytes()) }
        convert_op!(self, v, Val::I32Const(v), Val::F32Const(reinterpret(v)))
    }
    pub fn execute_f64reinterpret_f64(&mut self) -> ExecResult {
        fn reinterpret(n: u64) -> f64 { f64::from_le_bytes(n.to_le_bytes()) }
        convert_op!(self, v, Val::I64Const(v), Val::F64Const(reinterpret(v)))
    }
}