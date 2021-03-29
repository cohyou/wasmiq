use crate::{
    Instr,
    Thread,
    ValSize,
    ValSign,
    IUnOp,
    FUnOp,
    IBinOp,
    FBinOp,
    ITestOp,
    IRelOp,
    FRelOp,
    CvtOp,
    ValType,
    // LabelIdx,
};

use super::*;

// macro_rules! vali32 {
//     ($i:ident) => {
//         StackEntry::Value(Val::I32Const($i))
//     };
// }

// use std::collections::VecDeque;

impl<'a> Thread<'a> {
    pub fn spawn(&mut self, instrs: &Vec<Instr>) {
        self.execute_instrs(instrs);
    }

    fn execute_instrs(&mut self, instrs: &Vec<Instr>) -> Result {
        for instr in instrs {
            // let mut vals = vec![];
            // while let Some(StackEntry::Value(val)) = self.stack.pop() {
            //     vals.insert(0, val);
            // }

            match self.execute_instr(instr) {
                Result::Vals(vals) => {
                    // let entries = vals.iter().map(|v| StackEntry::Value(v.clone()));
                    // self.stack.extend(entries.collect::<Vec<StackEntry>>());
                
                    // if let Some(mut instr_new) = instrs_new {
                    //     while let Some(instr) = instr_new.pop() {
                    //         instrs.push_front(instr);
                    //     }
                    // }
                    // Result::Vals(vals)
                    // result_vals = vals;
                    let vals: Vec<StackEntry> = vals.iter()
                        .map(|v| StackEntry::Value(v.clone())).collect();
                    self.stack.extend(vals);
                },
                Result::Trap => return Result::Trap,
                // _ => unimplemented!(),
            }
        }
        Result::Vals(vec![])
    }

    fn execute_instr(&mut self, instr: &Instr) -> Result {
        match instr {
            /* Block Instructions */
    
            // Control Instructions
            Instr::Block(_blocktype, _instrs) => unimplemented!(),
            Instr::Loop(_blocktype, _instrs) => unimplemented!(),
            Instr::If(_blocktype, _instrs1, _instrs2) => unimplemented!(),
    
    
            /* Plain Instructions */
    
            // Control Instructions
            Instr::Unreachable => unimplemented!(),
            Instr::Nop => unimplemented!(),
            Instr::Br(_labelidx) => unimplemented!(),
            Instr::BrIf(_labelidx) => unimplemented!(),
            Instr::BrTable(_labelindices, _labelidx) => unimplemented!(),
            Instr::Return => unimplemented!(),
            Instr::Call(_funcidx) => unimplemented!(),
            Instr::CallIndirect(_funcidx) => unimplemented!(),
    
            // Parametric Instructions
            Instr::Drop(_valtype) => unimplemented!(),
            Instr::Select(_valtype) => unimplemented!(),
    
            // Variable Instructions
            Instr::LocalGet(_localidx) => unimplemented!(),
            Instr::LocalSet(_localidx) => unimplemented!(),
            Instr::LocalTee(_localidx) => unimplemented!(),
            Instr::GlobalGet(_globalidx) => unimplemented!(),
            Instr::GlobalSet(_globalidx) => unimplemented!(),
    
            // Memory Instructions
            Instr::Load(_valtype, _memarg) => unimplemented!(),
            Instr::Store(_valtype, _memarg) => unimplemented!(),
            Instr::ILoad8(_valsize, _valsign, _memarg) => unimplemented!(),
            Instr::ILoad16(_valsize, _valsign, _memarg) => unimplemented!(),
            Instr::I64Load32(_valsign, _memarg) => unimplemented!(),
            Instr::IStore8(_valsize, _memarg) => unimplemented!(),
            Instr::IStore16(_valsize, _memarg) => unimplemented!(),
            Instr::I64Store32(_memarg) => unimplemented!(),
            Instr::MemorySize => unimplemented!(),
            Instr::MemoryGrow => unimplemented!(),
    
            // Numeric Instructions
            Instr::I32Const(i) => Result::Vals(vec![Val::I32Const(i.clone())]),
            Instr::I64Const(i) => Result::Vals(vec![Val::I64Const(i.clone())]),
            Instr::F32Const(f) => Result::Vals(vec![Val::F32Const(f.clone())]),
            Instr::F64Const(f) => Result::Vals(vec![Val::F64Const(f.clone())]),
    
            Instr::IUnOp(ValSize::V32, IUnOp::Clz) => self.execute_iclz32(),
            Instr::IUnOp(ValSize::V64, IUnOp::Clz) => self.execute_iclz64(),
            Instr::IUnOp(ValSize::V32, IUnOp::Ctz) => self.execute_ictz32(),
            Instr::IUnOp(ValSize::V64, IUnOp::Ctz) => self.execute_ictz64(),
            Instr::IUnOp(ValSize::V32, IUnOp::Popcnt) => self.execute_ipopcnt32(),
            Instr::IUnOp(ValSize::V64, IUnOp::Popcnt) => self.execute_ipopcnt64(),
            Instr::FUnOp(ValSize::V32, FUnOp::Abs) => self.execute_fabs32(),
            Instr::FUnOp(ValSize::V64, FUnOp::Abs) => self.execute_fabs64(),
            Instr::FUnOp(ValSize::V32, FUnOp::Neg) => self.execute_fneg32(),
            Instr::FUnOp(ValSize::V64, FUnOp::Neg) => self.execute_fneg64(),
            Instr::FUnOp(ValSize::V32, FUnOp::Sqrt) => self.execute_fsqrt32(),
            Instr::FUnOp(ValSize::V64, FUnOp::Sqrt) => self.execute_fsqrt64(),
            Instr::FUnOp(ValSize::V32, FUnOp::Ceil) => self.execute_fceil32(),
            Instr::FUnOp(ValSize::V64, FUnOp::Ceil) => self.execute_fceil64(),
            Instr::FUnOp(ValSize::V32, FUnOp::Floor) => self.execute_ffloor32(),
            Instr::FUnOp(ValSize::V64, FUnOp::Floor) => self.execute_ffloor64(),
            Instr::FUnOp(ValSize::V32, FUnOp::Trunc) => self.execute_ftrunc32(),
            Instr::FUnOp(ValSize::V64, FUnOp::Trunc) => self.execute_ftrunc64(),
            Instr::FUnOp(ValSize::V32, FUnOp::Nearest) => self.execute_fnearest32(),
            Instr::FUnOp(ValSize::V64, FUnOp::Nearest) => self.execute_fnearest64(),
    
            Instr::IBinOp(ValSize::V32, IBinOp::Add) => self.execute_iadd32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Add) => self.execute_iadd64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Sub) => self.execute_isub32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Sub) => self.execute_isub64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Mul) => self.execute_imul32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Mul) => self.execute_imul64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Div(ValSign::U)) => self.execute_idiv_u32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Div(ValSign::U)) => self.execute_idiv_u64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Div(ValSign::S)) => self.execute_idiv_s32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Div(ValSign::S)) => self.execute_idiv_s64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Rem(ValSign::U)) => self.execute_irem_u32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Rem(ValSign::U)) => self.execute_irem_u64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Rem(ValSign::S)) => self.execute_irem_s32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Rem(ValSign::S)) => self.execute_irem_s64(),
            Instr::IBinOp(ValSize::V32, IBinOp::And) => self.execute_iand32(),
            Instr::IBinOp(ValSize::V64, IBinOp::And) => self.execute_iand64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Or) => self.execute_ior32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Or) => self.execute_ior64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Xor) => self.execute_ixor32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Xor) => self.execute_ixor64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Shl) => self.execute_ishl32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Shl) => self.execute_ishl64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Shr(ValSign::U)) => self.execute_ishr_u32(),
            Instr::IBinOp(ValSize::V32, IBinOp::Shr(ValSign::S)) => self.execute_ishr_s32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Shr(ValSign::U)) => self.execute_ishr_u64(),
            Instr::IBinOp(ValSize::V64, IBinOp::Shr(ValSign::S)) => self.execute_ishr_s64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Rotl) => self.execute_irotl32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Rotl) => self.execute_irotl64(),
            Instr::IBinOp(ValSize::V32, IBinOp::Rotr) => self.execute_irotr32(),
            Instr::IBinOp(ValSize::V64, IBinOp::Rotr) => self.execute_irotr64(),
            Instr::FBinOp(ValSize::V32, FBinOp::Add) => self.execute_fadd32(),
            Instr::FBinOp(ValSize::V64, FBinOp::Add) => self.execute_fadd64(),
            Instr::FBinOp(ValSize::V32, FBinOp::Sub) => self.execute_fsub32(),
            Instr::FBinOp(ValSize::V64, FBinOp::Sub) => self.execute_fsub64(),
            Instr::FBinOp(ValSize::V32, FBinOp::Mul) => self.execute_fmul32(),
            Instr::FBinOp(ValSize::V64, FBinOp::Mul) => self.execute_fmul64(),
            Instr::FBinOp(ValSize::V32, FBinOp::Div) => self.execute_fdiv32(),
            Instr::FBinOp(ValSize::V64, FBinOp::Div) => self.execute_fdiv64(),
            Instr::FBinOp(ValSize::V32, FBinOp::Min) => self.execute_fmin32(),
            Instr::FBinOp(ValSize::V64, FBinOp::Min) => self.execute_fmin64(),
            Instr::FBinOp(ValSize::V32, FBinOp::Max) => self.execute_fmax32(),
            Instr::FBinOp(ValSize::V64, FBinOp::Max) => self.execute_fmax64(),
            Instr::FBinOp(ValSize::V32, FBinOp::Copysign) => self.execute_fcopysign32(),
            Instr::FBinOp(ValSize::V64, FBinOp::Copysign) => self.execute_fcopysign64(),
    
            Instr::ITestOp(ValSize::V32, ITestOp::Eqz) => self.execute_ieqz32(),
            Instr::ITestOp(ValSize::V64, ITestOp::Eqz) => self.execute_ieqz64(),
    
            Instr::IRelOp(ValSize::V32, IRelOp::Eq) => self.execute_ieq32(),
            Instr::IRelOp(ValSize::V64, IRelOp::Eq) => self.execute_ieq64(),
            Instr::IRelOp(ValSize::V32, IRelOp::Ne) => self.execute_ine32(),
            Instr::IRelOp(ValSize::V64, IRelOp::Ne) => self.execute_ine64(),
            Instr::IRelOp(ValSize::V32, IRelOp::Lt(ValSign::U)) => self.execute_ilt_u32(),
            Instr::IRelOp(ValSize::V32, IRelOp::Lt(ValSign::S)) => self.execute_ilt_s32(),
            Instr::IRelOp(ValSize::V64, IRelOp::Lt(ValSign::U)) => self.execute_ilt_u64(),
            Instr::IRelOp(ValSize::V64, IRelOp::Lt(ValSign::S)) => self.execute_ilt_s64(),
            Instr::IRelOp(ValSize::V32, IRelOp::Gt(ValSign::U)) => self.execute_igt_u32(),
            Instr::IRelOp(ValSize::V32, IRelOp::Gt(ValSign::S)) => self.execute_igt_s32(),
            Instr::IRelOp(ValSize::V64, IRelOp::Gt(ValSign::U)) => self.execute_igt_u64(),
            Instr::IRelOp(ValSize::V64, IRelOp::Gt(ValSign::S)) => self.execute_igt_s64(),
            Instr::IRelOp(ValSize::V32, IRelOp::Le(ValSign::U)) => self.execute_ile_u32(),
            Instr::IRelOp(ValSize::V32, IRelOp::Le(ValSign::S)) => self.execute_ile_s32(),
            Instr::IRelOp(ValSize::V64, IRelOp::Le(ValSign::U)) => self.execute_ile_u64(),
            Instr::IRelOp(ValSize::V64, IRelOp::Le(ValSign::S)) => self.execute_ile_s64(),
            Instr::IRelOp(ValSize::V32, IRelOp::Ge(ValSign::U)) => self.execute_ige_u32(),
            Instr::IRelOp(ValSize::V32, IRelOp::Ge(ValSign::S)) => self.execute_ige_s32(),
            Instr::IRelOp(ValSize::V64, IRelOp::Ge(ValSign::U)) => self.execute_ige_u64(),
            Instr::IRelOp(ValSize::V64, IRelOp::Ge(ValSign::S)) => self.execute_ige_s64(),
            Instr::FRelOp(ValSize::V32, FRelOp::Eq) => self.execute_feq32(),
            Instr::FRelOp(ValSize::V64, FRelOp::Eq) => self.execute_feq64(),
            Instr::FRelOp(ValSize::V32, FRelOp::Ne) => self.execute_fne32(),
            Instr::FRelOp(ValSize::V64, FRelOp::Ne) => self.execute_fne64(),
            Instr::FRelOp(ValSize::V32, FRelOp::Lt) => self.execute_flt32(),
            Instr::FRelOp(ValSize::V64, FRelOp::Lt) => self.execute_flt64(),
            Instr::FRelOp(ValSize::V32, FRelOp::Gt) => self.execute_fgt32(),
            Instr::FRelOp(ValSize::V64, FRelOp::Gt) => self.execute_fgt64(),
            Instr::FRelOp(ValSize::V32, FRelOp::Le) => self.execute_fle32(),
            Instr::FRelOp(ValSize::V64, FRelOp::Le) => self.execute_fle64(),
            Instr::FRelOp(ValSize::V32, FRelOp::Ge) => self.execute_fge32(),
            Instr::FRelOp(ValSize::V64, FRelOp::Ge) => self.execute_fge64(),
    
            Instr::CvtOp(CvtOp::IExtend8S(ValSize::V32)) => unimplemented!(),
            Instr::CvtOp(CvtOp::IExtend8S(ValSize::V64)) => unimplemented!(),
            Instr::CvtOp(CvtOp::IExtend16S(ValSize::V32)) => unimplemented!(),
            Instr::CvtOp(CvtOp::IExtend16S(ValSize::V64)) => unimplemented!(),
            Instr::CvtOp(CvtOp::I64Extend32S) => unimplemented!(),
            Instr::CvtOp(CvtOp::I32WrapFromI64) => unimplemented!(),
            Instr::CvtOp(CvtOp::I64ExtendFromI32(ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::I64ExtendFromI32(ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V32, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V32, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V64, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V32, ValSize::V64, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V32, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V32, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V64, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncFromF(ValSize::V64, ValSize::V64, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncSatFromF(ValSize::V32, ValSize::V32, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncSatFromF(ValSize::V32, ValSize::V32, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncSatFromF(ValSize::V32, ValSize::V64, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncSatFromF(ValSize::V32, ValSize::V64, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncSatFromF(ValSize::V64, ValSize::V32, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncSatFromF(ValSize::V64, ValSize::V32, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncSatFromF(ValSize::V64, ValSize::V64, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::ITruncSatFromF(ValSize::V64, ValSize::V64, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::F32DemoteFromF64) => unimplemented!(),
            Instr::CvtOp(CvtOp::F64PromoteFromF32) => unimplemented!(),
            Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V32, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V32, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V64, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V32, ValSize::V64, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V32, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V32, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V64, ValSign::U)) => unimplemented!(),
            Instr::CvtOp(CvtOp::FConvertFromI(ValSize::V64, ValSize::V64, ValSign::S)) => unimplemented!(),
            Instr::CvtOp(CvtOp::IReinterpretFromF(ValSize::V32)) => unimplemented!(),
            Instr::CvtOp(CvtOp::IReinterpretFromF(ValSize::V64)) => unimplemented!(),
            Instr::CvtOp(CvtOp::FReinterpretFromI(ValSize::V32)) => unimplemented!(),
            Instr::CvtOp(CvtOp::FReinterpretFromI(ValSize::V64)) => unimplemented!(),
    
            // Administrative Instructions
            // Instr::Trap => (None, None),
            // Instr::Invoke(funcaddr) => self.execute_invoke(funcaddr, vals),
            // Instr::InitElem(tableaddr, offset, funcindices) => {
            //     init_elem(tableaddr, offset, funcindices);
            //     (None, None)
            // },
            // Instr::InitData(memaddr, offset, bytes) => {
            //     init_data(memaddr, offset, bytes);
            //     (None, None)
            // },
            // Instr::Label(labelidx, instrs_cont, instrs) => self.execute_instrs_with_label(labelidx, instrs_cont, instrs),
            // Instr::Frame(_frameidx, _frame, _instrs) => unimplemented!(),
        }
    }

    pub fn execute_invoke(&mut self, funcaddr: &FuncAddr) {
        // let mut instrs = vec![];
        let funcinst = self.store.funcs[funcaddr.clone()].clone();
    
        match funcinst {
            FuncInst::User(userfunc) => {
                let (argtypes, returntypes) = userfunc.tp;
                let localtypes = userfunc.code.locals;
                let expr = userfunc.code.body;
                let mut args = vec![];
                
                let arg_count = argtypes.len();
                for _ in 0..arg_count {
                    if let Some(StackEntry::Value(val)) = self.stack.pop() {
                        args.push(val);
                    }
                }
    
                let mut locals = vec![];
                for localtype in localtypes {
                    let val = match localtype {
                        ValType::I32 => Val::I32Const(0),
                        ValType::I64 => Val::I64Const(0),
                        ValType::F32 => Val::F32Const(0.0),
                        ValType::F64 => Val::F64Const(0.0),
                    };
                    locals.push(val);
                }
    
                locals.extend(args);
    
                let frame = Frame{ module: userfunc.module, locals: locals };
                let m = returntypes.len();
                let activation = StackEntry::Activation(m as u32, frame);
                self.stack.push(activation);
                let label = StackEntry::Label(m as u32, vec![]);

                self.execute_instrs_with_label(label, &expr.0);

                let mut vals = vec![];
                let n = m;
                for _ in 0..n {
                    if let Some(val) = self.stack.pop() {
                        vals.push(val);
                    }
                }

                // pop the label
                self.stack.pop();  

                self.stack.extend(vals);
            },
            FuncInst::Host(hostfunc) => {
                let f = hostfunc.hostcode;
                f();
            },
        }
    }

    fn execute_instrs_with_label(&mut self, label: StackEntry, instrs: &Vec<Instr>) {
        self.stack.push(label);

        self.execute_instrs(instrs);

        let mut vals = vec![];
        let m = vals.len();
        for _ in 0..m {
            if let Some(val) = self.stack.pop() {
                vals.push(val);
            }
        }

        // pop the label
        self.stack.pop();  
        
        self.stack.extend(vals);
    }
}





