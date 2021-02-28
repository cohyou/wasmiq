#[derive(Clone, Copy)]
pub enum ValType {
    I32, I64, F32, F64,
}

pub type ResultType = Vec<ValType>;

pub type FuncType = (ResultType, ResultType);

#[derive(Clone)]
pub struct Limits {
    min: u32,
    max: Option<u32>,
}

#[derive(Clone)]
pub struct MemType(Limits);

#[derive(Clone)]
pub struct TableType(Limits, ElemType);

impl TableType {
    pub fn is_funcref(&self) -> bool { true }
}

#[derive(Clone)]
enum ElemType { FuncRef, }

#[derive(Clone)]
pub struct GlobalType(pub ValType, Mut);

impl GlobalType {
    pub fn is_var(&self) -> bool { self.1 == Mut::Var }
}

#[derive(Clone, PartialEq)]
enum Mut { Const, Var }

pub enum ExternType {
    Func(FuncType),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}