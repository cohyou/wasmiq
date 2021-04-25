use crate::{
    Store,
    Module,
    ExternVal,
    ModuleInst,
    Error,
    Frame,

    FuncInst,
    ExternType,
    Thread,
    StackEntry,
    Val,

    FuncAddr,
    GlobalType,
    GlobalAddr,
    GlobalInst,
    Func,
    FuncType,
    TableInst,
    Limits,
    TableType,
    TableAddr,
    MemAddr,
    MemType,
    MemInst,
    Export,
    ExportDesc,
    ExportInst,
    ElemType,
    ValType,
    Expr,
    ExecResult,
};

pub fn module_instanciate(store: &mut Store, module: Module, externvals: Vec<ExternVal>) -> Result<ModuleInst, Error> {
    let (frame, result) = module.instanciate(store, externvals);
    match result {
        ExecResult::Vals(_) => Ok(frame.module),
        ExecResult::Trap(err) => Err(err),
    }
}

impl Module {
    fn instanciate(&self, store: &mut Store, externvals: Vec<ExternVal>) -> (Frame, ExecResult) {
        let frame_default = Frame::default();
        let trap = ExecResult::Trap(Error::Invalid("Module::instanciate".to_owned()));

        let externtypes = match self.validate() {
            Err(err) => {
                let trap = ExecResult::Trap(err);
                return (frame_default, trap);
            },
            Ok(externtypes) => externtypes,
        };
        let externtypes_imp = externtypes.0;
        if externtypes_imp.len() != externvals.len() { 
            let trap = ExecResult::Trap(Error::Invalid("Module::instanciate externtypes_imp.len() != externvals.len()".to_owned()));
            return (frame_default, trap);
        }
        let mut globaladdrs = vec![];
        for (ext_val, ext_type) in externvals.iter().zip(externtypes_imp) {
            match ext_val {
                ExternVal::Func(funcaddr) => {
                    let functype = match store.funcs.get(funcaddr.clone()) {
                        None => return (frame_default, trap),
                        Some(FuncInst::User(funcinst)) => funcinst.tp.clone(),
                        Some(FuncInst::Host(funcinst)) => funcinst.tp.clone(),
                    };
                    if let ExternType::Func(ft) = ext_type {
                        if Module::match_functype(functype, ft) {
                            let trap = ExecResult::Trap(Error::Invalid("Module::instanciate Module::match_functype(functype, ft)".to_owned()));
                            return (frame_default, trap);
                        }
                    } else {
                        let trap = ExecResult::Trap(Error::Invalid("Module::instanciate ExternType::Func(ft) = ext_type".to_owned()));
                        return (frame_default, trap);
                    }
                },
                ExternVal::Table(tableaddr) => {
                    let tabletype = 
                    if let Some(tabletype) = find_tabletype(store, tableaddr.clone()) {
                        tabletype
                    } else {
                        let trap = ExecResult::Trap(Error::Invalid("Module::instanciate Some(tabletype) = find_tabletype(store, tableaddr.clone())".to_owned()));
                        return (frame_default, trap);
                    };
                    if let ExternType::Table(tt) = ext_type {
                        if Module::match_tabletype(tabletype, tt) {
                            let trap = ExecResult::Trap(Error::Invalid("Module::instanciate Module::match_tabletype(tabletype, tt)".to_owned()));
                            return (frame_default, trap);
                        }
                    } else {
                        let trap = ExecResult::Trap(Error::Invalid("Module::instanciate ExternType::Table(tt) = ext_type".to_owned()));
                        return (frame_default, trap);
                    }
                },
                ExternVal::Mem(memaddr) => {
                    let memtype = 
                    if let Some(memtype) = find_memtype(store, memaddr.clone()) {
                        memtype
                    } else {
                        let trap = ExecResult::Trap(Error::Invalid("Module::instanciate Some(memtype) = find_memtype(store, memaddr.clone())".to_owned()));
                        return (frame_default, trap);
                    };
                    if let ExternType::Mem(mt) = ext_type {
                        if Module::match_memtype(memtype, mt) {
                            let trap = ExecResult::Trap(Error::Invalid("Module::instanciate Module::match_memtype(memtype, mt)".to_owned()));
                            return (frame_default, trap);
                        }
                    } else {
                        let trap = ExecResult::Trap(Error::Invalid("Module::instanciate ExternType::Mem(mt) = ext_type".to_owned()));
                        return (frame_default, trap);
                    }
                },
                ExternVal::Global(globaladdr) => {
                    let globaltype = 
                    if let Some(globaltype) = find_globaltype(store, globaladdr.clone()) {
                        globaltype
                    } else {
                        let trap = ExecResult::Trap(Error::Invalid("Module::instanciate Some(globaltype) = find_globaltype(store, globaladdr.clone())".to_owned()));
                        return (frame_default, trap);
                    };
                    if let ExternType::Global(gt) = ext_type {
                        if Module::match_globaltype(globaltype, gt) {
                            let trap = ExecResult::Trap(Error::Invalid("Module::instanciate Module::match_globaltype(globaltype, gt)".to_owned()));
                            return (frame_default, trap);
                        }
                    } else {
                        let trap = ExecResult::Trap(Error::Invalid("Module::instanciate ExternType::Global(gt) = ext_type".to_owned()));
                        return (frame_default, trap);
                    }
                    globaladdrs.push(globaladdr.clone());
                },
            }
        }

        let mut thread = Thread::new(store);

        let mut moduleinst_g = ModuleInst::default();
        moduleinst_g.globaladdrs = globaladdrs;
        let frame_g = Frame { module: moduleinst_g, locals: vec![] };
        thread.stack.push(StackEntry::Activation(0, frame_g));
        let mut vals = vec![];
        for global in &self.globals {
            vals.push(Self::evaluate_expr(thread.store, global.init.clone()));
        }
        thread.stack.pop();

        let moduleinst = self.alloc_module(thread.store, vec![], vals);
        let tableaddrs = moduleinst.tableaddrs.clone();
        let memaddrs = moduleinst.memaddrs.clone();
        let frame = Frame { module: moduleinst, locals: vec![] };
        thread.stack.push(StackEntry::Activation(0, frame));

        let mut init_elem_list = vec![]; 
        for elem in &self.elem {
            let eo = if let Val::I32Const(eo) = Self::evaluate_expr(thread.store, elem.offset.clone()) {
                eo
            } else {
                let trap = ExecResult::Trap(Error::Invalid("Module::instanciate Val::I32Const(eo) = Self::evaluate_expr(thread.store, elem.offset.clone())".to_owned()));
                return (frame_default, trap);
            };
            let tableidx = elem.table;
            let tableaddr = tableaddrs[tableidx as usize];
            let tableinst = &thread.store.tables[tableaddr];
            let eend = eo as usize + elem.init.len();

            if eend > tableinst.elem.len() {
                let trap = ExecResult::Trap(Error::Invalid("Module::instanciate eend > tableinst.elem.len()".to_owned()));
                return (frame_default, trap);
            }
            init_elem_list.push(eo);
        }

        let mut init_data_list = vec![]; 
        for data in &self.data {
            let data_o = if let Val::I32Const(data_o) = Self::evaluate_expr(thread.store, data.offset.clone()) {
                data_o
            } else {
                let trap = ExecResult::Trap(Error::Invalid("Module::instanciate Val::I32Const(data_o) = Self::evaluate_expr(thread.store, data.offset.clone())".to_owned()));
                return (frame_default, trap);
            };
            let memidx = data.data;
            let memaddr = memaddrs[memidx as usize];
            let meminst = &thread.store.mems[memaddr];
            let dend = data_o as usize + data.init.len();

            if dend > meminst.data.len() {
                let trap = ExecResult::Trap(Error::Invalid("Module::instanciate dend > meminst.data.len()".to_owned()));
                return (frame_default, trap);
            }
            init_data_list.push(data_o);
        }

        let frame = if let Some(StackEntry::Activation(0, frame)) = thread.stack.pop() {
            frame
        } else {
            let trap = ExecResult::Trap(Error::Invalid("Module::instanciate Some(StackEntry::Activation(0, frame)) = thread.stack.pop()".to_owned()));
            return (frame_default, trap);
        };
        for (elem, eo) in self.elem.iter().zip(init_elem_list) {
            for (j, funcidx) in elem.init.iter().enumerate() {
                let funcaddr = frame.module.funcaddrs[funcidx.clone() as usize];
                let tableidx = elem.table;
                let tableaddr = tableaddrs[tableidx as usize];
                let tableinst = &mut thread.store.tables[tableaddr];
                tableinst.elem[eo as usize + j] = Some(funcaddr);
            }
        }
        for (data, data_o) in self.data.iter().zip(init_data_list) {
            let memidx = data.data;
            let memaddr = memaddrs[memidx as usize];
            let meminst = &mut thread.store.mems[memaddr];
            for (j, byte) in data.init.iter().enumerate() {
                meminst.data[data_o as usize + j] = byte.clone();
            }
        }

        if let Some(start) = &self.start {
            let funcaddr = frame.module.funcaddrs[start.0 as usize];
            let mut thread = Thread::new(store);
            thread.execute_invoke(&funcaddr);
        }

        (frame, ExecResult::Vals(vec![]))
    }

    pub fn invoke(store: &mut Store, funcaddr: FuncAddr, vals: Vec<Val>) -> ExecResult {
        let funcinst = if let Some(funcinst) = store.funcs.get(funcaddr) {
            funcinst
        } else {
            return ExecResult::Trap(Error::Invalid("Module::invoke store.funcs.get(funcaddr) is None".to_owned()));
        };
        let (argtypes, returntypes) = match funcinst {
            FuncInst::User(user) => user.tp.clone(),
            FuncInst::Host(host) => host.tp.clone(),
        };
        if vals.len() != argtypes.len() {
            return ExecResult::Trap(Error::Invalid("Module::invoke vals.len() != argtypes.len()".to_owned()));
        }
        for (argtype, val) in argtypes.iter().zip(vals.clone()) {
            let matches = match val {
                Val::I32Const(_) => argtype == &ValType::I32,
                Val::I64Const(_) => argtype == &ValType::I64,
                Val::F32Const(_) => argtype == &ValType::F32,
                Val::F64Const(_) => argtype == &ValType::F64,
            };
            if !matches { return ExecResult::Trap(Error::Invalid("Module::invoke !matches".to_owned())); }
        }

        let dummy_frame = Frame{ module: ModuleInst::default(), locals: vec![] };
        let mut thread = Thread::new(store);
        thread.stack.push(StackEntry::Activation(0, dummy_frame.clone()));
        let vals: Vec<StackEntry> = vals.clone().iter().map(|v| StackEntry::Value(v.clone())).collect();
        thread.stack.extend(vals);

        let mut thread = Thread::new(store);
        thread.execute_invoke(&funcaddr);

        let mut returnvals = vec![];
        for _ in 0..returntypes.len() {
            if let Some(StackEntry::Value(v)) = thread.stack.pop() {
                returnvals.push(v);
            }
        }

        ExecResult::Vals(returnvals)
    }

    fn alloc_module(&self, store: &mut Store, externvals: Vec<ExternVal>, vals: Vec<Val>) -> ModuleInst {
        let mut moduleinst = ModuleInst::default();
        moduleinst.types = self.types.clone();

        // fn hostfunc() {}
        // alloc_hostfunc(store, (vec![], vec![]), hostfunc);

        let mut funcaddrs = vec![];
        for func in &self.funcs {
            let funcaddr = alloc_func(store, func, &moduleinst);
            funcaddrs.push(funcaddr);
        }

        let mut tableaddrs = vec![];
        for table in &self.tables {
            let tableaddr = alloc_table(store, table.0.clone());
            tableaddrs.push(tableaddr);
        }

        let mut memaddrs = vec![];
        for mem in &self.mems {
            let memaddr = alloc_mem(store, mem.0.clone());
            memaddrs.push(memaddr);
        }

        let mut globaladdrs = vec![];
        for (i, global) in self.globals.iter().enumerate() {
            let globaladdr = alloc_global(store, global.tp.clone(), vals[i]);
            globaladdrs.push(globaladdr);
        }


        let mut funcaddrs_ext = vec![];
        let mut tableaddrs_ext = vec![];
        let mut memaddrs_ext = vec![];
        let mut globaladdrs_ext = vec![];
        for externval in externvals {
            match externval {
                ExternVal::Func(func) => funcaddrs_ext.push(func),
                ExternVal::Global(global) => globaladdrs_ext.push(global),
                ExternVal::Mem(mem) => memaddrs_ext.push(mem),
                ExternVal::Table(table) => tableaddrs_ext.push(table),
            }
        }

        funcaddrs_ext.extend(funcaddrs);
        tableaddrs_ext.extend(tableaddrs);
        memaddrs_ext.extend(memaddrs);
        globaladdrs_ext.extend(globaladdrs);

        
        let mut exportinsts = vec![];
        for export in &self.exports {
            let Export{ name: _, desc} = export;
            let externval = match desc {
                ExportDesc::Func(funcidx) => ExternVal::Func(funcaddrs_ext[funcidx.clone() as usize]),
                ExportDesc::Table(tableidx) => ExternVal::Table(tableaddrs_ext[tableidx.clone() as usize]),
                ExportDesc::Mem(memidx) => ExternVal::Mem(memaddrs_ext[memidx.clone() as usize]),
                ExportDesc::Global(globalidx) => ExternVal::Global(globaladdrs_ext[globalidx.clone() as usize]),
            };
            let exportinst = ExportInst{ name: export.name.clone(), value: externval };
            exportinsts.push(exportinst);
        }

        moduleinst.types = self.types.clone();
        moduleinst.funcaddrs = funcaddrs_ext;
        moduleinst.tableaddrs = tableaddrs_ext;
        moduleinst.memaddrs = memaddrs_ext;
        moduleinst.globaladdrs = globaladdrs_ext;
        moduleinst.exports = exportinsts;

        moduleinst
    }

    fn evaluate_expr(store: &mut Store, expr: Expr) -> Val {
        let mut thread = Thread::new(store);
        thread.spawn(&mut expr.0.clone());
        if let Some(StackEntry::Value(val)) = thread.stack.pop() {
            val
        } else {
            panic!("evaluate_offset");
        }
    }

    fn match_functype(ft1: FuncType, ft2: FuncType) -> bool {
        ft1 != ft2
    }

    fn match_limits(limits1: Limits, limits2: Limits) -> bool {
        let Limits{min: n1, max: m1} = limits1;
        let Limits{min: n2, max: m2} = limits2;
        if n1 >= n2 {
            if let Some(m2) = m2 {
                if let Some(m1) = m1 {
                    m1 <= m2
                } else {
                    false
                }
            } else {
                true
            }
        } else {
            false
        }
    }

    fn match_tabletype(tt1: TableType, tt2: TableType) -> bool {
        let TableType(limits1, elemtype1) = tt1;
        let TableType(limits2, elemtype2) = tt2;
        Module::match_limits(limits1, limits2) && elemtype1 == elemtype2
    }

    fn match_memtype(MemType(limits1): MemType, MemType(limits2): MemType) -> bool {
        Module::match_limits(limits1, limits2)
    }

    fn match_globaltype(gt1: GlobalType, gt2: GlobalType) -> bool {
        gt1 != gt2
    }
}

fn alloc_func(store: &mut Store, func: &Func, moduleinst: &ModuleInst) -> FuncAddr {
    let addr = store.funcs.len();
    let functype = &moduleinst.types[func.tp as usize];
    let funcinst = FuncInst::user(functype.clone(), moduleinst.clone(), func.clone());
    store.funcs.push(funcinst);
    addr
}

pub fn alloc_hostfunc(store: &mut Store, functype: FuncType, hostfunc: fn()) -> FuncAddr {
    let addr = store.funcs.len();
    let funcinst = FuncInst::host(functype, hostfunc);
    store.funcs.push(funcinst);
    addr
}

pub fn alloc_table<'a>(store: &'a mut Store, tabletype: TableType) -> TableAddr {
    let addr = store.tables.len();
    let TableType(Limits{ min: n, max: m }, _) = tabletype;
    let mut elem = vec![];
    for _ in 0..n { elem.push(None) }
    let tableinst = TableInst{ elem: elem, max: m };
    store.tables.push(tableinst);
    addr
}

pub fn alloc_mem<'a>(store: &'a mut Store, memtype: MemType) -> MemAddr {
    let addr = store.mems.len();
    let MemType(Limits{ min: n, max: m }) = memtype;
    let data = Vec::with_capacity((n * 64) as usize);
    let meminst = MemInst{ data: data, max: m };
    store.mems.push(meminst);
    addr
}

pub fn alloc_global<'a>(store: &'a mut Store, globaltype: GlobalType, val: Val) -> GlobalAddr {
    let addr = store.globals.len();
    let globalinst = GlobalInst{ value: val, mutability: globaltype.1 };
    store.globals.push(globalinst);
    addr
}

pub fn find_tabletype(store: &Store, tableaddr: TableAddr) -> Option<TableType> {
    match store.tables.get(tableaddr.clone()) {
        None => None,
        Some(TableInst{elem, max: m}) => {
            Some(TableType(Limits{min: elem.len() as u32, max: m.clone()}, ElemType::FuncRef))
        }
    }
}

pub fn grow_table(tableinst: &mut TableInst, n: usize) -> std::result::Result<(), Error> {
    let len = n + (tableinst.elem.len() / (64*1024));
    if len > 2usize.pow(32) {
        return Err(Error::Invalid("grow_table len > 2usize.pow(32)".to_owned()));
    }
    if let Some(mx) = tableinst.max {
        if (mx as usize) < len {
            return Err(Error::Invalid("grow_table (mx as usize) < len".to_owned()));
        }
    }
    for _ in 0..n {
        tableinst.elem.push(None);    
    }

    Ok(())
}

pub fn find_memtype(store: &Store, memaddr: TableAddr) -> Option<MemType> {
    match store.mems.get(memaddr.clone()) {
        None => None,
        Some(MemInst{data, max}) => {
            Some(MemType(Limits{min: (data.len()/64) as u32, max: max.clone()}))
        }
    }
}

pub fn grow_mem(meminst: &mut MemInst, n: usize) -> std::result::Result<(), Error> {
    let len = n + (meminst.data.len() / (64*1024));
    if len > 2usize.pow(16) {
        return Err(Error::Invalid("grow_mem len > 2usize.pow(16)".to_owned()));
    }
    if let Some(mx) = meminst.max {
        if (mx as usize) < len {
            return Err(Error::Invalid("grow_mem (mx as usize) < len".to_owned()));
        }
    }
    for _ in 0..n {
        let page = [0x00;64*1024];
        meminst.data.extend(Vec::from(page));    
    }

    Ok(())
}

pub fn find_globaltype(store: &Store, globaladdr: GlobalAddr) -> Option<GlobalType> {
    match store.globals.get(globaladdr.clone()) {
        None => None,
        Some(GlobalInst{value: val, mutability: mt}) => {
            let vt = match val {
                Val::I32Const(_) => ValType::I32,
                Val::I64Const(_) => ValType::I64,
                Val::F32Const(_) => ValType::F32,
                Val::F64Const(_) => ValType::F64,
            };
            Some(GlobalType(vt, mt.clone()))
        },
    }
}