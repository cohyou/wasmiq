use crate::{
    FuncType,
};

// use crate::instr::*;
use super::*;

impl<R> Parser<R> where R: Read + Seek {
    pub(super) fn parse_type(&mut self) -> Result<(), ParseError> {

        self.match_keyword(Keyword::Type)?;

        // type id
        // parse_optional_id!(self, self.contexts[0].types);
        if let tk!(TokenKind::Id(s)) = &self.lookahead {
            let new_s = s.clone();
            self.contexts[0].types.push(Some(Id::Named(new_s)));
            self.consume()?;
        } else if let tk!(TokenKind::GenSym(idx)) = &self.lookahead {
            self.contexts[0].types.push(Some(Id::Anonymous(idx.clone())));
            self.consume()?;
        } else {
            self.contexts[0].types.push(None);
        }

        // functype
        self.match_lparen()?;
        let functype = self.parse_functype()?;

        self.module.types.push(functype.clone());
        self.contexts[0].typedefs.push(functype);

        self.match_rparen()?;

        Ok(())
    }

    fn parse_functype(&mut self) -> Result<FuncType, ParseError> {
        let mut functype = FuncType::default();

        self.match_keyword(Keyword::Func)?;

        if !self.is_rparen()? {
            self.parse_signature(&mut functype.0, &mut functype.1)?;
        }        

        self.match_rparen()?;

        Ok(functype)
    }
}