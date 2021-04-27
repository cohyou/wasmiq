mod validate;

pub use validate::{
    vt,
    vt_rev,
};

use crate::{
    ValType,
    TypeIdx,
    FuncIdx,
    GlobalIdx,
    LocalIdx,
    LabelIdx,
    FuncType,
    ModuleInst,
};

#[derive(PartialEq, Clone, Default, Debug)]
pub struct Expr(pub Vec<Instr>);

#[derive(PartialEq, Clone, Debug, Default)]
pub struct MemArg {
    pub offset: u32,
    pub align: u32,
}

#[derive(PartialEq, Clone, Debug)]
pub enum BlockType {
    TypeIdx(TypeIdx),
    ValType(Option<ValType>),
}
impl BlockType {
    pub fn extend(&self, moduleinst: &ModuleInst) -> FuncType {
        match self {
            BlockType::TypeIdx(typeidx) => {
                moduleinst.types[typeidx.clone() as usize].clone()
            },
            BlockType::ValType(None) => {
                (vec![], vec![])
            },
            BlockType::ValType(Some(valtype)) => {
                (vec![], vec![valtype.clone()])
            },
        }
    }
}
impl Default for BlockType {
    fn default() -> Self {
        BlockType::ValType(None)
    }
}
#[derive(PartialEq, Clone, Debug)]
pub enum Instr {
    /* Block Instructions */

    // Control Instructions
    Block(BlockType, Vec<Instr>),
    Loop(BlockType, Vec<Instr>),
    If(BlockType, Vec<Instr>, Vec<Instr>),

    /* Plain Instructions */

    // Control Instructions
    Unreachable,
    Nop,
    Br(LabelIdx),
    BrIf(LabelIdx),
    BrTable(Vec<LabelIdx>, LabelIdx),
    Return,
    Call(FuncIdx),
    CallIndirect(FuncIdx),

    // Parametric Instructions
    Drop(Option<ValType>),
    Select(Option<ValType>),

    // Variable Instructions
    LocalGet(LocalIdx),
    LocalSet(LocalIdx),
    LocalTee(LocalIdx),
    GlobalGet(GlobalIdx),
    GlobalSet(GlobalIdx),

    // Memory Instructions
    Load(ValType, MemArg),
    Store(ValType, MemArg),
    ILoad8(ValSize, ValSign, MemArg),
    ILoad16(ValSize, ValSign, MemArg),
    I64Load32(ValSign, MemArg),
    IStore8(ValSize, MemArg),
    IStore16(ValSize, MemArg),
    I64Store32(MemArg),
    MemorySize,
    MemoryGrow,

    // Numeric Instructions
    I32Const(u32),
    I64Const(u64),
    F32Const(f32),
    F64Const(f64),

    IUnOp(ValSize, IUnOp),
    FUnOp(ValSize, FUnOp),

    IBinOp(ValSize, IBinOp),
    FBinOp(ValSize, FBinOp),

    ITestOp(ValSize, ITestOp),

    IRelOp(ValSize, IRelOp),
    FRelOp(ValSize, FRelOp),

    CvtOp(CvtOp),

    // Administrative Instructions
    // Trap,
    // Invoke(FuncAddr),
    // InitElem(TableAddr, u32, Vec<FuncIdx>),
    // InitData(MemAddr, u32, Vec<u8>),
    // Label(u32, Vec<Instr>, Vec<Instr>),
    // Frame(u32, Frame, Vec<Instr>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValSize { V32, V64 }

#[derive(Debug, PartialEq, Clone)]
pub enum ValSign { U, S }

#[derive(Debug, Clone, PartialEq)]
pub enum IUnOp { Clz, Ctz, Popcnt, }

#[derive(Debug, Clone, PartialEq)]
pub enum IBinOp {
    Add, Sub, Mul, Div(ValSign), Rem(ValSign),
    And, Or, Xor, Shl, Shr(ValSign), Rotl, Rotr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FUnOp { Abs, Neg, Sqrt, Ceil, Floor, Trunc, Nearest, }

#[derive(Debug, Clone, PartialEq)]
pub enum FBinOp { Add, Sub, Mul, Div, Min, Max, Copysign, }

#[derive(Debug, Clone, PartialEq)]
pub enum ITestOp { Eqz, }

#[derive(Debug, Clone, PartialEq)]
pub enum IRelOp { Eq, Ne, Lt(ValSign), Gt(ValSign), Le(ValSign), Ge(ValSign), }

#[derive(Debug, Clone, PartialEq)]
pub enum FRelOp { Eq, Ne, Lt, Gt, Le, Ge, }

#[derive(Debug, Clone, PartialEq)]
pub enum CvtOp {
    IExtend8S(ValSize),
    IExtend16S(ValSize),
    I64Extend32S,
    I32WrapFromI64,
    I64ExtendFromI32(ValSign),
    ITruncFromF(ValSize, ValSize, ValSign),
    ITruncSatFromF(ValSize, ValSize, ValSign),
    F32DemoteFromF64,
    F64PromoteFromF32,
    FConvertFromI(ValSize, ValSize, ValSign),
    IReinterpretFromF(ValSize),
    FReinterpretFromI(ValSize),
}
