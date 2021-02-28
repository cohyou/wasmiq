use crate::{
    ValType,
    ResultType,
    FuncType,
    TableType,
    GlobalType,
    TypeIdx,
    GlobalIdx,
    LocalIdx,
    LabelIdx,
    Error,
    Context,
    MemArg,
};
use super::{
    Instr,
    ValSize,
    CvtOp,
};

macro_rules! instr_tp {
    ($res: ident) => {
        Ok((vec![], vec![ValType::$res]))
    };
    ($arg: ident -> $res: ident) => {
        Ok((vec![ValType::$arg], vec![ValType::$res]))
    };
    ($arg1: ident $arg2: ident -> $res: ident) => {
        Ok((vec![ValType::$arg1, ValType::$arg2], vec![ValType::$res]))
    };
    ($arg1: ident $arg2: ident $arg3: ident -> $res: ident) => {
        Ok((vec![ValType::$arg1, ValType::$arg2, ValType::$arg3], vec![ValType::$res]))
    };
    ($arg: ident ->) => {
        Ok((vec![ValType::$arg], vec![]))
    };
    ($arg1: ident $arg2: ident ->) => {
        Ok((vec![ValType::$arg1, ValType::$arg2], vec![]))
    };
    (() -> ()) => {
        Ok((vec![], vec![]))
    };
}

impl Instr {
    fn validate(&self, context: &Context) -> Result<FuncType, Error> {
        match &self {
            /*
            NUMERIC INSTRUCTIONS
            */

            /* t.const c */
            Instr::I32Const(_) => instr_tp!(I32),
            Instr::I64Const(_) => instr_tp!(I64),
            Instr::F32Const(_) => instr_tp!(F32),
            Instr::F64Const(_) => instr_tp!(F64),

            /* t.unop */
            Instr::IUnOp(valsize, _) => {
                match valsize {
                    ValSize::V32 => instr_tp!(I32 -> I32),
                    ValSize::V64 => instr_tp!(I64 -> I64),
                }
            },
            Instr::FUnOp(valsize, _) => {
                match valsize {
                    ValSize::V32 => instr_tp!(F32 -> F32),
                    ValSize::V64 => instr_tp!(F64 -> F64),
                }
            },
            // TODO: extendN_s

            /* t.binop */
            Instr::IBinOp(valsize, _) => {
                match valsize {
                    ValSize::V32 => instr_tp!(I32 I32 -> I32),
                    ValSize::V64 => instr_tp!(I64 I64 -> I64),
                }
            },
            Instr::FBinOp(valsize, _) => {
                match valsize {
                    ValSize::V32 => instr_tp!(F32 F32 -> F32),
                    ValSize::V64 => instr_tp!(F64 F64 -> F64),
                }
            },

            /* t.testop */
            Instr::ITestOp(valsize, _) => {
                match valsize {
                    ValSize::V32 => instr_tp!(I32 -> I32),
                    ValSize::V64 => instr_tp!(I64 -> I32),
                }
            },

            /* t.relop */
            Instr::IRelOp(valsize, _) => {
                match valsize {
                    ValSize::V32 => instr_tp!(I32 I32 -> I32),
                    ValSize::V64 => instr_tp!(I64 I64 -> I32),
                }
            },
            Instr::FRelOp(valsize, _) => {
                match valsize {
                    ValSize::V32 => instr_tp!(F32 F32 -> I32),
                    ValSize::V64 => instr_tp!(F64 F64 -> I32),
                }
            },

            /* t2.cvtop_t1_sx? */
            Instr::CvtOp(cvtop) => {
                match cvtop {
                    CvtOp::I32WrapFromI64 => instr_tp!(I64 -> I32),
                    CvtOp::I64ExtendFromI32(_) => instr_tp!(I32 -> I64),
                    CvtOp::ITruncFromF(valsize_i, valsize_f, _) => {
                        let arg_tp = match valsize_f {
                            ValSize::V32 => ValType::F32,
                            ValSize::V64 => ValType::F64,
                        };
                        let ret_tp = match valsize_i {
                            ValSize::V32 => ValType::I32,
                            ValSize::V64 => ValType::I64,
                        };               
                        Ok((vec![arg_tp], vec![ret_tp]))
                    },
                    CvtOp::F32DemoteFromF64 => instr_tp!(F64 -> F32),
                    CvtOp::F64PromoteFromF32 => instr_tp!(F32 -> F64),
                    CvtOp::FConvertFromI(valsize_f, valsize_i, _) => {
                        let arg_tp = match valsize_i {
                            ValSize::V32 => ValType::I32,
                            ValSize::V64 => ValType::I64,
                        }; 
                        let ret_tp = match valsize_f {
                            ValSize::V32 => ValType::F32,
                            ValSize::V64 => ValType::F64,
                        };
                        Ok((vec![arg_tp], vec![ret_tp]))
                    },
                    CvtOp::IReinterpretFromF(valsize) => {
                        match valsize {
                            ValSize::V32 => instr_tp!(F32 -> I32),
                            ValSize::V64 => instr_tp!(F64 -> I64),
                        }
                    },
                    CvtOp::FReinterpretFromI(valsize) => {
                        match valsize {
                            ValSize::V32 => instr_tp!(I32 -> F32),
                            ValSize::V64 => instr_tp!(I64 -> F64),
                        }
                    },
                }
            },
            // TODO: trunc_sat


            /*
            PARAMETRIC INSTRUCTIONS
            */

            // TODO: value-polymorphic
            // t ->
            Instr::Drop => instr_tp!(I32 ->),

            // TODO: value-polymorphic
            // t t i32 -> t
            Instr::Select => instr_tp!(I64 I64 I32 -> I64),


            /*
            VARIABLE INSTRUCTIONS
            */
            Instr::LocalGet(localidx) => {
                let tp = Instr::check_local(context, localidx, "local.get")?;
                Ok((vec![], vec![tp]))
            },
            Instr::LocalSet(localidx) => {
                let tp = Instr::check_local(context, localidx, "local.set")?;
                Ok((vec![tp], vec![]))
            },
            Instr::LocalTee(localidx) => {
                let tp = Instr::check_local(context, localidx, "local.tee")?;
                Ok((vec![tp], vec![tp]))
            },
            Instr::GlobalGet(globalidx) => {
                let globaltype = Instr::check_global(context, globalidx, "global.get")?;
                Ok((vec![], vec![globaltype.0]))
            },
            Instr::GlobalSet(globalidx) => {
                let globaltype = Instr::check_global(context, globalidx, "global.set")?;

                if globaltype.is_var() {
                    Ok((vec![globaltype.0], vec![]))
                } else {
                    let message = "instr global.set validate: can't set value to const global";
                    Err(Error::Mutability(message.to_string()))
                }
            },

            /*
            MEMORY INSTRUCTIONS
            */
            Instr::Load(valtype, memarg) => {
                let opname = "load";
                let _ = Instr::check_mem_exist(context, opname)?;
                let width = match valtype {
                    ValType::I32 | ValType::F32 => 32,
                    ValType::I64 | ValType::F64 => 64,
                };
                let _ = Instr::check_mem_alignment(opname, memarg, width)?;

                Ok((vec![ValType::I32], vec![valtype.clone()]))
            },
            Instr::ILoad8(valsize, _, memarg) => {
                let opname = "iload8";
                let _ = Instr::check_mem_exist(context, opname)?;
                let _ = Instr::check_mem_alignment(opname, memarg, 8)?;

                let valtype = match valsize {
                    ValSize::V32 => ValType::I32,
                    ValSize::V64 => ValType::I64,
                };

                Ok((vec![ValType::I32], vec![valtype.clone()]))
            },
            Instr::ILoad16(valsize, _, memarg) => {
                let opname = "iload16";
                let _ = Instr::check_mem_exist(context, opname)?;
                let _ = Instr::check_mem_alignment(opname, memarg, 16)?;

                let valtype = match valsize {
                    ValSize::V32 => ValType::I32,
                    ValSize::V64 => ValType::I64,
                };

                Ok((vec![ValType::I32], vec![valtype.clone()]))
            },
            Instr::I64Load32(_, memarg) => {
                let opname = "i64load32";
                let _ = Instr::check_mem_exist(context, opname)?;
                let _ = Instr::check_mem_alignment(opname, memarg, 32)?;

                instr_tp!(I32 -> I64)
            },
            Instr::Store(valtype, memarg) => {
                let opname = "store";
                let _ = Instr::check_mem_exist(context, opname)?;
                let width = match valtype {
                    ValType::I32 | ValType::F32 => 32,
                    ValType::I64 | ValType::F64 => 64,
                };
                let _ = Instr::check_mem_alignment(opname, memarg, width)?;

                Ok((vec![ValType::I32, valtype.clone()], vec![]))
            },
            Instr::IStore8(valsize, memarg) => {
                let opname = "istore8";
                let _ = Instr::check_mem_exist(context, opname)?;
                let _ = Instr::check_mem_alignment(opname, memarg, 8)?;

                let valtype = match valsize {
                    ValSize::V32 => ValType::I32,
                    ValSize::V64 => ValType::I64,
                };

                Ok((vec![ValType::I32, valtype.clone()], vec![]))
            },
            Instr::IStore16(valsize, memarg) => {
                let opname = "istore16";
                let _ = Instr::check_mem_exist(context, opname)?;
                let _ = Instr::check_mem_alignment(opname, memarg, 16)?;

                let valtype = match valsize {
                    ValSize::V32 => ValType::I32,
                    ValSize::V64 => ValType::I64,
                };

                Ok((vec![ValType::I32, valtype.clone()], vec![]))
            },
            Instr::I64Store32(memarg) => {
                let opname = "i64store32";
                let _ = Instr::check_mem_exist(context, opname)?;
                let _ = Instr::check_mem_alignment(opname, memarg, 32)?;

                instr_tp!(I32 I64 ->)
            },
            Instr::MemorySize => {
                let _ = Instr::check_mem_exist(context, "memorysize")?;
                instr_tp!(I32)
            },
            Instr::MemoryGrow => {
                let _ = Instr::check_mem_exist(context, "memorygrow")?;
                instr_tp!(I32 -> I32)
            },

            /*
            CONTROL INSTRUCTIONS
            */
            Instr::Nop => instr_tp!(() -> ()),
            // TODO: stack-polymorphic
            Instr::Unreachable => instr_tp!(() -> ()),
            // TODO: stack-polymorphic
            Instr::Br(labelidx) => {
                let label = Instr::check_label(context, labelidx, "br")?;
                unimplemented!()
            },
            Instr::BrIf(labelidx) => {
                let label = Instr::check_label(context, labelidx, "br")?;
                let mut args = label.clone();
                args.push(ValType::I32);
                Ok((args, label))
            },
            // TODO: stack-polymorphic
            Instr::BrTable(labelindices, labelidx) => unimplemented!(),
            // TODO: stack-polymorphic
            Instr::Return => {
                if let Some(rettp) = context.rtn() {
                    unimplemented!()
                } else {
                    Err(Error::PreCondition("instr return validate: context.return is absent".to_string()))
                }
            },
            Instr::Call(funcidx) => {
                if let Some(functype) = context.func(funcidx.clone()) {
                    Ok(functype.clone())
                } else {
                    Err(Error::OutOfIndex("instr call validate: funcidx".to_string()))
                }
            },
            Instr::CallIndirect(funcidx) => {
                let opname = "callindirect";
                let tabletype = Instr::check_table_exist(context, opname)?;
                if !tabletype.is_funcref() {
                    Err(Error::PreCondition(format!("instr {} validate: table.elemtype is not funcref", opname)))
                } else {
                    let tp = Instr::check_type(context, funcidx, opname)?;
                    let mut res = tp.clone();
                    res.0.push(ValType::I32);
    
                    Ok(res)
                }
            },


            _ => unimplemented!(),
        }
    }

    fn check_type(context: &Context, typeidx: &TypeIdx, opname: &str) -> Result<FuncType, Error> {
        let tp = context.tp(typeidx.clone())
            .ok_or(Error::OutOfIndex(format!("instr {} validate: typeidx", opname)))?;
        Ok(tp)
    }
    fn check_local(context: &Context, localidx: &LocalIdx, opname: &str) -> Result<ValType, Error> {
        let tp = context.local(localidx.clone())
            .ok_or(Error::OutOfIndex(format!("instr {} validate: localidx", opname)))?;
        Ok(tp)
    }

    fn check_global(context: &Context, globalidx: &GlobalIdx, opname: &str) -> Result<GlobalType, Error> {
        let globaltype = context.global(globalidx.clone())
            .ok_or(Error::OutOfIndex(format!("instr {} validate: globalidx", opname)))?;
        Ok(globaltype)
    }

    fn check_label(context: &Context, labelidx: &LabelIdx, opname: &str) -> Result<ResultType, Error> {
        let label = context.label(labelidx.clone())
            .ok_or(Error::OutOfIndex(format!("instr {} validate: labelidx", opname)))?;
        Ok(label)
    }

    fn check_table_exist(context: &Context, opname: &str) -> Result<TableType, Error> {
        let tabletype = context.table()
            .ok_or(Error::OutOfIndex(format!("instr {} validate: not exist table", opname)))?;
        Ok(tabletype)
    }

    fn check_mem_exist(context: &Context, opname: &str) -> Result<(), Error> {
        let _memtype = context.mem()
            .ok_or(Error::OutOfIndex(format!("instr {} validate: not exist mem", opname)))?;
        Ok(())
    }

    fn check_mem_alignment(opname: &str, memarg: &MemArg, width: u8) -> Result<(), Error> {
        if memarg.is_valid(width) {
            Ok(())
        } else {
            Err(Error::OutOfRange(format!("instr {} validate: memarg.align is too large", opname)))
        }
    }
}

impl MemArg {
    fn is_valid(&self, width: u8) -> bool {
        match width {
            8 => self.align == 0,
            16 => self.align <= 1,
            32 => self.align <= 2,
            64 => self.align <= 3,
            _ => unimplemented!(),
        }
    }
}
