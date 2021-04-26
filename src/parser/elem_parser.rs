use crate::{
    Elem,
};

use super::*;

impl<R> Parser<R> where R: Read + Seek {
    pub(super) fn parse_elem(&mut self) -> Result<(), ParseError> {        
        self.match_keyword(Keyword::Elem)?;

        // table id
        let tableidx = self.resolve_id(&self.contexts[0].tables.clone())?;

        // offset
        self.match_lparen()?;
        let offset = self.parse_offset()?;

        // func indices
        let mut func_indices = vec![];

        loop {
            if let tk!(TokenKind::RightParen) = self.lookahead {
                self.match_rparen()?;
                break;
            }

            let funcidx = self.resolve_id(&self.contexts[0].funcs.clone())?;
            func_indices.push(funcidx);            
        }

        let elem = Elem {
            table: tableidx, 
            offset: offset, 
            init: func_indices
        };
        p!(elem);

        self.module.elem.push(elem);

        Ok(())
    }
}

