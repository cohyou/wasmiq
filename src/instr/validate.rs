mod sequence;

use crate::{
    ValType as ValTypeOriginal,
    ResultType as ResultTypeOriginal,
    FuncType as FuncTypeOriginal,
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
    BlockType,
    Instr,
    ValSize,
    CvtOp,
};

#[macro_export]
macro_rules! ft {
    ($args: expr, $rets: expr) => {
        Ok((ResultType($args), ResultType($rets)))
    };
}

#[macro_export]
macro_rules! instr_tp {
    ($res: ident) => {
        ft!(vec![], vec![ValType::$res])
    };
    ($arg: ident -> $res: ident) => {
        ft!(vec![ValType::$arg], vec![ValType::$res])
    };
    ($arg1: ident $arg2: ident -> $res: ident) => {
        ft!(vec![ValType::$arg1, ValType::$arg2], vec![ValType::$res])
    };
    ($arg1: ident $arg2: ident $arg3: ident -> $res: ident) => {
        ft!(vec![ValType::$arg1, ValType::$arg2, ValType::$arg3], vec![ValType::$res])
    };
    ($arg: ident ->) => {
        ft!(vec![ValType::$arg], vec![])
    };
    ($arg1: ident $arg2: ident ->) => {
        ft!(vec![ValType::$arg1, ValType::$arg2], vec![])
    };
    (() -> ()) => {
        ft!(vec![], vec![])
    };
    (Ellipsis -> Ellipsis) => {
        ft!(vec![ValType::Ellipsis], vec![ValType::Ellipsis])
    }
}

type TypeValIdx = u32;

#[derive(Clone, PartialEq, Debug)]
pub enum ValType {
    I32, I64, F32, F64,
    TypeVal(TypeValIdx),
    Ellipsis,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ResultType(pub Vec<ValType>);

use std::slice::Iter;
impl ResultType {
    fn iter(&self) -> Iter<'_, ValType> {
        self.0.iter()
    }

    // fn get(&self, idx: usize) -> Option<&ValType> {
    //     self.0.get(idx)
    // }

    // fn len(&self) -> usize {
    //     self.0.len()
    // }

    fn last(&self) -> Option<&ValType> {
        self.0.last()
    }

    fn last2(&self) -> Option<&ValType> {
        self.0.get(self.0.len() - 2)
    }

    // fn valtypes(&self) -> Vec<ValType> {
    //     self.0
    // }
}

type FuncType = (ResultType, ResultType);

pub fn vt(vt: &ValTypeOriginal) -> ValType {
    match vt {
        ValTypeOriginal::I32 => ValType::I32,
        ValTypeOriginal::I64 => ValType::I64,
        ValTypeOriginal::F32 => ValType::F32,
        ValTypeOriginal::F64 => ValType::F64,
    }
}

pub fn vt_rev(vt: &ValType) -> ValTypeOriginal {
    match vt {
        ValType::I32 => ValTypeOriginal::I32,
        ValType::I64 => ValTypeOriginal::I64,
        ValType::F32 => ValTypeOriginal::F32,
        ValType::F64 => ValTypeOriginal::F64,
        _ => unimplemented!(),
    }
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
                    CvtOp::IExtend8S(_valsize) => unimplemented!(),
                    CvtOp::IExtend16S(_valsize) => unimplemented!(),
                    CvtOp::I64Extend32S => unimplemented!(),
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
                        ft!(vec![arg_tp], vec![ret_tp])
                    },
                    CvtOp::ITruncSatFromF(_valsize_i, _valsize_f, _) => unimplemented!(),
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
                        ft!(vec![arg_tp], vec![ret_tp])
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

            // value-polymorphic
            Instr::Drop(None) => ft!(vec![ValType::TypeVal(0)], vec![]),
            Instr::Drop(Some(valtype)) => ft!(vec![vt(valtype)], vec![]),
            // value-polymorphic
            Instr::Select(None) => {
                ft!(vec![ValType::TypeVal(0), ValType::TypeVal(0), ValType::I32], vec![ValType::TypeVal(0)])
            },
            Instr::Select(Some(valtype)) => {
                ft!(vec![vt(valtype), vt(valtype), ValType::I32], vec![vt(valtype)])
            },
            /*
            VARIABLE INSTRUCTIONS
            */
            Instr::LocalGet(localidx) => {
                let tp = Instr::check_local(context, localidx, "local.get")?;
                ft!(vec![], vec![vt(&tp)])
            },
            Instr::LocalSet(localidx) => {
                let tp = Instr::check_local(context, localidx, "local.set")?;
                ft!(vec![vt(&tp)], vec![])
            },
            Instr::LocalTee(localidx) => {
                let tp = Instr::check_local(context, localidx, "local.tee")?;
                ft!(vec![vt(&tp)], vec![vt(&tp)])
            },
            Instr::GlobalGet(globalidx) => {
                let globaltype = Instr::check_global(context, globalidx, "global.get")?;
                ft!(vec![], vec![vt(&globaltype.0)])
            },
            Instr::GlobalSet(globalidx) => {
                let globaltype = Instr::check_global(context, globalidx, "global.set")?;

                if globaltype.is_var() {
                    ft!(vec![vt(&globaltype.0)], vec![])
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
                    ValTypeOriginal::I32 | ValTypeOriginal::F32 => 32,
                    ValTypeOriginal::I64 | ValTypeOriginal::F64 => 64,
                };
                let _ = Instr::check_mem_alignment(opname, memarg, width)?;

                ft!(vec![ValType::I32], vec![vt(valtype)])
            },
            Instr::ILoad8(valsize, _, memarg) => {
                let opname = "iload8";
                let _ = Instr::check_mem_exist(context, opname)?;
                let _ = Instr::check_mem_alignment(opname, memarg, 8)?;

                let valtype = match valsize {
                    ValSize::V32 => ValType::I32,
                    ValSize::V64 => ValType::I64,
                };

                ft!(vec![ValType::I32], vec![valtype.clone()])
            },
            Instr::ILoad16(valsize, _, memarg) => {
                let opname = "iload16";
                let _ = Instr::check_mem_exist(context, opname)?;
                let _ = Instr::check_mem_alignment(opname, memarg, 16)?;

                let valtype = match valsize {
                    ValSize::V32 => ValType::I32,
                    ValSize::V64 => ValType::I64,
                };

                ft!(vec![ValType::I32], vec![valtype.clone()])
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
                    ValTypeOriginal::I32 | ValTypeOriginal::F32 => 32,
                    ValTypeOriginal::I64 | ValTypeOriginal::F64 => 64,
                };
                let _ = Instr::check_mem_alignment(opname, memarg, width)?;

                ft!(vec![ValType::I32, vt(valtype)], vec![])
            },
            Instr::IStore8(valsize, memarg) => {
                let opname = "istore8";
                let _ = Instr::check_mem_exist(context, opname)?;
                let _ = Instr::check_mem_alignment(opname, memarg, 8)?;

                let valtype = match valsize {
                    ValSize::V32 => ValType::I32,
                    ValSize::V64 => ValType::I64,
                };

                ft!(vec![ValType::I32, valtype.clone()], vec![])
            },
            Instr::IStore16(valsize, memarg) => {
                let opname = "istore16";
                let _ = Instr::check_mem_exist(context, opname)?;
                let _ = Instr::check_mem_alignment(opname, memarg, 16)?;

                let valtype = match valsize {
                    ValSize::V32 => ValType::I32,
                    ValSize::V64 => ValType::I64,
                };

                ft!(vec![ValType::I32, valtype.clone()], vec![])
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
            Instr::Unreachable => instr_tp!(Ellipsis -> Ellipsis),
            Instr::Block(blocktype, instrs) => {
                let ft = blocktype.validate(context)?;
                let vts: Vec<ValTypeOriginal> = ft.1.0.iter().map(|v| vt_rev(v)).collect();
                let context = context.clone_with_labels(vts);
                Instr::validate_instr_sequence(&context, &instrs)
            },
            Instr::Loop(blocktype, instrs) => {
                let ft = blocktype.validate(context)?;
                let vts: Vec<ValTypeOriginal> = ft.0.0.iter().map(|v| vt_rev(v)).collect();
                let context = context.clone_with_labels(vts);
                Instr::validate_instr_sequence(&context, &instrs)
            },
            Instr::If(blocktype, instrs1, None) => {
                let ft = blocktype.validate(context)?;
                let vts: Vec<ValTypeOriginal> = ft.1.0.iter().map(|v| vt_rev(v)).collect();
                let context = context.clone_with_labels(vts);
                Instr::validate_instr_sequence(&context, &instrs1)
            },
            Instr::If(blocktype, instrs1, Some(instrs2)) => {
                let ft = blocktype.validate(context)?;
                let vts: Vec<ValTypeOriginal> = ft.1.0.iter().map(|v| vt_rev(v)).collect();
                let context = context.clone_with_labels(vts);
                let functype1 = Instr::validate_instr_sequence(&context, &instrs1)?;
                let functype2 = Instr::validate_instr_sequence(&context, &instrs2)?;
                if functype1 != functype2 {
                    return Err(Error::Invalid("Instr::If validate functype1 != functype2".to_owned()));
                }
                Ok(functype1)
            },
            Instr::Br(labelidx) => {
                let label = Instr::check_label(context, labelidx, "br")?;
                let label: Vec<ValType> = label.iter().map(|v| vt(v)).collect();
                let mut vts = vec![ValType::Ellipsis];
                vts.extend(label);
                ft!(vts, vec![ValType::Ellipsis])
            },
            Instr::BrIf(labelidx) => {
                let label = Instr::check_label(context, labelidx, "brif")?;
                let label: Vec<ValType> = label.iter().map(|v| vt(v)).collect();
                let mut args = label.clone();
                args.push(ValType::I32);
                ft!(args, label)
            },
            Instr::BrTable(_labelindices, labelidx) => {
                let label = Instr::check_label(context, labelidx, "brtable")?;
                let label: Vec<ValType> = label.iter().map(|v| vt(v)).collect();
                let mut args = label.clone();
                args.push(ValType::I32);
                let mut vts = vec![ValType::Ellipsis];
                vts.extend(args);
                ft!(vts, vec![ValType::Ellipsis])
            },
            Instr::Return => {
                if let Some(rettp) = context.rtn() {
                    let rettp: Vec<ValType> = rettp.iter().map(|v| vt(v)).collect();
                    let mut vts = vec![ValType::Ellipsis];
                    vts.extend(rettp);
                    ft!(vts, vec![ValType::Ellipsis])
                } else {
                    Err(Error::PreCondition("instr return validate: context.return is absent".to_string()))
                }
            },
            Instr::Call(funcidx) => {
                if let Some(functype) = context.func(funcidx.clone()) {
                    let ft0 = functype.0.iter().map(|v| vt(v)).collect();
                    let ft1 = functype.1.iter().map(|v| vt(v)).collect();
                    ft!(ft0, ft1)
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
                    let mut tp0: Vec<ValType> = tp.0.iter().map(|v| vt(v)).collect();
                    let tp1: Vec<ValType> = tp.1.iter().map(|v| vt(v)).collect();
                    tp0.push(ValType::I32);
    
                    ft!(tp0, tp1)
                }
            },

            // _ => unimplemented!(),
        }
    }

    fn check_type(context: &Context, typeidx: &TypeIdx, opname: &str) -> Result<FuncTypeOriginal, Error> {
        let tp = context.tp(typeidx.clone())
            .ok_or(Error::OutOfIndex(format!("instr {} validate: typeidx", opname)))?;
        Ok(tp)
    }

    fn check_local(context: &Context, localidx: &LocalIdx, opname: &str) -> Result<ValTypeOriginal, Error> {
        let tp = context.local(localidx.clone())
            .ok_or(Error::OutOfIndex(format!("instr {} validate: localidx", opname)))?;
        Ok(tp)
    }

    fn check_global(context: &Context, globalidx: &GlobalIdx, opname: &str) -> Result<GlobalType, Error> {
        let globaltype = context.global(globalidx.clone())
            .ok_or(Error::OutOfIndex(format!("instr {} validate: globalidx", opname)))?;
        Ok(globaltype)
    }

    fn check_label(context: &Context, labelidx: &LabelIdx, opname: &str) -> Result<ResultTypeOriginal, Error> {
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

impl BlockType {
    fn validate(&self, context: &Context) -> Result<FuncType, Error> {
        match &self {
            BlockType::TypeIdx(idx) => {
                let functype = context.tp(idx.clone())
                    .ok_or(Error::OutOfIndex(format!("blocktype validate: typeidx")))?;
                let tp0: Vec<ValType> = functype.0.iter().map(|v| vt(v)).collect();
                let tp1: Vec<ValType> = functype.1.iter().map(|v| vt(v)).collect();

                ft!(tp0, tp1)
            },
            BlockType::ValType(Some(valtype)) => ft!(vec![], vec![vt(valtype)]),
            BlockType::ValType(None) => instr_tp!(() -> ()),
        }
    }
}