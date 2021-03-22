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
};

#[derive(PartialEq)]
pub struct Expr(pub Vec<Instr>);

impl Expr {
    pub fn is_constant(&self) -> bool {
        unimplemented!()
    }
}

#[derive(PartialEq)]
pub struct MemArg {
    offset: u32,
    align: u32,
}

#[derive(PartialEq)]
pub enum BlockType {
    TypeIdx(TypeIdx),
    ValType(Option<ValType>),
}

#[derive(PartialEq)]
pub enum Instr {
    /* Block Instructions */

    // Control Instructions
    Block(BlockType, Vec<Instr>),
    Loop(BlockType, Vec<Instr>),
    If(BlockType, Vec<Instr>, Option<Vec<Instr>>),

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
    I32Const(i32),
    I64Const(i64),
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
    // InitElem(TableAddr, u32, Vec<Funcidx>),
    // InitData(MemAddr, u32, Vec<u8>),
    // Label(usize, Vec<Instr>, Vec<Instr>),
    // Frame(usize, Frame, Vec<Instr>),
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
    I32WrapFromI64,
    I64ExtendFromI32(ValSign),
    ITruncFromF(ValSize, ValSize, ValSign),
    F32DemoteFromF64,
    F64PromoteFromF32,
    FConvertFromI(ValSize, ValSize, ValSign),
    IReinterpretFromF(ValSize),
    FReinterpretFromI(ValSize),
}
