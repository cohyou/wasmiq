mod operation32;
mod operation64;
mod thread;
mod unop;
mod binop;
mod testop;
mod relop;

pub use operation32::*;
pub use operation64::*;
pub use unop::*;
pub use binop::*;
pub use testop::*;
pub use relop::*;

// use std::rc::Weak;
// use std::collections::VecDeque;

use crate::{
    FuncType,
    Byte,
    Name,
    Mut,
    Func,
    Instr,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Val {
    I32Const(u32),
    I64Const(u64),
    F32Const(f32),
    F64Const(f64),
}

pub enum Result {
    Vals(Vec<Val>),
    Trap,
}

#[derive(Default)]
pub struct Store {
    pub funcs: Vec<FuncInst>,
    pub tables: Vec<TableInst>,
    pub mems: Vec<MemInst>,
    pub globals: Vec<GlobalInst>,
}

type Addr = usize;
pub type FuncAddr = Addr;
pub type TableAddr = Addr;
pub type MemAddr = Addr;
type GlobalAddr = Addr;

#[derive(Default, PartialEq, Clone)]
pub struct ModuleInst {
    types: Vec<FuncType>,
    funcaddrs: Vec<FuncAddr>,
    tableaddrs: Vec<TableAddr>,
    memaddrs: Vec<MemAddr>,
    globaladdrs: Vec<GlobalAddr>,
    exports: Vec<ExportInst>,
}

pub enum FuncInst {
    User(UserFuncInst),
    Host(HostFuncInst),
}

pub struct UserFuncInst {
    pub tp: FuncType,
    module: ModuleInst,
    code: Func,
}

pub struct HostFuncInst {
    pub tp: FuncType,
    hostcode: fn(),
}

pub struct TableInst {
    elem: Vec<FuncElem>,
    max: Option<u32>,
}
type FuncElem = Option<FuncAddr>;

pub struct MemInst {
    data: Vec<Byte>,
    max: Option<u32>,
}

pub struct GlobalInst {
    value: Val,
    mutability: Mut, 
}

#[derive(PartialEq, Clone)]
struct ExportInst {
    name: Name,
    value: ExternVal,
}

#[derive(PartialEq, Clone)]
pub enum ExternVal {
    Func(FuncAddr),
    Table(TableAddr),
    Mem(MemAddr),
    Global(GlobalAddr),
}

struct Stack(pub Vec<StackEntry>);

pub enum StackEntry {
    Value(Val),
    Label(u32, Vec<Instr>),
    Activation(u32, Frame),
}

#[derive(Default, PartialEq, Clone)]
pub struct Frame {
    locals: Vec<Val>,
    pub module: ModuleInst,
}

pub struct Thread {
    pub frame: Frame,
    pub instrs: Vec<Instr>,
    pub stack: Vec<StackEntry>,
}

impl Thread {
    pub fn trap_with_frame(frame: Frame) -> Self {
        Thread {
            frame: frame, 
            instrs: vec![Instr::Trap],
            stack: vec![],
        }
    }
}