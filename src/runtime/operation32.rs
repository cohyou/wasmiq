use super::*;

pub fn iadd32(c1: u32, c2: u32) -> u32 { c1.wrapping_add(c2) }
pub fn isub32(c1: u32, c2: u32) -> u32 { c1.wrapping_sub(c2) }
pub fn imul32(c1: u32, c2: u32) -> u32 { c1.wrapping_mul(c2) }
pub fn idiv_u32(c1: u32, c2: u32) -> u32 { c1.wrapping_div(c2) }
pub fn idiv_s32(c1: u32, c2: u32) -> u32 { c1.wrapping_div_euclid(c2) }
pub fn irem_u32(c1: u32, c2: u32) -> u32 { c1.wrapping_rem(c2) }
pub fn irem_s32(c1: u32, c2: u32) -> u32 { c1.wrapping_rem_euclid(c2) }
pub fn iand32(c1: u32, c2: u32) -> u32 { c1 & c2 }
pub fn ior32(c1: u32, c2: u32) -> u32 { c1 | c2 }
pub fn ixor32(c1: u32, c2: u32) -> u32 { c1 ^ c2 }
pub fn ishl32(c1: u32, c2: u32) -> u32 { c1.wrapping_shl(c2) }
pub fn ishr_u32(c1: u32, c2: u32) -> u32 { c1.wrapping_shr(c2) }
pub fn ishr_s32(c1: u32, c2: u32) -> u32 { unsigned32(signed32(c1).wrapping_shr(c2)) }
pub fn irotl32(c1: u32, c2: u32) -> u32 { c1.rotate_left(c2 % 32) }
pub fn irotr32(c1: u32, c2: u32) -> u32 { c1.rotate_right(c2 % 32) }
pub fn iclz32(i: u32) -> u32 { i.leading_zeros() }
pub fn ictz32(i: u32) -> u32 { i.trailing_zeros() }
pub fn ipopcnt32(i: u32) -> u32 { i.count_ones() }
pub fn ieqz32(i: u32) -> u32 { if i == 0 { 1 } else { 0 } }
pub fn ieq32(c1: u32, c2: u32) -> u32 { if c1 == c2 { 1 } else { 0 } }
pub fn ine32(c1: u32, c2: u32) -> u32 { if c1 != c2 { 1 } else { 0 } }
pub fn ilt_u32(c1: u32, c2: u32) -> u32 { if c1 < c2 { 1 } else { 0 } }
pub fn ilt_s32(c1: u32, c2: u32) -> u32 { if signed32(c1) < signed32(c2) { 1 } else { 0 } }
pub fn igt_u32(c1: u32, c2: u32) -> u32 { if c1 > c2 { 1 } else { 0 } }
pub fn igt_s32(c1: u32, c2: u32) -> u32 { if signed32(c1) > signed32(c2) { 1 } else { 0 } }
pub fn ile_u32(c1: u32, c2: u32) -> u32 { if c1 <= c2 { 1 } else { 0 } }
pub fn ile_s32(c1: u32, c2: u32) -> u32 { if signed32(c1) <= signed32(c2) { 1 } else { 0 } }
pub fn ige_u32(c1: u32, c2: u32) -> u32 { if c1 >= c2 { 1 } else { 0 } }
pub fn ige_s32(c1: u32, c2: u32) -> u32 { if signed32(c1) >= signed32(c2) { 1 } else { 0 } }
// iextendM_s

pub fn fadd32(c1: f32, c2: f32) -> f32 { c1 + c2 }
pub fn fsub32(c1: f32, c2: f32) -> f32 { c1 - c2 }
pub fn fmul32(c1: f32, c2: f32) -> f32 { c1 * c2 }
pub fn fdiv32(c1: f32, c2: f32) -> f32 { c1 / c2 }
pub fn fmin32(c1: f32, c2: f32) -> f32 { c1.min(c2) }
pub fn fmax32(c1: f32, c2: f32) -> f32 { c1.max(c2) }
pub fn fcopysign32(c1: f32, c2: f32) -> f32 { c1.copysign(c2) }
pub fn fabs32(f: f32) -> f32 { f.abs() }
pub fn fneg32(f: f32) -> f32 { -f }
pub fn fsqrt32(f: f32) -> f32 { f.sqrt() }
pub fn fceil32(f: f32) -> f32 { f.ceil() }
pub fn ffloor32(f: f32) -> f32 { f.floor() }
pub fn ftrunc32(f: f32) -> f32 { f.trunc() }
pub fn fnearest32(f: f32) -> f32 { f.round() }
pub fn feq32(c1: f32, c2: f32) -> f32 { if c1 == c2 { 1.0 } else { 0.0 } }
pub fn fne32(c1: f32, c2: f32) -> f32 { if c1 != c2 { 1.0 } else { 0.0 } }
pub fn flt32(c1: f32, c2: f32) -> f32 { if c1 < c2 { 1.0 } else { 0.0 } }
pub fn fgt32(c1: f32, c2: f32) -> f32 { if c1 > c2 { 1.0 } else { 0.0 } }
pub fn fle32(c1: f32, c2: f32) -> f32 { if c1 <= c2 { 1.0 } else { 0.0 } }
pub fn fge32(c1: f32, c2: f32) -> f32 { if c1 >= c2 { 1.0 } else { 0.0 } }

// extend_u
// fn extend8s32(_c: u8) -> u32 { unimplemented!() }
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