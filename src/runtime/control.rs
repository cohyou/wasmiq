use crate::{
    FuncIdx,
    LabelIdx,
    BlockType,
};

use super::*;

impl<'a> Thread<'a> {
    pub fn execute_block(&mut self, blocktype: &BlockType, instrs: &Vec<Instr>) -> Result {
        let (_, frame) = self.current_frame();
        let (argtypes, returntypes) = blocktype.extend(&frame.module);
        let mut vals = vec![];
        for _ in 0..argtypes.len() {
            if let Some(StackEntry::Value(val)) = self.stack.pop() {
                vals.push(StackEntry::Value(val));
            } else {
                unreachable!();
            }
        }
        let label = StackEntry::Label(returntypes.len() as u32, vec![]);
        self.execute_instrs_with_label(label, instrs)
    }

    pub fn execute_loop(&mut self, blocktype: &BlockType, instrs: &Vec<Instr>) -> Result {
        let (_, frame) = self.current_frame();
        let (argtypes, _) = blocktype.extend(&frame.module);
        let mut vals = vec![];
        for _ in 0..argtypes.len() {
            if let Some(StackEntry::Value(val)) = self.stack.pop() {
                vals.push(StackEntry::Value(val));
            } else {
                unreachable!();
            }
        }
        let cont = vec![Instr::Loop(blocktype.clone(), instrs.clone())];
        let label = StackEntry::Label(argtypes.len() as u32, cont);
        self.execute_instrs_with_label(label, instrs)
    }

    pub fn execute_if(&mut self, blocktype: &BlockType, instrs1: &Vec<Instr>, instrs2: &Option<Vec<Instr>>) -> Result {
        let (_, frame) = self.current_frame();
        let (argtypes, returntypes) = blocktype.extend(&frame.module);

        let label = StackEntry::Label(returntypes.len() as u32, vec![]);

        let c = if let Some(StackEntry::Value(Val::I32Const(c))) = self.stack.pop() {
            c
        } else {
            unreachable!();
        };

        let mut vals = vec![];
        for _ in 0..argtypes.len() {
            if let Some(StackEntry::Value(val)) = self.stack.pop() {
                vals.push(StackEntry::Value(val));
            } else {
                unreachable!();
            }
        }

        if c == 0 {
            if let Some(instrs) = instrs2 {
                self.execute_instrs_with_label(label, instrs)
            } else {
                self.execute_instrs_with_label(label, &vec![])
            }
        } else {
            self.execute_instrs_with_label(label, instrs1)
        }
    }

    fn find_label(&mut self, labelidx: &LabelIdx) -> (u32, Vec<Instr>) {
        let mut cnt = 0;
        for entry in self.stack.iter().rev() {
            if let StackEntry::Label(arity, cont) = entry {
                if &cnt == labelidx {
                    return (arity.clone(), cont.clone());
                }
                cnt += 1;
            }
        }
        unreachable!()
    }

    pub fn execute_br(&mut self, labelidx: &LabelIdx) -> Result {
        let (n, cont) = self.find_label(labelidx);
        let mut vals = vec![];
        {
            for _ in 0..n.clone() {
                if let Some(val) = self.stack.pop() {
                    vals.push(val);
                } else {
                    unreachable!();
                }
            }
        }

        for _ in 0..labelidx.clone() {
            while let Some(StackEntry::Value(_)) = self.stack.pop() {}
            self.stack.pop();
        }

        self.stack.extend(vals);

        self.execute_instrs(&cont)
    }

    pub fn execute_brif(&mut self, labelidx: &LabelIdx) -> Result {
        if let Some(StackEntry::Value(Val::I32Const(c))) = self.stack.pop() {
            if c == 0 {
                Result::Vals(vec![])
            } else {
                self.execute_br(labelidx)
            }
        } else {
            unreachable!()
        }
    }

    pub fn execute_brtable(&mut self, labelindices: &Vec<LabelIdx>, labelidx: &LabelIdx) -> Result {
        if let Some(StackEntry::Value(Val::I32Const(i))) = self.stack.pop() {
            if (i as usize) < labelindices.len() {
                let l_i = labelindices[i as usize]; 
                self.execute_br(&l_i)
            } else {
                self.execute_br(labelidx)
            }
        } else {
            unreachable!()
        }
    }

    pub fn execute_return(&mut self) -> Result {
        let (n, _) = self.current_frame();

        let mut vals = vec![];
        for _ in 0..n {
            if let Some(StackEntry::Value(val)) = self.stack.pop() {
                vals.push(val);
            } else {
                unreachable!();
            }
        }
        loop {
            if let Some(StackEntry::Activation(_, _)) = self.stack.pop() {
                break;
            }
        }
        Result::Vals(vals)
    }

    pub fn execute_call(&mut self, funcidx: &FuncIdx) -> Result {
        let (_, frame) = self.current_frame();
        let a = frame.module.funcaddrs[funcidx.clone() as usize];
        self.execute_invoke(&a)
    }

    pub fn execute_callindirect(&mut self, funcidx: &FuncIdx) -> Result {
        let (_, frame) = self.current_frame();
        let ta = frame.module.tableaddrs[0];
        let table = &self.store.tables[ta];
        let ft_expect = &frame.module.types[funcidx.clone() as usize];
        if let Some(StackEntry::Value(Val::I32Const(i))) = self.stack.pop() {
            if (i as usize) < table.elem.len() { return Result::Trap; }
            if let Some(a) = table.elem[i as usize] {
                if let FuncInst::User(f) = &self.store.funcs[a] {
                    let ft_actual = &f.tp;
                    if ft_actual != ft_expect {
                        Result::Trap
                    } else {
                        self.execute_invoke(&a)
                    }
                } else {
                    unimplemented!()
                }
            } else {
                Result::Trap
            }
        } else {
            unreachable!()
        }
    }
}