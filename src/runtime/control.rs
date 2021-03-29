use super::*;

impl<'a> Thread<'a> {
    pub fn execute_block(&mut self) -> Result { unimplemented!() }
    pub fn execute_loop(&mut self) -> Result { unimplemented!() }
    pub fn execute_if(&mut self) -> Result { unimplemented!() }

    pub fn execute_br(&mut self) -> Result { unimplemented!() }
    pub fn execute_brif(&mut self) -> Result { unimplemented!() }
    pub fn execute_brtable(&mut self) -> Result { unimplemented!() }
    pub fn execute_return(&mut self) -> Result { unimplemented!() }
    pub fn execute_call(&mut self) -> Result { unimplemented!() }
    pub fn execute_callindirect(&mut self) -> Result { unimplemented!() }
}