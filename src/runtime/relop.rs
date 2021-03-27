use super::*;


/* integer */

pub fn execute_ieq32(vals: &mut Vec<Val>) -> Result { execute_irelop32(vals, ieq32) }
pub fn execute_ine32(vals: &mut Vec<Val>) -> Result { execute_irelop32(vals, ine32) }
pub fn execute_ilt_u32(vals: &mut Vec<Val>) -> Result { execute_irelop32(vals, ilt_u32) }
pub fn execute_ilt_s32(vals: &mut Vec<Val>) -> Result { execute_irelop32(vals, ilt_s32) }
pub fn execute_igt_u32(vals: &mut Vec<Val>) -> Result { execute_irelop32(vals, igt_u32) }
pub fn execute_igt_s32(vals: &mut Vec<Val>) -> Result { execute_irelop32(vals, igt_s32) }
pub fn execute_ile_u32(vals: &mut Vec<Val>) -> Result { execute_irelop32(vals, ile_u32) }
pub fn execute_ile_s32(vals: &mut Vec<Val>) -> Result { execute_irelop32(vals, ile_s32) }
pub fn execute_ige_u32(vals: &mut Vec<Val>) -> Result { execute_irelop32(vals, ige_u32) }
pub fn execute_ige_s32(vals: &mut Vec<Val>) -> Result { execute_irelop32(vals, ige_s32) }

fn execute_irelop32(vals: &mut Vec<Val>, func: fn(u32, u32) -> u32) -> Result {
    assert!(vals.len() >= 2);
    if let Some(Val::I32Const(c2)) = vals.pop() {
        if let Some(Val::I32Const(c1)) = vals.pop() {
            Result::Vals(vec![Val::I32Const(func(c1, c2))])
        } else {
            Result::Trap
        }
    } else {
        Result::Trap
    }
}


pub fn execute_ieq64(vals: &mut Vec<Val>) -> Result { execute_irelop64(vals, ieq64) }
pub fn execute_ine64(vals: &mut Vec<Val>) -> Result { execute_irelop64(vals, ine64) }
pub fn execute_ilt_u64(vals: &mut Vec<Val>) -> Result { execute_irelop64(vals, ilt_u64) }
pub fn execute_ilt_s64(vals: &mut Vec<Val>) -> Result { execute_irelop64(vals, ilt_s64) }
pub fn execute_igt_u64(vals: &mut Vec<Val>) -> Result { execute_irelop64(vals, igt_u64) }
pub fn execute_igt_s64(vals: &mut Vec<Val>) -> Result { execute_irelop64(vals, igt_s64) }
pub fn execute_ile_u64(vals: &mut Vec<Val>) -> Result { execute_irelop64(vals, ile_u64) }
pub fn execute_ile_s64(vals: &mut Vec<Val>) -> Result { execute_irelop64(vals, ile_s64) }
pub fn execute_ige_u64(vals: &mut Vec<Val>) -> Result { execute_irelop64(vals, ige_u64) }
pub fn execute_ige_s64(vals: &mut Vec<Val>) -> Result { execute_irelop64(vals, ige_s64) }

fn execute_irelop64(vals: &mut Vec<Val>, func: fn(u64, u64) -> u64) -> Result {
    assert!(vals.len() >= 2);
    if let Some(Val::I64Const(c2)) = vals.pop() {
        if let Some(Val::I64Const(c1)) = vals.pop() {
            Result::Vals(vec![Val::I64Const(func(c1, c2))])
        } else {
            Result::Trap
        }
    } else {
        Result::Trap
    }
}

/* floating-point number */

pub fn execute_feq32(vals: &mut Vec<Val>) -> Result { execute_frelop32(vals, feq32) }
pub fn execute_fne32(vals: &mut Vec<Val>) -> Result { execute_frelop32(vals, fne32) }
pub fn execute_flt32(vals: &mut Vec<Val>) -> Result { execute_frelop32(vals, flt32) }
pub fn execute_fgt32(vals: &mut Vec<Val>) -> Result { execute_frelop32(vals, fgt32) }
pub fn execute_fle32(vals: &mut Vec<Val>) -> Result { execute_frelop32(vals, fle32) }
pub fn execute_fge32(vals: &mut Vec<Val>) -> Result { execute_frelop32(vals, fge32) }

fn execute_frelop32(vals: &mut Vec<Val>, func: fn(f32, f32) -> f32) -> Result {
    assert!(vals.len() >= 2);
    if let Some(Val::F32Const(c2)) = vals.pop() {
        if let Some(Val::F32Const(c1)) = vals.pop() {
            Result::Vals(vec![Val::F32Const(func(c1, c2))])
        } else {
            Result::Trap
        }
    } else {
        Result::Trap
    }
}

pub fn execute_feq64(vals: &mut Vec<Val>) -> Result { execute_frelop64(vals, feq64) }
pub fn execute_fne64(vals: &mut Vec<Val>) -> Result { execute_frelop64(vals, fne64) }
pub fn execute_flt64(vals: &mut Vec<Val>) -> Result { execute_frelop64(vals, flt64) }
pub fn execute_fgt64(vals: &mut Vec<Val>) -> Result { execute_frelop64(vals, fgt64) }
pub fn execute_fle64(vals: &mut Vec<Val>) -> Result { execute_frelop64(vals, fle64) }
pub fn execute_fge64(vals: &mut Vec<Val>) -> Result { execute_frelop64(vals, fge64) }

fn execute_frelop64(vals: &mut Vec<Val>, func: fn(f64, f64) -> f64) -> Result {
    assert!(vals.len() >= 2);
    if let Some(Val::F64Const(c2)) = vals.pop() {
        if let Some(Val::F64Const(c1)) = vals.pop() {
            Result::Vals(vec![Val::F64Const(func(c1, c2))])
        } else {
            Result::Trap
        }
    } else {
        Result::Trap
    }
}