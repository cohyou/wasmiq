use crate::{
    ValType,
    MemArg,
    Error,
    ValSize,
    ValSign,
    MemInst,
};

use super::*;
use super::{
    Result as ExecResult,
};

impl<'a> Thread<'a> {
    pub fn execute_load(&mut self, valtype: &ValType, memarg: &MemArg) -> ExecResult {
        self.execute_load_internal(valtype, &ValSign::U, memarg, Thread::valtype2usize(valtype))
    }
    pub fn execute_iload8(&mut self, valsize: &ValSize, valsign: &ValSign, memarg: &MemArg) -> ExecResult {
        self.execute_load_internal(&Thread::valsize2valtype(valsize), valsign, memarg, 8)
    }
    pub fn execute_iload16(&mut self, valsize: &ValSize, valsign: &ValSign, memarg: &MemArg) -> ExecResult {
        self.execute_load_internal(&Thread::valsize2valtype(valsize), valsign, memarg, 16)
    }
    pub fn execute_i64load32(&mut self, valsign: &ValSign, memarg: &MemArg)-> ExecResult {
        self.execute_load_internal(&ValType::I64, valsign, memarg, 32)
    }
    pub fn execute_load_internal(&mut self, valtype: &ValType,  _valsign: &ValSign, memarg: &MemArg, n: u32) -> ExecResult {
        let (_, frame) = self.current_frame();
        let memaddr = frame.module.memaddrs[0];
        let mem = &self.store.mems[memaddr];
        let c = if let Some(StackEntry::Value(Val::I32Const(c))) = self.stack.pop() {
            c
        } else {
            unreachable!()
        };

        let ea = c + memarg.offset;
        let ea = ea as usize;
        let max = ea + (n / 8) as usize;
        if max > mem.data.len() {
            return Result::Trap;
        }

        let slice = &mem.data[ea..max];
        match n {
            8 => {
                let mut bytes = [0x00; 1];
                for i in 0..1 {
                    bytes[i] = slice[i];
                }
                let v = u8::from_le_bytes(bytes);
                match valtype {
                    ValType::I32 => Result::i32val(v as u32),
                    ValType::I64 => Result::i64val(v as u64),
                    // ValType::F32 => Result::f32val(v as f32),
                    // ValType::F64 => Result::f64val(v as f64),
                    _ => unreachable!(),
                }
            },
            16 => {
                let mut bytes = [0x00; 2];
                for i in 0..2 {
                    bytes[i] = slice[i];
                }
                let v = u16::from_le_bytes(bytes);
                match valtype {
                    ValType::I32 => Result::i32val(v as u32),
                    ValType::I64 => Result::i64val(v as u64),
                    // ValType::F32 => Result::f32val(v as f32),
                    // ValType::F64 => Result::f64val(v as f64),
                    _ => unreachable!(),
                }
            },
            32 => {
                let mut bytes = [0x00; 4];
                for i in 0..4 {
                    bytes[i] = slice[i];
                }
                let v = u32::from_le_bytes(bytes);
                match valtype {
                    ValType::I32 => Result::i32val(v as u32),
                    ValType::I64 => Result::i64val(v as u64),
                    ValType::F32 => Result::f32val(v as f32),
                    ValType::F64 => Result::f64val(v as f64),
                }
            },
            64 => {
                let mut bytes = [0x00; 8];
                for i in 0..8 {
                    bytes[i] = slice[i];
                }
                let v = u64::from_le_bytes(bytes);
                match valtype {
                    ValType::I32 => Result::i32val(v as u32),
                    ValType::I64 => Result::i64val(v as u64),
                    ValType::F32 => Result::f32val(v as f32),
                    ValType::F64 => Result::f64val(v as f64),
                }
            },
            _ => unreachable!(),
        }
    }

    pub fn execute_store(&mut self, valtype: &ValType, memarg: &MemArg) -> ExecResult {
        self.execute_store_internal(valtype, memarg, Thread::valtype2usize(valtype))
    }
    pub fn execute_istore8(&mut self, valsize: &ValSize, memarg: &MemArg) -> ExecResult {
        self.execute_store_internal(&Thread::valsize2valtype(valsize), memarg, 8)
    }
    pub fn execute_istore16(&mut self, valsize: &ValSize, memarg: &MemArg) -> ExecResult {
        self.execute_store_internal(&Thread::valsize2valtype(valsize), memarg, 16)
    }
    pub fn execute_i64store32(&mut self, memarg: &MemArg) -> ExecResult {
        self.execute_store_internal(&ValType::I64, memarg, 32)
    }

    pub fn execute_store_internal(&mut self, valtype: &ValType, memarg: &MemArg, n: u32) -> ExecResult {
        let (_, frame) = self.current_frame();
        let memaddr = frame.module.memaddrs[0];
        let mem = &mut self.store.mems[memaddr];
        let c = if let Some(StackEntry::Value(Val::I32Const(c))) = self.stack.pop() {
            c
        } else {
            unreachable!()
        };

        let ea = c + memarg.offset;

        let ea = ea as usize;
        let max = ea + (n / 8) as usize;
        if max > mem.data.len() {
            return Result::Trap;
        }
        let slice = &mut mem.data[ea..max];

        match valtype {
            ValType::I32 => {
                let wrapped_n =
                if let Some(StackEntry::Value(Val::I32Const(v))) = self.stack.pop() {
                    v % 2u32.pow(n)
                } else {
                    unreachable!()
                };
                let bytes = wrapped_n.to_le_bytes();
                for i in 0..(n/8) as usize {
                    slice[i] = bytes[i];
                }
                Result::Vals(vec![])
            },
            ValType::I64 => {
                let wrapped_n = 
                if let Some(StackEntry::Value(Val::I64Const(v))) = self.stack.pop() {
                    v % 2u64.pow(n)
                } else {
                    unreachable!()
                };
                let bytes = wrapped_n.to_le_bytes();
                for i in 0..(n/8) as usize {
                    slice[i] = bytes[i];
                }
                Result::Vals(vec![])
            },
            ValType::F32 => {
                let n = 
                if let Some(StackEntry::Value(Val::F32Const(n))) = self.stack.pop() {
                    n
                } else {
                    unreachable!()
                };
                let bytes = n.to_le_bytes();
                for i in 0..4 {
                    slice[i] = bytes[i];
                }
                Result::Vals(vec![])
            },
            ValType::F64 => {
                let n = 
                if let Some(StackEntry::Value(Val::F64Const(n))) = self.stack.pop() {
                    n
                } else {
                    unreachable!()
                };
                let bytes = n.to_le_bytes();
                for i in 0..8 {
                    slice[i] = bytes[i];
                }
                Result::Vals(vec![])
            },
        }
    }

    fn valtype2usize(valtype: &ValType) -> u32 {
        match valtype {
            ValType::I32 | ValType::F32 => 32,
            ValType::I64 | ValType::F64 => 64,
        }
    }

    fn valsize2valtype(valsize: &ValSize) -> ValType {
        match valsize {
            ValSize::V32 => ValType::I32,
            ValSize::V64 => ValType::I64,
        }
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