use crate::{
    LocalIdx,
};

use super::*;

impl<'a> Thread<'a> {
    pub fn execute_localget(&mut self, _localidx: &LocalIdx) -> Result { 
        unimplemented!()
    }
    pub fn execute_localset(&mut self) -> Result { unimplemented!() }
    pub fn execute_localtee(&mut self) -> Result { unimplemented!() }
    pub fn execute_globalget(&mut self) -> Result { unimplemented!() }
    pub fn execute_globalset(&mut self) -> Result { unimplemented!() }
}