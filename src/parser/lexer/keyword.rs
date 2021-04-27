use crate::{
    ValType,
    Instr,
    LabelIdx,
};

use crate::instr::*;
// use super::super::context::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Module,

    Type,
    Import,
    Func,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Elem,
    Data,

    Local,
    Param,
    Result,
    AnyFunc,
    Mutable,
    Offset,
    FuncRef,
    Else,
    End,

    Then,

    ValType(ValType),

    Instr(Instr),

    MemArgOffset(u32),
    MemArgAlign(u32),
}

pub(super) fn vec_to_keyword(s: &[u8]) -> Option<Keyword> {
    match s {
        b"module" => Some(Keyword::Module),

        b"type" => Some(Keyword::Type),
        b"import" => Some(Keyword::Import),
        b"func" => Some(Keyword::Func),
        b"table" => Some(Keyword::Table),
        b"memory" => Some(Keyword::Memory),
        b"global" => Some(Keyword::Global),
        b"export" => Some(Keyword::Export),
        b"start" => Some(Keyword::Start),
        b"elem" => Some(Keyword::Elem),
        b"data" => Some(Keyword::Data),

        b"local" => Some(Keyword::Local),
        b"param" => Some(Keyword::Param),
        b"result" => Some(Keyword::Result),
        b"anyfunc" => Some(Keyword::AnyFunc),
        b"mut" => Some(Keyword::Mutable),
        b"offset" => Some(Keyword::Offset),
        b"funcref" => Some(Keyword::FuncRef),
        b"else" => Some(Keyword::Else),
        b"end" => Some(Keyword::End),

        b"then" => Some(Keyword::Then),

        b"i32" | b"i64" | b"f32" | b"f64" => vec_to_valtype(s).map(|vt| Keyword::ValType(vt)),


        _ if s.starts_with(b"offset=") => {
            let mut s_iter = s.split(|&b| b == b'=');
            let _ = s_iter.next();
            s_iter.next().and_then(|offset|
                String::from_utf8(offset.to_vec()).ok().and_then(|offset|
                    offset.parse::<u32>().ok().and_then(|offset|
                        Some(Keyword::MemArgOffset(offset))
                    )
                )
            )
        },
        _ if s.starts_with(b"align=") => {
            let mut s_iter = s.split(|&b| b == b'=');
            let _ = s_iter.next();
            s_iter.next().and_then(|align|
                String::from_utf8(align.to_vec()).ok().and_then(|align|
                    align.parse::<u32>().ok().and_then(|align|
                        Some(Keyword::MemArgAlign(align))
                    )
                )
            )
        },

        _ => vec_to_instr(s).map(|instr| Keyword::Instr(instr)),
    }
}

fn vec_to_instr(s: &[u8]) -> Option<Instr> {
    let blocktype = BlockType::default();
    let bt = default_br_table();
    let memarg = MemArg::default();

    match s {
        b"block" => Some(Instr::Block(blocktype, vec![])),
        b"loop" => Some(Instr::Loop(blocktype, vec![])),
        b"if" => Some(Instr::If(blocktype, vec![], vec![])),

        b"unreachable" => Some(Instr::Unreachable),
        b"nop" => Some(Instr::Nop),
        b"br" => Some(Instr::Br(0)),
        b"br_if" => Some(Instr::BrIf(0)),
        b"br_table" => Some(Instr::BrTable(bt, 0)),
        b"return" => Some(Instr::Return),
        b"call" => Some(Instr::Call(0)),
        b"call_indirect" => Some(Instr::CallIndirect(0)),

        b"drop" => Some(Instr::Drop(None)),
        b"select" => Some(Instr::Select(None)),

        b"local.get" => Some(Instr::LocalGet(0)),
        b"local.set" => Some(Instr::LocalSet(0)),
        b"local.tee" => Some(Instr::LocalTee(0)),
        b"global.get" => Some(Instr::GlobalGet(0)),
        b"global.set" => Some(Instr::GlobalSet(0)),

        b"i64.store32" => Some(Instr::I64Store32(memarg)),
        b"memory.size" => Some(Instr::MemorySize),
        b"memory.grow" => Some(Instr::MemoryGrow),

        b"i32.wrap_i64" => Some(Instr::CvtOp(CvtOp::I32WrapFromI64)),
        b"i64.extend_i32_s" => Some(Instr::CvtOp(CvtOp::I64ExtendFromI32(ValSign::S))),
        b"i64.extend_i32_u" => Some(Instr::CvtOp(CvtOp::I64ExtendFromI32(ValSign::U))),

        b"i32.trunc_f32_s" => Some(Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V32, ValSign::S))),
        b"i32.trunc_f32_u" => Some(Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V32, ValSign::U))),
        b"i32.trunc_f64_s" => Some(Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V64, ValSign::S))),
        b"i32.trunc_f64_u" => Some(Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V64, ValSign::U))),
        b"i64.trunc_f32_s" => Some(Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V32, ValSign::S))),
        b"i64.trunc_f32_u" => Some(Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V32, ValSign::U))),
        b"i64.trunc_f64_s" => Some(Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V64, ValSign::S))),
        b"i64.trunc_f64_u" => Some(Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V64, ValSign::U))),

        b"f32.demote_f64" => Some(Instr::CvtOp(CvtOp::F32DemoteFromF64)),
        b"f64.promote_f32" => Some(Instr::CvtOp(CvtOp::F64PromoteFromF32)),

        b"f32.convert_i32_s" => Some(Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V32, ValSign::S))),
        b"f32.convert_i32_u" => Some(Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V32, ValSign::U))),
        b"f32.convert_i64_s" => Some(Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V64, ValSign::S))),
        b"f32.convert_i64_u" => Some(Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V64, ValSign::U))),
        b"f64.convert_i32_s" => Some(Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V32, ValSign::S))),
        b"f64.convert_i32_u" => Some(Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V32, ValSign::U))),
        b"f64.convert_i64_s" => Some(Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V64, ValSign::S))),
        b"f64.convert_i64_u" => Some(Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V64, ValSign::U))),

        b"i32.reinterpret_f32" => Some(Instr::CvtOp(CvtOp::IReinterpretFromF(ValSize::V32))),
        b"i64.reinterpret_f64" => Some(Instr::CvtOp(CvtOp::IReinterpretFromF(ValSize::V64))),
        b"f32.reinterpret_i32" => Some(Instr::CvtOp(CvtOp::FReinterpretFromI(ValSize::V32))),
        b"f64.reinterpret_i64" => Some(Instr::CvtOp(CvtOp::FReinterpretFromI(ValSize::V64))),

        _ => {
            let mut s_iter = s.split(|&b| b == b'.');
            let vt_b = s_iter.next().unwrap();
            let instr = s_iter.next().unwrap();

            let vt = vec_to_valtype(vt_b).unwrap();
            let vs = vec_to_valsize(vt_b).unwrap();
            match instr {
                b"load" => Some(Instr::Load(vt, memarg)),
                b"store" => Some(Instr::Store(vt, memarg)),
                b"store8" => Some(Instr::IStore8(vs, memarg)),
                b"store16" => Some(Instr::IStore16(vs, memarg)),
                b"const" => {
                    match vt {
                        ValType::I32 => Some(Instr::I32Const(0)),
                        ValType::I64 => Some(Instr::I64Const(0)),
                        ValType::F32 => Some(Instr::F32Const(0.0)),
                        ValType::F64 => Some(Instr::F64Const(0.0)),
                    }
                }
                b"clz" => Some(Instr::IUnOp(vs, IUnOp::Clz)),
                b"ctz" => Some(Instr::IUnOp(vs, IUnOp::Ctz)),
                b"popcnt" => Some(Instr::IUnOp(vs, IUnOp::Popcnt)),

                b"add" | b"sub" | b"mul" => {
                    match vt {
                        ValType::I32 | ValType::I64 => {
                            Some(Instr::IBinOp(vs, vec_to_ibinop(instr).unwrap()))
                        },
                        ValType::F32 | ValType::F64 => {
                            Some(Instr::FBinOp(vs, vec_to_fbinop(instr).unwrap()))
                        },                        
                    }                    
                },

                // b"mul" => Some(Instr::IBinOp(vs, IBinOp::Mul)),
                // b"and" => Some(Instr::IBinOp(vs, IBinOp::And)),
                b"or" => Some(Instr::IBinOp(vs, IBinOp::Or)),
                b"xor" => Some(Instr::IBinOp(vs, IBinOp::Xor)),
                b"shl" => Some(Instr::IBinOp(vs, IBinOp::Shl)),
                b"rotl" => Some(Instr::IBinOp(vs, IBinOp::Rotl)),
                b"rotr" => Some(Instr::IBinOp(vs, IBinOp::Rotr)),

                b"abs" => Some(Instr::FUnOp(vs, FUnOp::Abs)),
                b"neg" => Some(Instr::FUnOp(vs, FUnOp::Neg)),
                b"ceil" => Some(Instr::FUnOp(vs, FUnOp::Ceil)),
                b"floor" => Some(Instr::FUnOp(vs, FUnOp::Floor)),
                b"trunc" => Some(Instr::FUnOp(vs, FUnOp::Trunc)),
                b"nearest" => Some(Instr::FUnOp(vs, FUnOp::Nearest)),
                b"sqrt" => Some(Instr::FUnOp(vs, FUnOp::Sqrt)),
                
                b"div" => Some(Instr::FBinOp(vs, FBinOp::Div)),
                b"min" => Some(Instr::FBinOp(vs, FBinOp::Min)),
                b"max" => Some(Instr::FBinOp(vs, FBinOp::Max)),
                b"copysign" => Some(Instr::FBinOp(vs, FBinOp::Copysign)),

                b"eqz" => Some(Instr::ITestOp(vs, ITestOp::Eqz)),

                b"eq" | b"ne" => {
                    match vt {
                        ValType::I32 | ValType::I64 => {
                            Some(Instr::IRelOp(vs, vec_to_irelop(instr).unwrap()))
                        },
                        ValType::F32 | ValType::F64 => {
                            Some(Instr::FRelOp(vs, vec_to_frelop(instr).unwrap()))
                        },                        
                    }                    
                }

                b"lt" | b"gt" | b"le" | b"ge" => Some(Instr::FRelOp(vs, vec_to_frelop(instr).unwrap())),

                _ => {
                    let instr_tokens: Vec<&[u8]> = instr.split(|&b| b == b'_').collect();
                    let sign = vec_to_valsign(instr_tokens[1]).unwrap();
                    match instr_tokens[0] {
                        b"load8" => Some(Instr::ILoad8(vs, sign, memarg)),
                        b"load16" => Some(Instr::ILoad16(vs, sign, memarg)),
                        b"load32" => {
                            if vs == ValSize::V64 { Some(Instr::I64Load32(sign, memarg)) } else { None }
                        },
                        b"div" => Some(Instr::IBinOp(vs, IBinOp::Div(sign))),
                        b"rem" => Some(Instr::IBinOp(vs, IBinOp::Rem(sign))),
                        b"shr" => Some(Instr::IBinOp(vs, IBinOp::Shr(sign))),

                        b"lt" => Some(Instr::IRelOp(vs, IRelOp::Lt(sign))),
                        b"gt" => Some(Instr::IRelOp(vs, IRelOp::Gt(sign))),
                        b"le" => Some(Instr::IRelOp(vs, IRelOp::Le(sign))),
                        b"ge" => Some(Instr::IRelOp(vs, IRelOp::Ge(sign))),

                        _ => panic!("invalid instr name: {:?}", String::from_utf8(s.to_vec())),
                    }
                }
            }            
        },        
    }
}

fn vec_to_valtype(s: &[u8]) -> Option<ValType> {
    match s {
        b"i32" => Some(ValType::I32),
        b"i64" => Some(ValType::I64),
        b"f32" => Some(ValType::F32),
        b"f64" => Some(ValType::F64),
        _ => None,
    }
}

fn vec_to_valsize(s: &[u8]) -> Option<ValSize> {
    match s {
        b"i32" | b"f32" => Some(ValSize::V32),
        b"i64" | b"f64" => Some(ValSize::V64),
        _ => None,
    }
}

fn vec_to_valsign(s: &[u8]) -> Option<ValSign> {
    match s {
        b"s" => Some(ValSign::S),
        b"u" => Some(ValSign::U),
        _ => None,
    }
}

fn vec_to_ibinop(s: &[u8]) -> Option<IBinOp> {
    match s {
        b"add" => Some(IBinOp::Add),
        b"sub" => Some(IBinOp::Sub),
        b"mul" => Some(IBinOp::Mul),
        _ => None,
    }
}

fn vec_to_fbinop(s: &[u8]) -> Option<FBinOp> {
    match s {
        b"add" => Some(FBinOp::Add),
        b"sub" => Some(FBinOp::Sub),
        b"mul" => Some(FBinOp::Mul),
        _ => None,
    }
}

fn vec_to_irelop(s: &[u8]) -> Option<IRelOp> {
    match s {
        b"eq" => Some(IRelOp::Eq),
        b"ne" => Some(IRelOp::Ne),
        _ => None,
    }
}

fn vec_to_frelop(s: &[u8]) -> Option<FRelOp> {
    match s {
        b"eq" => Some(FRelOp::Eq),
        b"ne" => Some(FRelOp::Ne),
        b"lt" => Some(FRelOp::Lt),
        b"gt" => Some(FRelOp::Gt),
        b"le" => Some(FRelOp::Le),
        b"ge" => Some(FRelOp::Ge),
        _ => None,
    }
}

// fn default_result_type() -> ResultType { vec![] }
fn default_br_table() -> Vec<LabelIdx> { vec![] }

use std::fmt::Display;
impl Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Keyword::Module => write!(f, "module"),
            Keyword::Param => write!(f, "param"),
            Keyword::Result => write!(f, "result"),
            Keyword::Else => write!(f, "else"),
            Keyword::End => write!(f, "end"),
            Keyword::Type => write!(f, "type"),
            Keyword::Func => write!(f, "func"),
            Keyword::Import => write!(f, "import"),
            Keyword::Mutable => write!(f, "mut"),
            Keyword::Global => write!(f, "global"),
            Keyword::Elem => write!(f, "elem"),
            Keyword::Data => write!(f, "data"),
            Keyword::Offset => write!(f, "offset"),
            Keyword::Export => write!(f, "export"),
            Keyword::Table => write!(f, "table"),
            Keyword::Memory => write!(f, "memory"),
            Keyword::FuncRef => write!(f, "funcref"),
            Keyword::Local => write!(f, "local"),
            Keyword::ValType(ValType::I32) => write!(f, "i32"),
            Keyword::ValType(ValType::I64) => write!(f, "i64"),
            Keyword::ValType(ValType::F32) => write!(f, "f32"),
            Keyword::ValType(ValType::F64) => write!(f, "f64"),
            Keyword::Instr(Instr::Nop) => write!(f, "nop"),
            Keyword::Instr(Instr::Unreachable) => write!(f, "unreachable"),
            Keyword::Instr(Instr::Block(_, _)) => write!(f, "block"),
            Keyword::Instr(Instr::Loop(_, _)) => write!(f, "loop"),
            Keyword::Instr(Instr::If(_, _, _)) => write!(f, "if"),
            Keyword::Instr(Instr::I32Const(_)) => write!(f, "i32.const"),
            Keyword::Instr(Instr::I64Const(_)) => write!(f, "i64.const"),
            Keyword::Instr(Instr::F32Const(_)) => write!(f, "f32.const"),
            Keyword::Instr(Instr::F64Const(_)) => write!(f, "f64.const"),
            Keyword::Instr(Instr::LocalGet(_)) => write!(f, "local.get"),
            Keyword::Instr(Instr::Drop(_)) => write!(f, "drop"),
            Keyword::Instr(Instr::Select(_)) => write!(f, "select"),
            Keyword::Instr(Instr::IBinOp(ValSize::V32, IBinOp::Add)) => write!(f, "i32.add"),
            Keyword::Instr(Instr::IBinOp(ValSize::V32, IBinOp::Sub)) => write!(f, "i32.sub"),
            Keyword::Instr(Instr::IRelOp(ValSize::V32, IRelOp::Lt(ValSign::S))) => write!(f, "i32.lt_s"),
            Keyword::Instr(instr) => write!(f, "{:?}", instr),
            _ => write!(f, "{:?}", self),
        }
    }
}
