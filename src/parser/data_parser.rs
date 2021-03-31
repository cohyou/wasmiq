use crate::{
    Data,
};

use super::*;

impl<R> Parser<R> where R: Read + Seek {
    pub(super) fn parse_data(&mut self) -> Result<(), ParseError> {        
        self.match_keyword(Keyword::Data)?;

        // mem id
        let memidx = match &self.lookahead {
            nm!(Number::Integer(_)) |
            tk!(TokenKind::Id(_)) => {
                self.resolve_id(&self.contexts[0].mems.clone())?
            },
            _ => 0, 
        };

        // offset
        let offset = if self.is_lparen()? {
            self.match_lparen()?;
            self.parse_offset()?
        } else {
            let instr = self.parse_instr()?;
            Expr(vec![instr])
        };

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

