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

#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(Clone)]
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

#[derive(Clone)]
pub struct UserFuncInst {
    pub tp: FuncType,
    module: ModuleInst,
    code: Func,
}

#[derive(Clone)]
pub struct HostFuncInst {
    pub tp: FuncType,
    pub hostcode: fn(),
}

#[derive(Clone)]
pub struct TableInst {
    pub elem: Vec<FuncElem>,
    pub max: Option<u32>,
}
type FuncElem = Option<FuncAddr>;

#[derive(Clone)]
pub struct MemInst {
    pub data: Vec<Byte>,
    pub max: Option<u32>,
}

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

#[derive(Default, PartialEq, Clone, Debug)]
pub struct Frame {
    pub locals: Vec<Val>,
    pub module: ModuleInst,
}

pub struct Thread<'a> {
    pub store: &'a mut Store,
    // pub frame: Frame,
    // pub instrs: Vec<Instr>,
    pub stack: Vec<StackEntry>,
}

impl<'a> Thread<'a> {
    pub fn new(store: &'a mut Store) -> Self {
        Thread {
            store: store,
            stack: vec![],
        }
    }
    // pub fn new_with_frame(store: &'a mut Store, frame: Frame) -> Self {
    //     Thread {
    //         store: store,
    //         stack: vec![],
    //     }
    // }
    // pub fn trap_with_frame(store: &'a mut Store, frame: Frame) -> Self {
    //     Thread {
    //         store: store,
    //         stack: vec![],
    //     }
    // }
}