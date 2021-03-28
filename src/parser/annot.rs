use std::fmt::Debug;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub struct Loc(pub usize, pub usize);

impl Loc {
    // pub fn merge(&self, other: &Loc) -> Loc {
    //     use std::cmp::{max, min};
    //     Loc(min(self.0, other.0), max(self.1, other.1))
    // }
    pub fn newline(&mut self) { self.0 += 1; self.1 = 0; }
    pub fn add_pos(&mut self) { self.1 += 1; }
    pub fn added(&self, offset: usize) -> Loc {
        Loc(self.0, self.1 + offset)
    }
}

impl Default for Loc {
    fn default() -> Self { Loc(1, 0) }
}

impl Debug for Loc {
   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.0, self.1)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Annot<T> {
    pub value: T,
    pub loc: Loc,
}

impl<T> Annot<T> {
    pub fn new(value: T, loc: Loc) -> Self {
        Self { value, loc }
    }
}
