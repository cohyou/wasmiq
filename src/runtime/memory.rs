use crate::{
    ValType,
    MemArg,
    Error,
};

use super::*;
use super::{
    Result as ExecResult,
};

impl<'a> Thread<'a> {
    pub fn execute_load(&mut self, valtype: &ValType, memarg: &MemArg) -> ExecResult {
        let (_, frame) = self.current_frame();
        let memaddr = frame.module.memaddrs[0];
        let mem = &self.store.mems[memaddr];
        let c = if let Some(StackEntry::Value(Val::I32Const(c))) = self.stack.pop() {
            c
        } else {
            unreachable!()
        };

        let ea = c + memarg.offset;
        let n = match valtype {
            ValType::I32 | ValType::F32 => 32,
            ValType::I64 | ValType::F64 => 64,
        };
        let max = (n / 8) as usize;
        let ea = ea as usize;
        if ea + max > mem.data.len() {
            return Result::Trap;
        }

        // let bytes = mem.data[ea..max];
        // let c = u64::from_le_bytes(bytes);

        // Result::Vals(vec![Val::I64Const(c)])
        unimplemented!()
    }

    pub fn execute_store(&mut self, valtype: &ValType, memarg: &MemArg) -> ExecResult {
        let (_, frame) = self.current_frame();
        let memaddr = frame.module.memaddrs[0];
        let mem = &self.store.mems[memaddr];
        let c = if let Some(StackEntry::Value(Val::I32Const(c))) = self.stack.pop() {
            c
        } else {
            unreachable!()
        };

        let ea = c + memarg.offset;
        let n = match valtype {
            ValType::I32 | ValType::F32 => 32,
            ValType::I64 | ValType::F64 => 64,
        };
        let max = (n / 8) as usize;
        let ea = ea as usize;
        if ea + max > mem.data.len() {
            return Result::Trap;
        }
        unimplemented!()
    }

    pub fn execute_memorysize(&mut self) -> Result {
        let (_, frame) = self.current_frame();
        let memaddr = frame.module.memaddrs[0];
        let mem = &self.store.mems[memaddr];
        let sz = mem.data.len() / (64*1024);
        Result::Vals(vec![Val::I32Const(sz as u32)])
    }

    pub fn execute_memorygrow(&mut self) -> Result {
        let (_, frame) = self.current_frame();
        let memaddr = frame.module.memaddrs[0];
        let meminst = &self.store.mems[memaddr];
        let sz = meminst.data.len() / (64*1024);
        let n = if let Some(StackEntry::Value(Val::I32Const(n))) = self.stack.pop() {
            n
        } else {
            unreachable!()
        };

        let err = u32::MAX;

        if let Err(_) = grow_mem(&mut self.store.mems[memaddr], n as usize){
            Result::Vals(vec![Val::I32Const(err)])
        } else {
            Result::Vals(vec![Val::I32Const(sz as u32)])
        }
    }
}

pub fn grow_mem(meminst: &mut MemInst, n: usize) -> std::result::Result<(), Error> {
    let len = n + (meminst.data.len() / (64*1024));
    if len > 2usize.pow(16) { return Err(Error::Invalid); }
    if let Some(mx) = meminst.max {
        if (mx as usize) < len { return Err(Error::Invalid); }
    }
    for _ in 0..n {
        let page = [0x00;64*1024];
        meminst.data.extend(Vec::from(page));    
    }

    Ok(())
}