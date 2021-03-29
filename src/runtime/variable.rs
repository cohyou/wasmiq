use crate::{
    LocalIdx,
    GlobalIdx,
};

use super::*;

impl<'a> Thread<'a> {
    pub fn execute_localget(&mut self, localidx: &LocalIdx) -> Result {
        let (_, frame) = self.current_frame();
        let local_value = frame.locals[localidx.clone() as usize];
        Result::Vals(vec![local_value])
    }

    pub fn execute_localset(&mut self, localidx: &LocalIdx) -> Result {
        let (_, mut frame) = self.current_frame();
        if let Some(StackEntry::Value(val)) = self.stack.pop() {
            frame.locals[localidx.clone() as usize] = val;
            Result::Vals(vec![])
        } else {
            unreachable!()
        }
    }
    
    pub fn execute_localtee(&mut self, localidx: &LocalIdx) -> Result {
        let (_, mut frame) = self.current_frame();
        if let Some(StackEntry::Value(val)) = self.stack.pop() {
            frame.locals[localidx.clone() as usize] = val;
            Result::Vals(vec![val])
        } else {
            unreachable!()
        }
    }

    pub fn execute_globalget(&mut self, globalidx: &GlobalIdx) -> Result {
        let (_, frame) = self.current_frame();
        let addr = frame.module.globaladdrs[globalidx.clone() as usize];
        let globalinst = &self.store.globals[addr];
        Result::Vals(vec![globalinst.value])
    }

    pub fn execute_globalset(&mut self, globalidx: &GlobalIdx) -> Result {
        let (_, frame) = self.current_frame();
        if let Some(StackEntry::Value(val)) = self.stack.pop() {
            let addr = frame.module.globaladdrs[globalidx.clone() as usize];
            self.store.globals[addr].value = val;
            Result::Vals(vec![])
        } else {
            unreachable!()
        }
    }
}