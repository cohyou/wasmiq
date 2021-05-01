use crate::{
    Mem,
    MemType,
};

use super::*;

impl<R> Parser<R> where R: Read + Seek {

    pub fn parse_memory(&mut self) -> Result<(), ParseError> {
        // memtype
        let mem_type = self.parse_memory_type()?;

        self.match_rparen()?;

        self.module.mems.push(Mem(mem_type));        

        Ok(())
    }

    pub fn parse_memory_type(&mut self) -> Result<MemType, ParseError> {

        self.match_keyword(Keyword::Memory)?;

        // mem id
        parse_optional_id!(self, self.contexts[0].mems);

        let limits = self.parse_limits()?;

        Ok(MemType(limits))
    }
}