use crate::{GlobalIdx, LocalIdx};

use super::*;

impl<'a> Thread<'a> {
    pub fn execute_localget(&mut self, localidx: &LocalIdx) -> ExecResult {
        let (_, frame) = self.current_frame();
        if let Some(local_value) = frame.locals.get(localidx.clone() as usize) {
            ExecResult::Vals(vec![local_value.clone()])
        } else {
            panic!("execute_localget {:?} {:?}", localidx, self.stack);
        }
    }

    pub fn execute_localset(&mut self, localidx: &LocalIdx) -> ExecResult {
        let val = if let Some(StackEntry::Value(val)) = self.stack.pop() {
            val
        } else {
            unreachable!()
        };

        let (n, frame) = self.current_frame();
        let index = self.current_frame_index();
        let mut new_frame = frame.clone();
        new_frame.locals[localidx.clone() as usize] = val;
        self.stack[index] = StackEntry::Activation(n, new_frame);

        ExecResult::Vals(vec![])
    }

    pub fn execute_localtee(&mut self, localidx: &LocalIdx) -> ExecResult {
        let val = if let Some(StackEntry::Value(val)) = self.stack.pop() {
            val
        } else {
            unreachable!()
        };

        let (n, frame) = self.current_frame();
        let index = self.current_frame_index();
        let mut new_frame = frame.clone();
        new_frame.locals[localidx.clone() as usize] = val;
        self.stack[index] = StackEntry::Activation(n, new_frame);
        ExecResult::Vals(vec![val])
    }

    pub fn execute_globalget(&mut self, globalidx: &GlobalIdx) -> ExecResult {
        let (_, frame) = self.current_frame();
        let addr = frame.module.globaladdrs[globalidx.clone() as usize];
        let globalinst = &self.store.globals[addr];
        ExecResult::Vals(vec![globalinst.value])
    }

    pub fn execute_globalset(&mut self, globalidx: &GlobalIdx) -> ExecResult {
        let (_, frame) = self.current_frame();
        if let Some(StackEntry::Value(val)) = self.stack.pop() {
            let addr = frame.module.globaladdrs[globalidx.clone() as usize];
            self.store.globals[addr].value = val;
            ExecResult::Vals(vec![])
        } else {
            unreachable!()
        }
    }
}
