use crate::{
    Global,
    GlobalType,
    Mut,
};

use super::*;

impl<R> Parser<R> where R: Read + Seek {

    pub fn parse_global(&mut self) -> Result<(), ParseError> {
        let global_type = self.parse_global_type()?;

        let expr = self.parse_expr()?;
        
        self.module.globals.push(Global{ tp: global_type, init: expr });

        self.match_rparen()?;

        Ok(())
    }

    pub fn parse_global_type(&mut self) -> Result<GlobalType, ParseError> {        
        self.match_keyword(Keyword::Global)?;

        // global id
        let _id = self.parse_optional_id_global()?;
        // self.contexts[0].globals.push(id);
        
        // mutablity
        let mut mutablity = Mut::Const;

        // valtype
        let vt = if self.is_lparen()? {
            self.match_lparen()?;
            self.match_keyword(Keyword::Mutable)?;
            mutablity = Mut::Var;
            let vt = self.parse_valtype()?;
            self.match_rparen()?;
            vt 
        } else {
            self.parse_valtype()?
        };

        let global_type = GlobalType(vt, mutablity);

        Ok(global_type)
    }

    fn parse_optional_id_global(&mut self) -> Result<Option<Id>, ParseError> {
        if let tk!(TokenKind::Id(s)) = &self.lookahead {
            let id = Ok(Some(Id::Named(s.clone())));
            self.consume()?;
            id
        } else if let tk!(TokenKind::GenSym(idx)) = &self.lookahead {
            let id = Ok(Some(Id::Anonymous(idx.clone())));
            self.consume()?;
            id
        } else {
            Ok(None)
        }
    }
}