use crate::{
    ValType,
    ResultType,
    FuncType,
    TableType,
    MemType,
    GlobalType,
    ExternType,
    // Instr,
    Error,
};
use super::{
    Module,
    ImportDesc,
    Func,
    Table,
    Mem,
    Global,
    TypeIdx,
    FuncIdx,
    GlobalIdx,
    LocalIdx,
    LabelIdx,
};

pub struct Context {
    types: Vec<FuncType>,
    funcs: Option<Vec<FuncType>>,
    tables: Option<Vec<TableType>>,
    mems: Option<Vec<MemType>>,
    globals: Option<Vec<GlobalType>>,
    locals: Option<Vec<ValType>>,
    labels: Option<Vec<ResultType>>,
    rtn: Option<ResultType>,
}

impl Context {
    pub fn tp(&self, idx: TypeIdx) -> Option<FuncType> {
        self.types.get(idx.clone() as usize).cloned()
    }

    pub fn global(&self, idx: GlobalIdx) -> Option<GlobalType> {
        self.globals.as_ref().and_then(|globaltps| {
            globaltps.get(idx.clone() as usize).cloned()
        })
    }

    pub fn local(&self, idx: LocalIdx) -> Option<ValType> {
        self.locals.as_ref().and_then(|valtps| {
            valtps.get(idx.clone() as usize).cloned()
        })
    }

    pub fn table(&self) -> Option<TableType> {
        self.tables.as_ref().and_then(|tabletps| {
            tabletps.get(0).cloned()
        })
    }

    pub fn mem(&self) -> Option<MemType> {
        self.mems.as_ref().and_then(|valtps| {
            valtps.get(0).cloned()
        })
    }

    pub fn label(&self, idx: LabelIdx) -> Option<ResultType> {
        self.labels.as_ref().and_then(|restps| {
            restps.get(idx.clone() as usize).cloned()
        })
    }

    pub fn rtn(&self) -> Option<ResultType> {
        self.rtn.clone()
    }

    pub fn func(&self, idx: FuncIdx) -> Option<FuncType> {
        self.funcs.as_ref().and_then(|restps| {
            restps.get(idx.clone() as usize).cloned()
        })
    }
}
impl Module {
    pub fn validate(&self) -> Result<(), Error> {
        let mut context = Context {
            types: self.types.clone(),
            funcs: None,
            tables: None,
            mems: None,
            globals: None,
            locals: None,
            labels: None,
            rtn: None,
        };

        let (funcs, tables, mems, globals) = {
            let mut funcs: Vec<FuncType> = vec![];
            let mut tables: Vec<TableType> = vec![];
            let mut mems: Vec<MemType> = vec![];
            let mut globals: Vec<GlobalType> = vec![];

            for imp in self.imports.iter().map(|imp| imp.desc.validate(&context)) {
                let imp = imp?;
                match imp {
                    ExternType::Func(functype) => { funcs.push(functype); },
                    ExternType::Table(tabletype) => { tables.push(tabletype); },
                    ExternType::Mem(memtype) => { mems.push(memtype); },
                    ExternType::Global(globaltype) => { globals.push(globaltype); },
                }
            }
    
            for functype in self.funcs.iter().map(|f| f.validate(&context)) {
                let functype = functype?;
                funcs.push(functype);
            }

            for tabletype in self.tables.iter().map(|t| t.validate(&context)) {
                let tabletype = tabletype?;
                tables.push(tabletype);
            }

            for memtype in self.mems.iter().map(|t| t.validate(&context)) {
                let memtype = memtype?;
                mems.push(memtype);
            }

            for globaltype in self.globals.iter().map(|t| t.validate(&context)) {
                let globaltype = globaltype?;
                globals.push(globaltype);
            }

            (funcs, tables, mems, globals)
        };

        context.funcs = Some(funcs);
        context.tables = Some(tables);
        context.mems = Some(mems);
        context.globals = Some(globals);

        Err(Error::Invalid)
    }
}

impl ImportDesc {
    fn validate(&self, context: &Context) -> Result<ExternType, Error> {
        match &self {
            ImportDesc::Func(x) => {
                let tp = context.types.get(x.clone() as usize)
                    .ok_or(Error::OutOfIndex("importdesc validate: typeidx".to_string()))?;
                Ok(ExternType::Func(tp.clone()))
            },
            ImportDesc::Table(tabletype) => {
                Ok(ExternType::Table(tabletype.clone()))
            },
            ImportDesc::Mem(memtype) => {
                Ok(ExternType::Mem(memtype.clone()))
            },
            ImportDesc::Global(globaltype) => {
                Ok(ExternType::Global(globaltype.clone()))
            },
        }
    }
}

impl Func {
    fn validate(&self, context: &Context) -> Result<FuncType, Error> {
        unimplemented!()
    }
}

impl Table {
    fn validate(&self, context: &Context) -> Result<TableType, Error> {
        unimplemented!()
    }
}

impl Mem {
    fn validate(&self, context: &Context) -> Result<MemType, Error> {
        unimplemented!()
    }
}

impl Global {
    fn validate(&self, context: &Context) -> Result<GlobalType, Error> {
        unimplemented!()
    }
}