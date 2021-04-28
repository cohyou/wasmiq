use crate::{
    ValType,
    ResultType,
    FuncType,
    ElemType,
    TableType,
    MemType,
    GlobalType,
    ExternType,
    Name,
    Error,
};
use super::{
    Module,
    
    Func,
    Table,
    Data,
    Mem,
    Global,
    Elem,
    Start,
    ExportDesc,
    ImportDesc,
    
    TypeIdx,
    FuncIdx,
    GlobalIdx,
    LocalIdx,
    LabelIdx,
};

#[derive(Clone, Default, Debug)]
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

    pub fn clone_with_labels(&self, vts: Vec<ValType>) -> Context {
        let mut context = self.clone();
        let labels = {
            let mut new_labels = vec![vts];
            if let Some(labels) = &self.labels {
                new_labels.extend(labels.clone());
            }
            new_labels
        };
        context.labels = Some(labels);
        context
    }
}

impl Module {
    pub fn validate(&self) -> Result<(Vec<ExternType>, Vec<ExternType>), Error> {
        let mut context = Context {
            types: self.types.clone(),
            funcs: Some(vec![]),
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

            let mut context_func = context.clone();
            for f in &self.funcs {
                let functype = f.validate(&context_func)?;
                if let Some(funcs) = &context_func.funcs {
                    let mut new_funcs = funcs.clone();
                    new_funcs.push(functype);
                    context_func.funcs = Some(new_funcs);
                }
            }
            funcs = context_func.funcs.unwrap();

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

        let mut context_g = context.clone();
        context_g.globals = Some(globals);

        // functype is always valid
        // for tp in &self.types {
        //     tp.validate()
        // }

        for func in &self.funcs {
            func.validate(&context)?;
        }

        for table in &self.tables {
            table.validate(&context)?;
        }

        for mem in &self.mems {
            mem.validate(&context)?;
        }

        for global in &self.globals {
            global.validate(&context_g)?;
        }

        for el in &self.elem {
            el.validate(&context)?;
        }

        for dt in &self.data {
            dt.validate(&context)?;
        }

        if let Some(start) = &self.start {
            start.validate(&context)?;
        
        }

        let mut its = vec![];
        for imp in &self.imports {
            let externtype = imp.desc.validate(&context)?;
            its.push(externtype);
        }

        let mut ets = vec![];
        for exp in &self.exports {
            let externtype = exp.desc.validate(&context)?;
            ets.push(externtype);
        }

        if let Some(tables) = context.tables {
            if tables.len() > 1 { return Err(Error::Invalid("Module::validate tables.len() > 1".to_owned())); }
        }

        if let Some(mems) = context.mems {
            if mems.len() > 1 { return Err(Error::Invalid("Module::validate mems.len() > 1".to_owned())); }
        }

        let names = &self.exports.iter()
            .map(|exp| exp.name.clone()).collect::<Vec<Name>>();
        let mut names = names.clone();
        names.dedup();
        if &names.len() < &self.exports.len() {
            return Err(Error::Invalid("Module::validate &names.len() < &self.exports.len()".to_owned()));
        } 

        Ok((its, ets))
    }
}

use crate::instr::{
    // vt, 
    vt_rev,
};

impl Func {
    fn validate(&self, context: &Context) -> Result<FuncType, Error> {
        let functype = context.tp(self.tp.clone())
            .ok_or(Error::OutOfIndex(format!("func validate: self.tp")))?;
        let mut new_context = context.clone();
        let mut new_locals = functype.0.clone();
        
        new_locals.extend(self.locals.clone());
        
        new_context.locals = Some(new_locals);
        new_context.labels = Some(vec![functype.1.clone()]);
        new_context.rtn = Some(functype.1.clone());

        let expr_type = self.body.validate(&new_context)?;
        let vts: Vec<ValType> = expr_type.0.iter().map(|v| vt_rev(v)).collect();
        if vts != functype.1 {
            return Err(Error::Invalid(format!("Func{:?} has return type {:?} but {:?} occured", self.tp, vts, functype.1)));
        }

        Ok(functype)
    }
}

impl Table {
    fn validate(&self, context: &Context) -> Result<TableType, Error> {
        self.0.validate(context)?;
        Ok(self.0.clone())
    }
}

impl Mem {
    fn validate(&self, context: &Context) -> Result<MemType, Error> {
        self.0.validate(context)?;
        Ok(self.0.clone())
    }
}

impl Global {
    fn validate(&self, context: &Context) -> Result<GlobalType, Error> {
        let rt = self.init.validate(context)?;
        let vts: Vec<ValType> = rt.0.iter().map(|v| vt_rev(v)).collect();
        if vts != vec![self.tp.0] { return Err(Error::Invalid("Global::validate vts != vec![self.tp.0]".to_owned())); }
        if !self.init.is_constant(context) { return Err(Error::Invalid("Global::validate !self.init.is_constant(context)".to_owned())); } 
        Ok(self.tp.clone())
    }
}

impl Elem {
    fn validate(&self, context: &Context) -> Result<(), Error> {
        if self.table != 0 { return Err(Error::Invalid("Elem::validate self.table != 0".to_owned())); } 
        let TableType(_limits, elemtype) = context.table().ok_or(Error::Invalid("Elem::validate context.table()".to_owned()))?;

        if elemtype != ElemType::FuncRef { return Err(Error::Invalid("Elem::validate elemtype != ElemType::FuncRef".to_owned())); }

        let resulttype = self.offset.validate(context)?;
        let vts: Vec<ValType> = resulttype.0.iter().map(|v| vt_rev(v)).collect();
        if vts != vec![ValType::I32] {
            return Err(Error::Invalid("Elem::validate vts != vec![ValType::I32]".to_owned()));
        }

        if !self.offset.is_constant(context) {
            return Err(Error::Invalid("Elem::validate !self.offset.is_constant(context)".to_owned()));
        }

        for y in &self.init {
            if context.func(y.clone()).is_none() {
                return Err(Error::Invalid("Elem::validate context.func(y.clone()).is_none()".to_owned()));
            }
        }

        Ok(())
    }
}

impl Data {
    fn validate(&self, context: &Context) -> Result<(), Error> {
        if self.data != 0 { return Err(Error::Invalid("Data::validate self.data != 0".to_owned())); }

        let resulttype = self.offset.validate(context)?;
        let vts: Vec<ValType> = resulttype.0.iter().map(|v| vt_rev(v)).collect();
        if vts != vec![ValType::I32] {
            return Err(Error::Invalid("Data::validate vts != vec![ValType::I32]".to_owned()));
        }

        if !self.offset.is_constant(context) {
            return Err(Error::Invalid("Data::validate !self.offset.is_constant(context)".to_owned()));
        }

        Ok(())
    }
}

impl Start {
    fn validate(&self, context: &Context) -> Result<(), Error> {
        let functype = context.func(self.0).ok_or(Error::Invalid("Start::validate context.func(self.0)".to_owned()))?;
        if functype.0.len() > 0 || functype.1.len() > 0 {
            return Err(Error::Invalid("Start::validate functype.0.len() > 0 || functype.1.len() > 0".to_owned()));
        }
        Ok(())
    }
}

impl ExportDesc {
    fn validate(&self, context: &Context) -> Result<ExternType, Error> {
        match &self {
            ExportDesc::Func(x) => {
                let functype = context.func(x.clone())
                    .ok_or(Error::OutOfIndex(format!("ExportDesc::validate: funcidx")))?;
                Ok(ExternType::Func(functype.clone()))
            },
            ExportDesc::Table(x) => {
                if x != &0 { return Err(Error::Invalid("ExportDesc::validate Table　x != &0".to_owned())); }
                let tabletype = context.table().unwrap();
                Ok(ExternType::Table(tabletype.clone()))
            },
            ExportDesc::Mem(x) => {
                if x != &0 { return Err(Error::Invalid("ExportDesc::validate Mem x != &0".to_owned())); }
                let memtype = context.mem().unwrap();
                Ok(ExternType::Mem(memtype.clone()))
            },
            ExportDesc::Global(x) => {
                let globaltype = context.global(x.clone()).ok_or(Error::Invalid("ExportDesc::validate context.global(x.clone())".to_owned()))?;
                Ok(ExternType::Global(globaltype.clone()))
            },
        }
    }
}

impl ImportDesc {
    fn validate(&self, context: &Context) -> Result<ExternType, Error> {
        match &self {
            ImportDesc::Func(x) => {
                let tp = context.tp(x.clone())
                    .ok_or(Error::OutOfIndex(format!("importdesc validate: typeidx")))?;
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