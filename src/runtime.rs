mod operation32;
mod operation64;
mod thread;
mod unop;
mod binop;
mod testop;
mod relop;
mod cvtop;
mod parametric;
mod variable;
mod memory;
mod control;

pub use operation32::*;
pub use operation64::*;
pub use unop::*;
pub use binop::*;
pub use testop::*;
pub use relop::*;
pub use cvtop::*;
pub use parametric::*;
pub use variable::*;
pub use memory::*;
pub use control::*;

use crate::{
    FuncType,
    Byte,
    Name,
    Mut,
    Func,
    Instr,
    Error,
};

#[derive(Clone, Copy, PartialEq)]
pub enum Val {
    I32Const(u32),
    I64Const(u64),
    F32Const(f32),
    F64Const(f64),
}

impl Debug for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::I32Const(n) => write!(f, "i32({:?})", n),
            Val::I64Const(n) => write!(f, "i64({:?})", n),
            Val::F32Const(n) => write!(f, "f32({:?})", n),
            Val::F64Const(n) => write!(f, "f64({:?})", n),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ExecResult {
    Vals(Vec<Val>),
    Trap(Error),
}

impl ExecResult {
    pub fn i32val(n: u32) -> Self { ExecResult::Vals(vec![Val::I32Const(n)]) }
    pub fn i64val(n: u64) -> Self { ExecResult::Vals(vec![Val::I64Const(n)]) }
    pub fn f32val(n: f32) -> Self { ExecResult::Vals(vec![Val::F32Const(n)]) }
    pub fn f64val(n: f64) -> Self { ExecResult::Vals(vec![Val::F64Const(n)]) }
}

#[derive(Default, Debug)]
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
pub type GlobalAddr = Addr;

#[derive(Default, PartialEq, Clone, Debug)]
pub struct ModuleInst {
    pub types: Vec<FuncType>,
    pub funcaddrs: Vec<FuncAddr>,
    pub tableaddrs: Vec<TableAddr>,
    pub memaddrs: Vec<MemAddr>,
    pub globaladdrs: Vec<GlobalAddr>,
    pub exports: Vec<ExportInst>,
}

#[derive(Clone, Debug)]
pub enum FuncInst {
    User(UserFuncInst),
    Host(HostFuncInst),
}

impl FuncInst {
    pub fn user(tp: FuncType, module: ModuleInst, code: Func) -> FuncInst {
        FuncInst::User(UserFuncInst {tp, module, code})
    }
    pub fn host(tp: FuncType, hostcode: fn()) -> FuncInst {
        FuncInst::Host(HostFuncInst {tp, hostcode})
    }
}

#[derive(Clone, Debug)]
pub struct UserFuncInst {
    pub tp: FuncType,
    pub module: ModuleInst,
    code: Func,
}

#[derive(Clone, Debug)]
pub struct HostFuncInst {
    pub tp: FuncType,
    pub hostcode: fn(),
}

#[derive(Clone, Debug)]
pub struct TableInst {
    pub elem: Vec<FuncElem>,
    pub max: Option<u32>,
}
type FuncElem = Option<FuncAddr>;

#[derive(Clone, Debug)]
pub struct MemInst {
    pub data: Vec<Byte>,
    pub max: Option<u32>,
}

#[derive(Debug)]
pub struct GlobalInst {
    pub value: Val,
    pub mutability: Mut, 
}

#[derive(PartialEq, Clone, Debug)]
pub struct ExportInst {
    pub name: Name,
    pub value: ExternVal,
}

#[derive(PartialEq, Clone, Debug)]
pub enum ExternVal {
    Func(FuncAddr),
    Table(TableAddr),
    Mem(MemAddr),
    Global(GlobalAddr),
}

pub enum StackEntry {
    Value(Val),
    Label(u32, Vec<Instr>),
    Activation(u32, Frame),
}

use std::fmt::Debug;
impl Debug for StackEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackEntry::Value(v) => write!(f, "{:?}", v),
            StackEntry::Label(n, instrs) => write!(f, "Label{:?}<{:?}>", n, instrs),
            StackEntry::Activation(n, frame) => write!(f, "Frame{:?}{:?}", n, frame),
        }
    }
}

#[derive(Default, PartialEq, Clone)]
pub struct Frame {
    pub locals: Vec<Val>,
    pub module: ModuleInst,
}
impl Debug for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{:?}>", self.locals)
    }
}

#[derive(Debug)]
pub struct Thread<'a> {
    pub store: &'a mut Store,
    pub stack: Vec<StackEntry>,
}

impl<'a> Thread<'a> {
    pub fn new(store: &'a mut Store) -> Self {
        Thread {
            store: store,
            stack: vec![],
        }
    }
}

pub fn signed32(n: u32) -> i32 { i32::from_le_bytes(n.to_le_bytes()) }
pub fn signed64(n: u64) -> i64 { i64::from_le_bytes(n.to_le_bytes()) }
pub fn unsigned32(n: i32) -> u32 { u32::from_le_bytes(n.to_le_bytes()) }
pub fn unsigned64(n: i64) -> u64 { u64::from_le_bytes(n.to_le_bytes()) }
