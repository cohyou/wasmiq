use crate::{
    Data,
};

use super::*;

impl<R> Parser<R> where R: Read + Seek {
    pub(super) fn parse_data(&mut self) -> Result<(), ParseError> {        
        self.match_keyword(Keyword::Data)?;

        // mem id
        let memidx = self.resolve_id(&self.contexts[0].mems.clone())?;

        // offset
        self.match_lparen()?;
        let offset = self.parse_offset()?;

        // data string
        let datastring = self.parse_data_string()?;

        let data = Data {
            data: memidx, 
            offset: offset, 
            init: datastring
        };

        self.module.data.push(data);

        Ok(())
    }

}

