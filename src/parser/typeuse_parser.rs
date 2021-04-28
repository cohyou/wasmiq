use crate::{
    TypeIdx,
    FuncType,
};

// use crate::instr::*;
use super::*;

impl<R> Parser<R> where R: Read + Seek {
    pub fn parse_typeuse(&mut self, params: &mut Vec<ValType>, results: &mut Vec<ValType>) -> Result<TypeIdx, ParseError> {
        self.match_lparen()?;
        let typeidx = self.parse_typeuse_typeidx(params, results)?;

        if !self.is_rparen()? {
            self.parse_signature(params, results)?;
        }
        Ok(typeidx)
    }

    pub fn parse_signature(&mut self, params: &mut Vec<ValType>, results: &mut Vec<ValType>) -> Result<(), ParseError> {

        // params        
        parse_field!(self, Param, 
        if let Ok(param_vt) = self.parse_param() {
            params.push(param_vt);
        });

        // result        
        parse_field!(self, Result, 
        if let Ok(result_vt) = self.parse_result() {
            results.push(result_vt);
        });

        Ok(())
    }

    pub fn check_typeuse(&mut self, typeidx: TypeIdx, tp: FuncType) -> Result<(), ParseError> {
        let typedef = &self.contexts[0].typedefs[typeidx as usize];
        if tp.0.len() == 0 && tp.1.len() == 0 { return Ok(()) }
        if typedef != &tp {
            Err(ParseError::InvalidTypeuseDef(self.lookahead.clone(), typedef.clone(), tp))
        } else {
            Ok(())
        }
    }

    pub fn parse_typeuse_typeidx(&mut self, params: &mut Vec<ValType>, results: &mut Vec<ValType>) -> Result<TypeIdx, ParseError> {
        self.match_keyword(Keyword::Type)?;

        let mut typeidx = self.contexts[0].typedefs.len() as u32;

        if let tk!(TokenKind::GenSym(index)) = self.lookahead {
            match self.resolve_id(&self.contexts[0].types.clone()) {
                Ok(tpidx) => { typeidx = tpidx; },
                Err(ParseError::CantResolveId(_)) => { self.consume()?; },
                Err(error) => return Err(error), 
            }

            self.match_rparen()?;
    
            if !self.is_rparen()? {
                self.parse_signature(params, results)?;
            }
    
            for (i, functype) in self.contexts[0].typedefs.iter().enumerate() {
                if &functype.0 == params && &functype.1 == results {
                    typeidx = i as u32;
                }
            }

            if typeidx == self.contexts[0].typedefs.len() as u32 {
                let ft = (params.clone(), results.clone());
                self.contexts[0].types.push(Some(Id::Anonymous(index)));
                self.module.types.push(ft.clone());
                self.contexts[0].typedefs.push(ft.clone());
            }

        } else {
            typeidx = self.resolve_id(&self.contexts[0].types.clone())?;

            self.match_rparen()?;
        };

        Ok(typeidx)
    }

    pub(super) fn parse_param(&mut self) -> Result<ValType, ParseError> {

        self.match_keyword(Keyword::Param)?;

        // param id        
        let len = self.contexts.len();
        if let tk!(TokenKind::Id(s)) = &self.lookahead {            
            if len > 1 {
                let new_s = s.clone();                
                self.contexts.last_mut().unwrap().locals.push(Some(Id::Named(new_s)));
            }
            self.consume()?;
        } else {
            if len > 1 {
                self.contexts.last_mut().unwrap().locals.push(None);
            }
        }

        // valtype
        let vt = self.parse_valtype()?;

        self.match_rparen()?;

        Ok(vt)
    }

    pub(super) fn parse_result(&mut self) -> Result<ValType, ParseError> {

        self.match_keyword(Keyword::Result)?;

        // valtype
        let vt = self.parse_valtype()?;

        self.match_rparen()?;

        Ok(vt)
    }
}