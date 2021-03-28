use crate::{
    Mem,
    MemType,
};

use super::*;

impl<R> Parser<R> where R: Read + Seek {

    pub(super) fn parse_memory(&mut self) -> Result<(), ParseError> {
        // memtype
        let mem_type = self.parse_memory_type()?;

        self.module.mems.push(Mem(mem_type));        

        Ok(())
    }

    pub(super) fn parse_memory_type(&mut self) -> Result<MemType, ParseError> {

        self.match_keyword(Keyword::Memory)?;

        // mem id
        parse_optional_id!(self, self.contexts[0].mems);

        let limits = self.parse_limits()?;

        self.match_rparen()?;

        Ok(MemType(limits))
    }
}