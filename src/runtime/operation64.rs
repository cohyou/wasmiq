pub fn iadd64(c1: u64, c2: u64) -> u64 { c1.wrapping_add(c2) }
pub fn isub64(c1: u64, c2: u64) -> u64 { c1.wrapping_sub(c2) }
pub fn imul64(c1: u64, c2: u64) -> u64 { c1.wrapping_mul(c2) }
pub fn idiv_u64(c1: u64, c2: u64) -> u64 { c1.wrapping_div(c2) }
pub fn idiv_s64(c1: u64, c2: u64) -> u64 { c1.wrapping_div_euclid(c2) }
pub fn irem_u64(c1: u64, c2: u64) -> u64 { c1.wrapping_rem(c2) }
pub fn irem_s64(c1: u64, c2: u64) -> u64 { c1.wrapping_rem_euclid(c2) }
pub fn iand64(c1: u64, c2: u64) -> u64 { c1 & c2 }
pub fn ior64(c1: u64, c2: u64) -> u64 { c1 | c2 }
pub fn ixor64(_c1: u64, _c2: u64) -> u64 { unimplemented!() }
pub fn ishl64(c1: u64, c2: u64) -> u64 { c1.wrapping_shl(c2 as u32) }
pub fn ishr_u64(_c1: u64, _c2: u64) -> u64 { unimplemented!() }
pub fn ishr_s64(_c1: u64, _c2: u64) -> u64 { unimplemented!() }
pub fn irotl64(_c1: u64, _c2: u64) -> u64 { unimplemented!() }
pub fn irotr64(_c1: u64, _c2: u64) -> u64 { unimplemented!() }
pub fn iclz64(i: u64) -> u64 { i.leading_zeros().into() }
pub fn ictz64(i: u64) -> u64 { i.trailing_zeros().into() }
pub fn ipopcnt64(i: u64) -> u64 { i.count_ones().into() }
pub fn ieqz64(i: u64) -> u64 { if i == 0 { 1 } else { 0 } }
pub fn ieq64(c1: u64, c2: u64) -> u64 { if c1 == c2 { 1 } else { 0 } }
pub fn ine64(c1: u64, c2: u64) -> u64 { if c1 != c2 { 1 } else { 0 } }
pub fn ilt_u64(c1: u64, c2: u64) -> u64 { if c1 < c2 { 1 } else { 0 } }
pub fn ilt_s64(_c1: u64, _c2: u64) -> u64 { unimplemented!() }
pub fn igt_u64(c1: u64, c2: u64) -> u64 { if c1 > c2 { 1 } else { 0 } }
pub fn igt_s64(_c1: u64, _c2: u64) -> u64 { unimplemented!() }
pub fn ile_u64(c1: u64, c2: u64) -> u64 { if c1 <= c2 { 1 } else { 0 } }
pub fn ile_s64(_c1: u64, _c2: u64) -> u64 { unimplemented!() }
pub fn ige_u64(c1: u64, c2: u64) -> u64 { if c1 >= c2 { 1 } else { 0 } }
pub fn ige_s64(_c1: u64, _c2: u64) -> u64 { unimplemented!() }
// iextendM_s

pub fn fadd64(c1: f64, c2: f64) -> f64 { c1 + c2 }
pub fn fsub64(c1: f64, c2: f64) -> f64 { c1 - c2 }
pub fn fmul64(c1: f64, c2: f64) -> f64 { c1 * c2 }
pub fn fdiv64(c1: f64, c2: f64) -> f64 { c1 / c2 }
pub fn fmin64(c1: f64, c2: f64) -> f64 { if c1 < c2 { c1 } else { c2 } }
pub fn fmax64(c1: f64, c2: f64) -> f64 { if c1 > c2 { c1 } else { c2 } }
pub fn fcopysign64(c1: f64, c2: f64) -> f64 { c1.copysign(c2) }
pub fn fabs64(f: f64) -> f64 { f.abs() }
pub fn fneg64(f: f64) -> f64 { -f }
pub fn fsqrt64(f: f64) -> f64 { f.sqrt() }
pub fn fceil64(f: f64) -> f64 { f.ceil() }
pub fn ffloor64(f: f64) -> f64 { f.floor() }
pub fn ftrunc64(f: f64) -> f64 { f.trunc() }
pub fn fnearest64(f: f64) -> f64 { f.round() }
pub fn feq64(c1: f64, c2: f64) -> f64 { if c1 == c2 { 1.0 } else { 0.0 } }
pub fn fne64(c1: f64, c2: f64) -> f64 { if c1 != c2 { 1.0 } else { 0.0 } }
pub fn flt64(c1: f64, c2: f64) -> f64 { if c1 < c2 { 1.0 } else { 0.0 } }
pub fn fgt64(c1: f64, c2: f64) -> f64 { if c1 > c2 { 1.0 } else { 0.0 } }
pub fn fle64(c1: f64, c2: f64) -> f64 { if c1 <= c2 { 1.0 } else { 0.0 } }
pub fn fge64(c1: f64, c2: f64) -> f64 { if c1 >= c2 { 1.0 } else { 0.0 } }

// extend_u
// extend_s
// wrap
// trunc_u
// trunc_s
// trunc_sat_u
// trunc_sat_s
// promote
// demote
// convert_u
// convert_s
// reinterpret