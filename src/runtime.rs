mod operation;

use std::rc::Weak;

use crate::{
    FuncType,
    Byte,
    Name,
    Mut,
    Func,
    Instr,
};

enum Val {
    I32Const(i32),
    I64Const(i64),
    F32Const(f32),
    F64Const(f64),
}

enum Result {
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
type FuncAddr = Addr;
type TableAddr = Addr;
type MemAddr = Addr;
type GlobalAddr = Addr;

#[derive(Default)]
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

struct ExportInst {
    name: Name,
    value: ExternVal,
}

pub enum ExternVal {
    Func(FuncAddr),
    Table(TableAddr),
    Mem(MemAddr),
    Global(GlobalAddr),
}

struct Stack(pub Vec<StackEntry>);

enum StackEntry {
    Value(Val),
    Label(u32, Vec<Instr>),
    Activation(u32, Frame),
}

#[derive(Default)]
pub struct Frame {
    locals: Vec<Val>,
    pub module: ModuleInst,
}

pub struct Thread {
    pub frame: Frame,
    pub instrs: Vec<Instr>,
}