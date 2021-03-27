use super::*;


/* integer */

pub fn execute_iclz32(vals: &mut Vec<Val>) -> Result { execute_iunop32(vals, iclz32) }
pub fn execute_ictz32(vals: &mut Vec<Val>) -> Result { execute_iunop32(vals, ictz32) }
pub fn execute_ipopcnt32(vals: &mut Vec<Val>) -> Result { execute_iunop32(vals, ipopcnt32) }

fn execute_iunop32(vals: &mut Vec<Val>, func: fn(u32) -> u32) -> Result {
    assert!(vals.len() >= 1);
    if let Some(Val::I32Const(i)) = vals.pop() {
        Result::Vals(vec![Val::I32Const(func(i))])
    } else {
        Result::Trap
    }
}

pub fn execute_iclz64(vals: &mut Vec<Val>) -> Result { execute_iunop64(vals, iclz64) }
pub fn execute_ictz64(vals: &mut Vec<Val>) -> Result { execute_iunop64(vals, ictz64) }
pub fn execute_ipopcnt64(vals: &mut Vec<Val>) -> Result { execute_iunop64(vals, ipopcnt64) }

fn execute_iunop64(vals: &mut Vec<Val>, func: fn(u64) -> u64) -> Result {
    assert!(vals.len() >= 1);
    if let Some(Val::I64Const(i)) = vals.pop() {
        Result::Vals(vec![Val::I64Const(func(i))])
    } else {
        Result::Trap
    }
}


/* floating-point number */

pub fn execute_fabs32(vals: &mut Vec<Val>) -> Result { execute_funop32(vals, fabs32) }
pub fn execute_fneg32(vals: &mut Vec<Val>) -> Result { execute_funop32(vals, fneg32) }
pub fn execute_fsqrt32(vals: &mut Vec<Val>) -> Result { execute_funop32(vals, fsqrt32) }
pub fn execute_fceil32(vals: &mut Vec<Val>) -> Result { execute_funop32(vals, fceil32) }
pub fn execute_ffloor32(vals: &mut Vec<Val>) -> Result { execute_funop32(vals, ffloor32) }
pub fn execute_ftrunc32(vals: &mut Vec<Val>) -> Result { execute_funop32(vals, ftrunc32) }
pub fn execute_fnearest32(vals: &mut Vec<Val>) -> Result { execute_funop32(vals, fnearest32) }

pub fn execute_funop32(vals: &mut Vec<Val>, func:fn(f32) -> f32) -> Result {
    assert!(vals.len() >= 1);
    if let Some(Val::F32Const(f)) = vals.pop() {
        Result::Vals(vec![Val::F32Const(func(f))])
    } else {
        Result::Trap
    }
}

pub fn execute_fabs64(vals: &mut Vec<Val>) -> Result { execute_funop64(vals, fabs64) }
pub fn execute_fneg64(vals: &mut Vec<Val>) -> Result { execute_funop64(vals, fneg64) }
pub fn execute_fsqrt64(vals: &mut Vec<Val>) -> Result { execute_funop64(vals, fsqrt64) }
pub fn execute_fceil64(vals: &mut Vec<Val>) -> Result { execute_funop64(vals, fceil64) }
pub fn execute_ffloor64(vals: &mut Vec<Val>) -> Result { execute_funop64(vals, ffloor64) }
pub fn execute_ftrunc64(vals: &mut Vec<Val>) -> Result { execute_funop64(vals, ftrunc64) }
pub fn execute_fnearest64(vals: &mut Vec<Val>) -> Result { execute_funop64(vals, fnearest64) }

pub fn execute_funop64(vals: &mut Vec<Val>, func:fn(f64) -> f64) -> Result {
    assert!(vals.len() >= 1);
    if let Some(Val::F64Const(f)) = vals.pop() {
        Result::Vals(vec![Val::F64Const(func(f))])
    } else {
        Result::Trap
    }
}