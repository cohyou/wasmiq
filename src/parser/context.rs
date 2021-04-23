use std::fmt::Debug;
use crate::FuncType;

// テキストフォーマット独自のstruct
// 要するにシンボルテーブル
#[derive(Default, Clone)]
pub struct Context {
    pub types: Vec<Option<Id>>,
    pub funcs: Vec<Option<Id>>,
    pub tables: Vec<Option<Id>>,
    pub mems: Vec<Option<Id>>,
    pub globals: Vec<Option<Id>>,
    pub locals: Vec<Option<Id>>,
    pub labels: Vec<Option<Id>>,
    pub typedefs: Vec<FuncType>, // typedefs functype*
}

pub type Id = String;


impl Debug for Context {
fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
   writeln!(f, "{{")?;   
   if self.types.len() > 0 { writeln!(f, "  types: {:?}", self.types)?; }
   if self.funcs.len() > 0 {
      writeln!(f, "  funcs:")?;
      for func in &self.funcs {
         writeln!(f, "    {:?}", func)?;
      }
   }
   if self.tables.len() > 0 { writeln!(f, "  tables: {:?}", self.tables)?; }
   if self.mems.len() > 0 { writeln!(f, "  mems: {:?}", self.mems)?; }
   if self.globals.len() > 0 {
      writeln!(f, "  globals:")?;
      for global in &self.globals {
         writeln!(f, "    {:?}", global)?;
      }
   }
   if self.locals.len() > 0 { writeln!(f, "  locals: {:?}", self.locals)?; }
   if self.labels.len() > 0 { writeln!(f, "  labels: {:?}", self.labels)?; }
   if self.typedefs.len() > 0 {
      writeln!(f, "  typedefs:")?;
      for typedef in &self.typedefs {
         writeln!(f, "    {:?}", typedef)?;
      }
   }
   writeln!(f, "}}")        
}
}
