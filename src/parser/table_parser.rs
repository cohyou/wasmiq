use crate::{
    TableType,
    Table,
    ElemType,
};
use super::*;

impl<R> Parser<R> where R: Read + Seek {

    pub fn parse_table(&mut self) -> Result<(), ParseError> {        
        // tabletype
        let table_type = self.parse_table_type()?;

        self.match_rparen()?;

        self.module.tables.push(Table(table_type));

        Ok(())
    }

    pub fn parse_table_type(&mut self) -> Result<TableType, ParseError> {
        let mut table_type = TableType(Limits::default(), ElemType::FuncRef);
        self.match_keyword(Keyword::Table)?;

        // table id
        parse_optional_id!(self, self.contexts[0].tables);

        // limits
        table_type.0 = self.parse_limits()?;

        // 'funcref'
        self.match_keyword(Keyword::FuncRef)?;
        
        Ok(table_type)
    }
}