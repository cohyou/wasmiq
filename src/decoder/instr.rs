use std::io::Read;
use crate::{
    ValType, 
    
    Byte,
    // Expr,
};
use super::{
    byte_to_valtype,
};
use super::expr::{
    // decode_expr,
    decode_instrs,
};

use super::idx::{decode_labelidx, decode_labelindices, decode_funcidx, decode_localidx, decode_globalidx};
use super::decode_u32_from_leb128;
use crate::{Instr, BlockType, MemArg, ValSize, ValSign, ITestOp, IRelOp, FRelOp, IUnOp, IBinOp, FUnOp, FBinOp, CvtOp};


pub fn decode_instr(b: Byte, reader: &mut impl Read) -> Instr {
    match b {
        //////////////////////////
        // Control Instructions //
        //////////////////////////
        0x00 => Instr::Unreachable,
        0x01 => Instr::Nop,
        0x02 => {
            let block_type = decode_blocktype(reader);
            let instrs = decode_instrs(reader);
            Instr::Block(block_type, instrs)
        },
        0x03 => {
            let block_type = decode_blocktype(reader);
            let expr = decode_instrs(reader);
            Instr::Loop(block_type, expr)
        },
        0x04 => {
            let block_type = decode_blocktype(reader);
            let mut instrs_true = vec![];
            let mut expr_false = None;
            let mut instrs = vec![];

            loop {
                if let Some(Ok(b)) = reader.bytes().next() {
                    if b == 0x05 {
                        // else
                        instrs_true = instrs;
                        expr_false = Some(decode_instrs(reader));
                        break;
                    }  
                    if b == 0x08 {
                        // end
                        instrs_true = instrs;
                        break;
                    }  

                    let instr = decode_instr(b, reader);
                    instrs.push(instr);
                } else {
                    break;
                }
            }

            Instr::If(block_type, instrs_true, expr_false)
        },
        0x0C => Instr::Br(decode_labelidx(reader)),
        0x0D => Instr::BrIf(decode_labelidx(reader)),
        0x0E => Instr::BrTable(decode_labelindices(reader), decode_labelidx(reader)),
        0x0F => Instr::Return,
        0x10 => Instr::Call(decode_funcidx(reader)),
        0x11 => {
            let funcidx = decode_funcidx(reader);
            if let Some(Ok(b)) = reader.bytes().next() {
                if b == 0x00 {
                    // 0x00 is table index (for future)
                    Instr::CallIndirect(funcidx)
                } else {
                    panic!("invaild on decode_instr CALL INDIRECT");
                }
            } else {
                panic!("invaild on decode_instr CALL INDIRECT");
            }
        },

        /////////////////////////////
        // Parametric Instructions //
        /////////////////////////////
        0x1A => Instr::Drop(None),
        0x1B => Instr::Select(None),

        ///////////////////////////
        // Variable Instructions //
        ///////////////////////////
        0x20 => Instr::LocalGet(decode_localidx(reader)),
        0x21 => Instr::LocalSet(decode_localidx(reader)),
        0x22 => Instr::LocalTee(decode_localidx(reader)),
        0x23 => Instr::GlobalGet(decode_globalidx(reader)),
        0x24 => Instr::GlobalSet(decode_globalidx(reader)),

        /////////////////////////
        // Memory Instructions //
        /////////////////////////
        0x28 => Instr::Load(ValType::I32, decode_memarg(reader)),
        0x29 => Instr::Load(ValType::I64, decode_memarg(reader)),
        0x2A => Instr::Load(ValType::F32, decode_memarg(reader)),
        0x2B => Instr::Load(ValType::F64, decode_memarg(reader)),

        0x2C => Instr::ILoad8(ValSize::V32, ValSign::S, decode_memarg(reader)),
        0x2D => Instr::ILoad8(ValSize::V32, ValSign::U, decode_memarg(reader)),
        0x2E => Instr::ILoad16(ValSize::V32, ValSign::S, decode_memarg(reader)),
        0x2F => Instr::ILoad16(ValSize::V32, ValSign::U, decode_memarg(reader)),

        0x30 => Instr::ILoad8(ValSize::V64, ValSign::S, decode_memarg(reader)),
        0x31 => Instr::ILoad8(ValSize::V64, ValSign::U, decode_memarg(reader)),
        0x32 => Instr::ILoad16(ValSize::V64, ValSign::S, decode_memarg(reader)),
        0x33 => Instr::ILoad16(ValSize::V64, ValSign::U, decode_memarg(reader)),
        0x34 => Instr::I64Load32(ValSign::S, decode_memarg(reader)),
        0x35 => Instr::I64Load32(ValSign::U, decode_memarg(reader)),

        0x36 => Instr::Store(ValType::I32, decode_memarg(reader)),
        0x37 => Instr::Store(ValType::I64, decode_memarg(reader)),
        0x38 => Instr::Store(ValType::F32, decode_memarg(reader)),
        0x39 => Instr::Store(ValType::F64, decode_memarg(reader)),

        0x3A => Instr::IStore8(ValSize::V32, decode_memarg(reader)),
        0x3B => Instr::IStore16(ValSize::V32, decode_memarg(reader)),
        0x3C => Instr::IStore8(ValSize::V64, decode_memarg(reader)),
        0x3D => Instr::IStore16(ValSize::V64, decode_memarg(reader)),
        0x3E => Instr::I64Store32(decode_memarg(reader)),

        0x3F => {
            if let Some(Ok(b)) = reader.bytes().next() {
                if b == 0x00 {
                    // 0x00 is table index (for future)
                    Instr::MemorySize
                } else {
                    panic!("invaild on decode_instr CALL INDIRECT");
                }
            } else {
                panic!("invaild on decode_instr CALL INDIRECT");
            }
        },
        0x40 => {
            if let Some(Ok(b)) = reader.bytes().next() {
                if b == 0x00 {
                    // 0x00 is table index (for future)
                    Instr::MemoryGrow
                } else {
                    panic!("invaild on decode_instr CALL INDIRECT");
                }
            } else {
                panic!("invaild on decode_instr CALL INDIRECT");
            }
        },

        //////////////////////////
        // Numeric Instructions //
        //////////////////////////
        0x41 => Instr::I32Const(0),  // TODO: i32 (readerがない)
        0x42 => Instr::I64Const(0),  // TODO: i32 (readerがない)
        0x43 => Instr::F32Const(0.0),  // TODO: f32 (readerがない)
        0x44 => Instr::F64Const(0.0),  // TODO: f64 (readerがない)

        0x45 => Instr::ITestOp(ValSize::V32, ITestOp::Eqz),
        0x46 => Instr::IRelOp(ValSize::V32, IRelOp::Eq),
        0x47 => Instr::IRelOp(ValSize::V32, IRelOp::Ne),
        0x48 => Instr::IRelOp(ValSize::V32, IRelOp::Lt(ValSign::S)),
        0x49 => Instr::IRelOp(ValSize::V32, IRelOp::Lt(ValSign::U)),
        0x4A => Instr::IRelOp(ValSize::V32, IRelOp::Gt(ValSign::S)),
        0x4B => Instr::IRelOp(ValSize::V32, IRelOp::Gt(ValSign::U)),
        0x4C => Instr::IRelOp(ValSize::V32, IRelOp::Le(ValSign::S)),
        0x4D => Instr::IRelOp(ValSize::V32, IRelOp::Le(ValSign::U)),
        0x4E => Instr::IRelOp(ValSize::V32, IRelOp::Ge(ValSign::S)),
        0x4F => Instr::IRelOp(ValSize::V32, IRelOp::Ge(ValSign::U)),

        0x50 => Instr::ITestOp(ValSize::V64, ITestOp::Eqz),
        0x51 => Instr::IRelOp(ValSize::V64, IRelOp::Eq),
        0x52 => Instr::IRelOp(ValSize::V64, IRelOp::Ne),
        0x53 => Instr::IRelOp(ValSize::V64, IRelOp::Lt(ValSign::S)),
        0x54 => Instr::IRelOp(ValSize::V64, IRelOp::Lt(ValSign::U)),
        0x55 => Instr::IRelOp(ValSize::V64, IRelOp::Gt(ValSign::S)),
        0x56 => Instr::IRelOp(ValSize::V64, IRelOp::Gt(ValSign::U)),
        0x57 => Instr::IRelOp(ValSize::V64, IRelOp::Le(ValSign::S)),
        0x58 => Instr::IRelOp(ValSize::V64, IRelOp::Le(ValSign::U)),
        0x59 => Instr::IRelOp(ValSize::V64, IRelOp::Ge(ValSign::S)),
        0x5A => Instr::IRelOp(ValSize::V64, IRelOp::Ge(ValSign::U)),

        0x5B => Instr::FRelOp(ValSize::V32, FRelOp::Eq),
        0x5C => Instr::FRelOp(ValSize::V32, FRelOp::Ne),
        0x5D => Instr::FRelOp(ValSize::V32, FRelOp::Lt),
        0x5E => Instr::FRelOp(ValSize::V32, FRelOp::Gt),
        0x5F => Instr::FRelOp(ValSize::V32, FRelOp::Le),
        0x60 => Instr::FRelOp(ValSize::V32, FRelOp::Ge),

        0x61 => Instr::FRelOp(ValSize::V64, FRelOp::Eq),
        0x62 => Instr::FRelOp(ValSize::V64, FRelOp::Ne),
        0x63 => Instr::FRelOp(ValSize::V64, FRelOp::Lt),
        0x64 => Instr::FRelOp(ValSize::V64, FRelOp::Gt),
        0x65 => Instr::FRelOp(ValSize::V64, FRelOp::Le),
        0x66 => Instr::FRelOp(ValSize::V64, FRelOp::Ge),

        0x67 => Instr::IUnOp(ValSize::V32, IUnOp::Clz),
        0x68 => Instr::IUnOp(ValSize::V32, IUnOp::Ctz),
        0x69 => Instr::IUnOp(ValSize::V32, IUnOp::Popcnt),
        0x6A => Instr::IBinOp(ValSize::V32, IBinOp::Add),
        0x6B => Instr::IBinOp(ValSize::V32, IBinOp::Sub),
        0x6C => Instr::IBinOp(ValSize::V32, IBinOp::Mul),
        0x6D => Instr::IBinOp(ValSize::V32, IBinOp::Div(ValSign::S)),
        0x6E => Instr::IBinOp(ValSize::V32, IBinOp::Div(ValSign::U)),
        0x6F => Instr::IBinOp(ValSize::V32, IBinOp::Rem(ValSign::S)),
        0x70 => Instr::IBinOp(ValSize::V32, IBinOp::Rem(ValSign::U)),
        0x71 => Instr::IBinOp(ValSize::V32, IBinOp::And),
        0x72 => Instr::IBinOp(ValSize::V32, IBinOp::Or),
        0x73 => Instr::IBinOp(ValSize::V32, IBinOp::Xor),
        0x74 => Instr::IBinOp(ValSize::V32, IBinOp::Shl),
        0x75 => Instr::IBinOp(ValSize::V32, IBinOp::Shr(ValSign::S)),
        0x76 => Instr::IBinOp(ValSize::V32, IBinOp::Shr(ValSign::U)),
        0x77 => Instr::IBinOp(ValSize::V32, IBinOp::Rotl),
        0x78 => Instr::IBinOp(ValSize::V32, IBinOp::Rotr),

        0x79 => Instr::IUnOp(ValSize::V64, IUnOp::Clz),
        0x7A => Instr::IUnOp(ValSize::V64, IUnOp::Ctz),
        0x7B => Instr::IUnOp(ValSize::V64, IUnOp::Popcnt),
        0x7C => Instr::IBinOp(ValSize::V64, IBinOp::Add),
        0x7D => Instr::IBinOp(ValSize::V64, IBinOp::Sub),
        0x7E => Instr::IBinOp(ValSize::V64, IBinOp::Mul),
        0x7F => Instr::IBinOp(ValSize::V64, IBinOp::Div(ValSign::S)),
        0x80 => Instr::IBinOp(ValSize::V64, IBinOp::Div(ValSign::U)),
        0x81 => Instr::IBinOp(ValSize::V64, IBinOp::Rem(ValSign::S)),
        0x82 => Instr::IBinOp(ValSize::V64, IBinOp::Rem(ValSign::U)),
        0x83 => Instr::IBinOp(ValSize::V64, IBinOp::And),
        0x84 => Instr::IBinOp(ValSize::V64, IBinOp::Or),
        0x85 => Instr::IBinOp(ValSize::V64, IBinOp::Xor),
        0x86 => Instr::IBinOp(ValSize::V64, IBinOp::Shl),
        0x87 => Instr::IBinOp(ValSize::V64, IBinOp::Shr(ValSign::S)),
        0x88 => Instr::IBinOp(ValSize::V64, IBinOp::Shr(ValSign::U)),
        0x89 => Instr::IBinOp(ValSize::V64, IBinOp::Rotl),
        0x8A => Instr::IBinOp(ValSize::V64, IBinOp::Rotr),

        0x8B => Instr::FUnOp(ValSize::V32, FUnOp::Abs),
        0x8C => Instr::FUnOp(ValSize::V32, FUnOp::Neg),
        0x8D => Instr::FUnOp(ValSize::V32, FUnOp::Ceil),
        0x8E => Instr::FUnOp(ValSize::V32, FUnOp::Floor),
        0x8F => Instr::FUnOp(ValSize::V32, FUnOp::Trunc),
        0x90 => Instr::FUnOp(ValSize::V32, FUnOp::Nearest),
        0x91 => Instr::FUnOp(ValSize::V32, FUnOp::Sqrt),
        0x92 => Instr::FBinOp(ValSize::V32, FBinOp::Add),
        0x93 => Instr::FBinOp(ValSize::V32, FBinOp::Sub),
        0x94 => Instr::FBinOp(ValSize::V32, FBinOp::Mul),
        0x95 => Instr::FBinOp(ValSize::V32, FBinOp::Div),
        0x96 => Instr::FBinOp(ValSize::V32, FBinOp::Min),
        0x97 => Instr::FBinOp(ValSize::V32, FBinOp::Max),
        0x98 => Instr::FBinOp(ValSize::V32, FBinOp::Copysign),

        0x99 => Instr::FUnOp(ValSize::V64, FUnOp::Abs),
        0x9A => Instr::FUnOp(ValSize::V64, FUnOp::Neg),
        0x9B => Instr::FUnOp(ValSize::V64, FUnOp::Ceil),
        0x9C => Instr::FUnOp(ValSize::V64, FUnOp::Floor),
        0x9D => Instr::FUnOp(ValSize::V64, FUnOp::Trunc),
        0x9E => Instr::FUnOp(ValSize::V64, FUnOp::Nearest),
        0x9F => Instr::FUnOp(ValSize::V64, FUnOp::Sqrt),
        0xA0 => Instr::FBinOp(ValSize::V64, FBinOp::Add),
        0xA1 => Instr::FBinOp(ValSize::V64, FBinOp::Sub),
        0xA2 => Instr::FBinOp(ValSize::V64, FBinOp::Mul),
        0xA3 => Instr::FBinOp(ValSize::V64, FBinOp::Div),
        0xA4 => Instr::FBinOp(ValSize::V64, FBinOp::Min),
        0xA5 => Instr::FBinOp(ValSize::V64, FBinOp::Max),
        0xA6 => Instr::FBinOp(ValSize::V64, FBinOp::Copysign),

        0xA7 => Instr::CvtOp(CvtOp::I32WrapFromI64),
        0xA8 => Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V32, ValSign::S)),
        0xA9 => Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V32, ValSign::U)),
        0xAA => Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V64, ValSign::S)),
        0xAB => Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V64, ValSign::U)),
        0xAC => Instr::CvtOp(CvtOp::I64ExtendFromI32(ValSign::S)),
        0xAD => Instr::CvtOp(CvtOp::I64ExtendFromI32(ValSign::U)),
        0xAE => Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V32, ValSign::S)),
        0xAF => Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V32, ValSign::U)),
        0xB0 => Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V64, ValSign::S)),
        0xB1 => Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V64, ValSign::U)),
        0xB2 => Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V32, ValSign::S)),
        0xB3 => Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V32, ValSign::U)),
        0xB4 => Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V64, ValSign::S)),
        0xB5 => Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V64, ValSign::U)),
        0xB6 => Instr::CvtOp(CvtOp::F32DemoteFromF64),
        0xB7 => Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V32, ValSign::S)),
        0xB8 => Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V32, ValSign::U)),
        0xB9 => Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V64, ValSign::S)),
        0xBA => Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V64, ValSign::U)),
        0xBB => Instr::CvtOp(CvtOp::F64PromoteFromF32),
        0xBC => Instr::CvtOp(CvtOp::IReinterpretFromF(ValSize::V32)),
        0xBD => Instr::CvtOp(CvtOp::IReinterpretFromF(ValSize::V64)),
        0xBE => Instr::CvtOp(CvtOp::FReinterpretFromI(ValSize::V32)),
        0xBF => Instr::CvtOp(CvtOp::FReinterpretFromI(ValSize::V64)),

        0xC0 => unimplemented!(),  // i32.extend8_s
        0xC1 => unimplemented!(),  // i32.extend16_s
        0xC2 => unimplemented!(),  // i64.extend8_s
        0xC3 => unimplemented!(),  // i64.extend16_s
        0xC4 => unimplemented!(),  // i64.extend32_s

        0xFC => {
            // saturating truncation
            let variable_length = decode_u32_from_leb128(reader);

            match variable_length {
                0 => unimplemented!(),  // i32.trunc_sat_f32_s
                1 => unimplemented!(),  // i32.trunc_sat_f32_u
                2 => unimplemented!(),  // i32.trunc_sat_f64_s
                3 => unimplemented!(),  // i32.trunc_sat_f64_u
                4 => unimplemented!(),  // i64.trunc_sat_f32_s
                5 => unimplemented!(),  // i64.trunc_sat_f32_u
                6 => unimplemented!(),  // i64.trunc_sat_f64_s
                7 => unimplemented!(),  // i64.trunc_sat_f64_u
                _ => panic!("invalid on decode_instr"),
            }
        },
        _ => unimplemented!(), 
    }
}

fn decode_blocktype(reader: &mut impl Read) -> BlockType {
    // TODO: s33ではなく、u32で読んでいる
    if let Some(Ok(b)) = reader.bytes().next() {
        match b {
            0x40 => BlockType::ValType(None),
            0x7F | 0x7E | 0x7D | 0x7C => BlockType::ValType(Some(byte_to_valtype(b))),
            _ => BlockType::TypeIdx(decode_u32_from_leb128(reader)), 
        }
    } else {
        panic!("invalid on decode_blocktype");
    }
    
}

fn decode_memarg(reader: &mut impl Read) -> MemArg {
    let align = decode_u32_from_leb128(reader);
    let offset = decode_u32_from_leb128(reader);
    MemArg { align: align, offset: offset }
}