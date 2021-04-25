use crate::{
    Mut,
    Error,
    Context,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum ValType {
    I32, I64, F32, F64,
}

pub type ResultType = Vec<ValType>;

pub type FuncType = (ResultType, ResultType);

#[derive(Clone, Default, PartialEq, Debug)]
pub struct Limits {
    pub min: u32,
    pub max: Option<u32>,
}

impl Limits {
    pub fn validate(&self, _context: &Context, value: usize) -> Result<usize, Error> {
        if let Some(max) = self.max {
            if max < self.min { return Err(Error::Invalid("Limits::validate max < self.min".to_owned())); }
        }
        Ok(value)
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MemType(pub Limits);

impl MemType {
    pub fn validate(&self, context: &Context) -> Result<(), Error> {
        let _ = self.0.validate(context, u16::MAX as usize)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct TableType(pub Limits, pub ElemType);

impl TableType {
    pub fn is_funcref(&self) -> bool { true }
    pub fn validate(&self, context: &Context) -> Result<(), Error> {
        let _ = self.0.validate(context, u32::MAX as usize)?;
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum ElemType { FuncRef, }

#[derive(Clone, PartialEq, Debug)]
pub struct GlobalType(pub ValType, pub Mut);

impl GlobalType {
    pub fn is_var(&self) -> bool { self.1 == Mut::Var }
}

#[derive(Debug)]
pub enum ExternType {
    Func(FuncType),
    Table(TableType),
    Mem(MemType),
    Global(GlobalType),
}

impl ExternType {
    pub fn validate(&self, context: &Context) -> Result<(), Error> {
        match &self {
            ExternType::Func(_) => {},
            ExternType::Table(tabletype) => {
                tabletype.validate(context)?;
            },
            ExternType::Mem(memtype) => {
                memtype.validate(context)?;
            },
            ExternType::Global(_) => {},
        }
        
        Ok(())
    }
}