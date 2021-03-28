use crate::{
    Export,
    ExportDesc,
};

use super::*;

impl<R> Parser<R> where R: Read + Seek {

    pub(super) fn parse_export(&mut self) -> Result<(), ParseError> {
        self.match_keyword(Keyword::Export)?;

        // name
        let export_name = self.parse_name()?;

        self.match_lparen()?;

        // export desc
        let export_desc = self.parse_export_desc()?;

        self.module.exports.push(Export{ name: export_name, desc: export_desc });

        self.match_rparen()?;

        Ok(())
    }

    fn parse_export_desc(&mut self) -> Result<ExportDesc, ParseError> {
        match self.lookahead {
            kw!(Keyword::Func) => self.parse_export_desc_func(),
            kw!(Keyword::Table) => self.parse_export_desc_table(),
            kw!(Keyword::Memory) => self.parse_export_desc_memory(),
            kw!(Keyword::Global) => self.parse_export_desc_global(),
            _ => Err(self.err())
        }
    }

    fn parse_export_desc_func(&mut self) -> Result<ExportDesc, ParseError> {        
        self.match_keyword(Keyword::Func)?;

        // func id
        let funcidx = self.resolve_id(&self.contexts[0].funcs.clone())?;  

        self.match_rparen()?;

        Ok(ExportDesc::Func(funcidx))
    }

    fn parse_export_desc_table(&mut self) -> Result<ExportDesc, ParseError> {
         self.match_keyword(Keyword::Table)?;

        // table id
        let tableidx = self.resolve_id(&self.contexts[0].tables.clone())?;        

        self.match_rparen()?;

        Ok(ExportDesc::Table(tableidx))
    }

    fn parse_export_desc_memory(&mut self) -> Result<ExportDesc, ParseError> { 
        self.match_keyword(Keyword::Memory)?;       

        // memory id
        let memidx = self.resolve_id(&self.contexts[0].mems.clone())?;        

        self.match_rparen()?;

        Ok(ExportDesc::Mem(memidx))
    }

    fn parse_export_desc_global(&mut self) -> Result<ExportDesc, ParseError> {
        self.match_keyword(Keyword::Global)?;               

        // global id
        let globalidx = self.resolve_id(&self.contexts[0].globals.clone())?;  

        self.match_rparen()?;
        
        Ok(ExportDesc::Global(globalidx))
    }
}
