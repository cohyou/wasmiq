use super::*;


/* integer */

pub fn execute_ieqz32(vals: &mut Vec<Val>) -> Result { execute_itestop32(vals, ieqz32) }

fn execute_itestop32(vals: &mut Vec<Val>, func: fn(u32) -> u32) -> Result {
    assert!(vals.len() >= 1);
    if let Some(Val::I32Const(c)) = vals.pop() {
        Result::Vals(vec![Val::I32Const(func(c))])
    } else {
        Result::Trap
    }
}


/* floating-point number */

pub fn execute_ieqz64(vals: &mut Vec<Val>) -> Result { execute_itestop64(vals, ieqz64) }

fn execute_itestop64(vals: &mut Vec<Val>, func: fn(u64) -> u64) -> Result {
    assert!(vals.len() >= 1);
    if let Some(Val::I64Const(c)) = vals.pop() {
        Result::Vals(vec![Val::I64Const(func(c))])
    } else {
        Result::Trap
    }
}