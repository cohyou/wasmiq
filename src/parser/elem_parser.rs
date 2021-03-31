use crate::{
    Elem,
};

use super::*;

impl<R> Parser<R> where R: Read + Seek {
    pub(super) fn parse_elem(&mut self) -> Result<(), ParseError> {        
        self.match_keyword(Keyword::Elem)?;

        // table id
        let tableidx = match &self.lookahead {
            nm!(Number::Integer(_)) |
            tk!(TokenKind::Id(_)) => {
                self.resolve_id(&self.contexts[0].tables.clone())?
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

        self.module.elem.push(elem);

        Ok(())
    }
}

