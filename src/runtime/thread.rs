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
};

use super::*;

// macro_rules! vali32 {
//     ($i:ident) => {
//         StackEntry::Value(Val::I32Const($i))
//     };
// }

use std::collections::VecDeque;

impl Thread {
    pub fn spawn<'a>(&mut self, store: &'a mut Store, instrs: &mut VecDeque<Instr>) -> &'a mut Store {
        self.execute_instrs(store, instrs);
        store
    }

    fn execute_instrs<'a>(&mut self, store: &'a mut Store, instrs: &mut VecDeque<Instr>) {
        while let Some(instr) = instrs.pop_front() {
            let mut vals = vec![];
            while let Some(StackEntry::Value(val)) = self.stack.pop() {
                vals.insert(0, val);
            }

            match execute_instr(store, &instr, &mut vals) {
                (instrs_new, Some(Result::Vals(vals))) => {
                    let entries = vals.iter().map(|v| StackEntry::Value(v.clone()));
                    self.stack.extend(entries.collect::<Vec<StackEntry>>());
                
                    if let Some(mut instr_new) = instrs_new {
                        while let Some(instr) = instr_new.pop() {
                            instrs.push_front(instr);
                        }
                    }
                },
                (_, Some(Result::Trap)) => instrs.push_front(Instr::Trap),
                _ => unimplemented!(),
            }
        }
    }


}

fn execute_instr<'a>(store: &'a mut Store, instr: &Instr, vals: &mut Vec<Val>) -> (Option<Vec<Instr>>, Option<Result>) {
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
        Instr::I32Const(i) => (None, Some(Result::Vals(vec![Val::I32Const(i.clone())]))),
        Instr::I64Const(i) => (None, Some(Result::Vals(vec![Val::I64Const(i.clone())]))),
        Instr::F32Const(f) => (None, Some(Result::Vals(vec![Val::F32Const(f.clone())]))),
        Instr::F64Const(f) => (None, Some(Result::Vals(vec![Val::F64Const(f.clone())]))),

        Instr::IUnOp(ValSize::V32, IUnOp::Clz) => (None, Some(execute_iclz32(vals))),
        Instr::IUnOp(ValSize::V64, IUnOp::Clz) => (None, Some(execute_iclz64(vals))),
        Instr::IUnOp(ValSize::V32, IUnOp::Ctz) => (None, Some(execute_ictz32(vals))),
        Instr::IUnOp(ValSize::V64, IUnOp::Ctz) => (None, Some(execute_ictz64(vals))),
        Instr::IUnOp(ValSize::V32, IUnOp::Popcnt) => (None, Some(execute_ipopcnt32(vals))),
        Instr::IUnOp(ValSize::V64, IUnOp::Popcnt) => (None, Some(execute_ipopcnt64(vals))),
        Instr::FUnOp(ValSize::V32, FUnOp::Abs) => (None, Some(execute_fabs32(vals))),
        Instr::FUnOp(ValSize::V64, FUnOp::Abs) => (None, Some(execute_fabs64(vals))),
        Instr::FUnOp(ValSize::V32, FUnOp::Neg) => (None, Some(execute_fneg32(vals))),
        Instr::FUnOp(ValSize::V64, FUnOp::Neg) => (None, Some(execute_fneg64(vals))),
        Instr::FUnOp(ValSize::V32, FUnOp::Sqrt) => (None, Some(execute_fsqrt32(vals))),
        Instr::FUnOp(ValSize::V64, FUnOp::Sqrt) => (None, Some(execute_fsqrt64(vals))),
        Instr::FUnOp(ValSize::V32, FUnOp::Ceil) => (None, Some(execute_fceil32(vals))),
        Instr::FUnOp(ValSize::V64, FUnOp::Ceil) => (None, Some(execute_fceil64(vals))),
        Instr::FUnOp(ValSize::V32, FUnOp::Floor) => (None, Some(execute_ffloor32(vals))),
        Instr::FUnOp(ValSize::V64, FUnOp::Floor) => (None, Some(execute_ffloor64(vals))),
        Instr::FUnOp(ValSize::V32, FUnOp::Trunc) => (None, Some(execute_ftrunc32(vals))),
        Instr::FUnOp(ValSize::V64, FUnOp::Trunc) => (None, Some(execute_ftrunc64(vals))),
        Instr::FUnOp(ValSize::V32, FUnOp::Nearest) => (None, Some(execute_fnearest32(vals))),
        Instr::FUnOp(ValSize::V64, FUnOp::Nearest) => (None, Some(execute_fnearest64(vals))),

        Instr::IBinOp(ValSize::V32, IBinOp::Add) => (None, Some(execute_iadd32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Add) => (None, Some(execute_iadd64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Sub) => (None, Some(execute_isub32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Sub) => (None, Some(execute_isub64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Mul) => (None, Some(execute_imul32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Mul) => (None, Some(execute_imul64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Div(ValSign::U)) => (None, Some(execute_idiv_u32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Div(ValSign::U)) => (None, Some(execute_idiv_u64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Div(ValSign::S)) => (None, Some(execute_idiv_s32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Div(ValSign::S)) => (None, Some(execute_idiv_s64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Rem(ValSign::U)) => (None, Some(execute_irem_u32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Rem(ValSign::U)) => (None, Some(execute_irem_u64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Rem(ValSign::S)) => (None, Some(execute_irem_s32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Rem(ValSign::S)) => (None, Some(execute_irem_s64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::And) => (None, Some(execute_iand32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::And) => (None, Some(execute_iand64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Or) => (None, Some(execute_ior32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Or) => (None, Some(execute_ior64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Xor) => (None, Some(execute_ixor32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Xor) => (None, Some(execute_ixor64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Shl) => (None, Some(execute_ishl32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Shl) => (None, Some(execute_ishl64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Shr(ValSign::U)) => (None, Some(execute_ishr_u32(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Shr(ValSign::S)) => (None, Some(execute_ishr_s32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Shr(ValSign::U)) => (None, Some(execute_ishr_u64(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Shr(ValSign::S)) => (None, Some(execute_ishr_s64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Rotl) => (None, Some(execute_irotl32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Rotl) => (None, Some(execute_irotl64(vals))),
        Instr::IBinOp(ValSize::V32, IBinOp::Rotr) => (None, Some(execute_irotr32(vals))),
        Instr::IBinOp(ValSize::V64, IBinOp::Rotr) => (None, Some(execute_irotr64(vals))),
        Instr::FBinOp(ValSize::V32, FBinOp::Add) => (None, Some(execute_fadd32(vals))),
        Instr::FBinOp(ValSize::V64, FBinOp::Add) => (None, Some(execute_fadd64(vals))),
        Instr::FBinOp(ValSize::V32, FBinOp::Sub) => (None, Some(execute_fsub32(vals))),
        Instr::FBinOp(ValSize::V64, FBinOp::Sub) => (None, Some(execute_fsub64(vals))),
        Instr::FBinOp(ValSize::V32, FBinOp::Mul) => (None, Some(execute_fmul32(vals))),
        Instr::FBinOp(ValSize::V64, FBinOp::Mul) => (None, Some(execute_fmul64(vals))),
        Instr::FBinOp(ValSize::V32, FBinOp::Div) => (None, Some(execute_fdiv32(vals))),
        Instr::FBinOp(ValSize::V64, FBinOp::Div) => (None, Some(execute_fdiv64(vals))),
        Instr::FBinOp(ValSize::V32, FBinOp::Min) => (None, Some(execute_fmin32(vals))),
        Instr::FBinOp(ValSize::V64, FBinOp::Min) => (None, Some(execute_fmin64(vals))),
        Instr::FBinOp(ValSize::V32, FBinOp::Max) => (None, Some(execute_fmax32(vals))),
        Instr::FBinOp(ValSize::V64, FBinOp::Max) => (None, Some(execute_fmax64(vals))),
        Instr::FBinOp(ValSize::V32, FBinOp::Copysign) => (None, Some(execute_fcopysign32(vals))),
        Instr::FBinOp(ValSize::V64, FBinOp::Copysign) => (None, Some(execute_fcopysign64(vals))),

        Instr::ITestOp(ValSize::V32, ITestOp::Eqz) => (None, Some(execute_ieqz32(vals))),
        Instr::ITestOp(ValSize::V64, ITestOp::Eqz) => (None, Some(execute_ieqz64(vals))),

        Instr::IRelOp(ValSize::V32, IRelOp::Eq) => (None, Some(execute_ieq32(vals))),
        Instr::IRelOp(ValSize::V64, IRelOp::Eq) => (None, Some(execute_ieq64(vals))),
        Instr::IRelOp(ValSize::V32, IRelOp::Ne) => (None, Some(execute_ine32(vals))),
        Instr::IRelOp(ValSize::V64, IRelOp::Ne) => (None, Some(execute_ine64(vals))),
        Instr::IRelOp(ValSize::V32, IRelOp::Lt(ValSign::U)) => (None, Some(execute_ilt_u32(vals))),
        Instr::IRelOp(ValSize::V32, IRelOp::Lt(ValSign::S)) => (None, Some(execute_ilt_s32(vals))),
        Instr::IRelOp(ValSize::V64, IRelOp::Lt(ValSign::U)) => (None, Some(execute_ilt_u64(vals))),
        Instr::IRelOp(ValSize::V64, IRelOp::Lt(ValSign::S)) => (None, Some(execute_ilt_s64(vals))),
        Instr::IRelOp(ValSize::V32, IRelOp::Gt(ValSign::U)) => (None, Some(execute_igt_u32(vals))),
        Instr::IRelOp(ValSize::V32, IRelOp::Gt(ValSign::S)) => (None, Some(execute_igt_s32(vals))),
        Instr::IRelOp(ValSize::V64, IRelOp::Gt(ValSign::U)) => (None, Some(execute_igt_u64(vals))),
        Instr::IRelOp(ValSize::V64, IRelOp::Gt(ValSign::S)) => (None, Some(execute_igt_s64(vals))),
        Instr::IRelOp(ValSize::V32, IRelOp::Le(ValSign::U)) => (None, Some(execute_ile_u32(vals))),
        Instr::IRelOp(ValSize::V32, IRelOp::Le(ValSign::S)) => (None, Some(execute_ile_s32(vals))),
        Instr::IRelOp(ValSize::V64, IRelOp::Le(ValSign::U)) => (None, Some(execute_ile_u64(vals))),
        Instr::IRelOp(ValSize::V64, IRelOp::Le(ValSign::S)) => (None, Some(execute_ile_s64(vals))),
        Instr::IRelOp(ValSize::V32, IRelOp::Ge(ValSign::U)) => (None, Some(execute_ige_u32(vals))),
        Instr::IRelOp(ValSize::V32, IRelOp::Ge(ValSign::S)) => (None, Some(execute_ige_s32(vals))),
        Instr::IRelOp(ValSize::V64, IRelOp::Ge(ValSign::U)) => (None, Some(execute_ige_u64(vals))),
        Instr::IRelOp(ValSize::V64, IRelOp::Ge(ValSign::S)) => (None, Some(execute_ige_s64(vals))),
        Instr::FRelOp(ValSize::V32, FRelOp::Eq) => (None, Some(execute_feq32(vals))),
        Instr::FRelOp(ValSize::V64, FRelOp::Eq) => (None, Some(execute_feq64(vals))),
        Instr::FRelOp(ValSize::V32, FRelOp::Ne) => (None, Some(execute_fne32(vals))),
        Instr::FRelOp(ValSize::V64, FRelOp::Ne) => (None, Some(execute_fne64(vals))),
        Instr::FRelOp(ValSize::V32, FRelOp::Lt) => (None, Some(execute_flt32(vals))),
        Instr::FRelOp(ValSize::V64, FRelOp::Lt) => (None, Some(execute_flt64(vals))),
        Instr::FRelOp(ValSize::V32, FRelOp::Gt) => (None, Some(execute_fgt32(vals))),
        Instr::FRelOp(ValSize::V64, FRelOp::Gt) => (None, Some(execute_fgt64(vals))),
        Instr::FRelOp(ValSize::V32, FRelOp::Le) => (None, Some(execute_fle32(vals))),
        Instr::FRelOp(ValSize::V64, FRelOp::Le) => (None, Some(execute_fle64(vals))),
        Instr::FRelOp(ValSize::V32, FRelOp::Ge) => (None, Some(execute_fge32(vals))),
        Instr::FRelOp(ValSize::V64, FRelOp::Ge) => (None, Some(execute_fge64(vals))),

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
        Instr::Trap => (None, None),
        Instr::Invoke(funcaddr) => execute_invoke(store, funcaddr, vals),
        Instr::InitElem(tableaddr, offset, funcindices) => {
            init_elem(tableaddr, offset, funcindices);
            (None, None)
        },
        Instr::InitData(memaddr, offset, bytes) => {
            init_data(memaddr, offset, bytes);
            (None, None)
        },
        Instr::Label(_labelidx, _instrs_cont, _instrs) => unimplemented!(),
        Instr::Frame(_frameidx, _frame, _instrs) => unimplemented!(),
    }
}

fn execute_invoke<'a>(store: &'a mut Store, funcaddr: &FuncAddr, _vals: &mut Vec<Val>) -> (Option<Vec<Instr>>, Option<Result>) {
    let funcinst = store.funcs[funcaddr.clone()].clone();

    match funcinst {
        FuncInst::User(_userfunc) => {

        },
        FuncInst::Host(hostfunc) => {
            let f = hostfunc.hostcode;
            f();
        },
    }

    (Some(vec![]), Some(Result::Trap))
}