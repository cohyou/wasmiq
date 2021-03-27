use super::*;


/* integer */

pub fn execute_iadd32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, iadd32) }
pub fn execute_isub32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, isub32) }
pub fn execute_imul32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, imul32) }
pub fn execute_idiv_u32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, idiv_u32) }
pub fn execute_idiv_s32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, idiv_s32) }
pub fn execute_irem_u32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, irem_u32) }
pub fn execute_irem_s32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, irem_s32) }
pub fn execute_iand32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, iand32) }
pub fn execute_ior32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, ior32) }
pub fn execute_ixor32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, ixor32) }
pub fn execute_ishl32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, ishl32) }
pub fn execute_ishr_u32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, ishr_u32) }
pub fn execute_ishr_s32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, ishr_s32) }
pub fn execute_irotl32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, irotl32) }
pub fn execute_irotr32(vals: &mut Vec<Val>) -> Result { execute_ibinop32(vals, irotr32) }

fn execute_ibinop32(vals: &mut Vec<Val>, func: fn(u32, u32) -> u32) -> Result {
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

pub fn execute_iadd64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, iadd64) }
pub fn execute_isub64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, isub64) }
pub fn execute_imul64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, imul64) }
pub fn execute_idiv_u64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, idiv_u64) }
pub fn execute_idiv_s64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, idiv_s64) }
pub fn execute_irem_u64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, irem_u64) }
pub fn execute_irem_s64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, irem_s64) }
pub fn execute_iand64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, iand64) }
pub fn execute_ior64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, ior64) }
pub fn execute_ixor64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, ixor64) }
pub fn execute_ishl64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, ishl64) }
pub fn execute_ishr_u64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, ishr_u64) }
pub fn execute_ishr_s64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, ishr_s64) }
pub fn execute_irotl64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, irotl64) }
pub fn execute_irotr64(vals: &mut Vec<Val>) -> Result { execute_ibinop64(vals, irotr64) }

fn execute_ibinop64(vals: &mut Vec<Val>, func: fn(u64, u64) -> u64) -> Result {
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

pub fn execute_fadd32(vals: &mut Vec<Val>) -> Result { execute_fbinop32(vals, fadd32) }
pub fn execute_fsub32(vals: &mut Vec<Val>) -> Result { execute_fbinop32(vals, fsub32) }
pub fn execute_fmul32(vals: &mut Vec<Val>) -> Result { execute_fbinop32(vals, fmul32) }
pub fn execute_fdiv32(vals: &mut Vec<Val>) -> Result { execute_fbinop32(vals, fdiv32) }
pub fn execute_fmin32(vals: &mut Vec<Val>) -> Result { execute_fbinop32(vals, fmin32) }
pub fn execute_fmax32(vals: &mut Vec<Val>) -> Result { execute_fbinop32(vals, fmax32) }
pub fn execute_fcopysign32(vals: &mut Vec<Val>) -> Result { execute_fbinop32(vals, fcopysign32) }

fn execute_fbinop32(vals: &mut Vec<Val>, func: fn(f32, f32) -> f32) -> Result {
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

pub fn execute_fadd64(vals: &mut Vec<Val>) -> Result { execute_fbinop64(vals, fadd64) }
pub fn execute_fsub64(vals: &mut Vec<Val>) -> Result { execute_fbinop64(vals, fsub64) }
pub fn execute_fmul64(vals: &mut Vec<Val>) -> Result { execute_fbinop64(vals, fmul64) }
pub fn execute_fdiv64(vals: &mut Vec<Val>) -> Result { execute_fbinop64(vals, fdiv64) }
pub fn execute_fmin64(vals: &mut Vec<Val>) -> Result { execute_fbinop64(vals, fmin64) }
pub fn execute_fmax64(vals: &mut Vec<Val>) -> Result { execute_fbinop64(vals, fmax64) }
pub fn execute_fcopysign64(vals: &mut Vec<Val>) -> Result { execute_fbinop64(vals, fcopysign64) }

fn execute_fbinop64(vals: &mut Vec<Val>, func: fn(f64, f64) -> f64) -> Result {
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