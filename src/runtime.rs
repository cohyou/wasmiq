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

impl Result {
    pub fn i32val(n: u32) -> Self { Result::Vals(vec![Val::I32Const(n)]) }
    pub fn i64val(n: u64) -> Self { Result::Vals(vec![Val::I64Const(n)]) }
    pub fn f32val(n: f32) -> Self { Result::Vals(vec![Val::F32Const(n)]) }
    pub fn f64val(n: f64) -> Self { Result::Vals(vec![Val::F64Const(n)]) }
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

#[test]
fn test_trunc() {
    let n = f32::INFINITY;
    let i = n as i8;
    assert_eq!(f32::NEG_INFINITY as i8, 1i8);
    assert_eq!(i, 1i8);
    assert_eq!(1.0 as u8, 1u8);
}