#[allow(unused_macros)]
macro_rules! la {    
    ($this:ident) => {
        p!($this.lookahead);
    };
}

#[allow(unused_macros)]
macro_rules! lla {    
    ($i:expr, $this:ident) => {
        pp!($i, $this.lookahead);
    };
}

#[macro_export]
macro_rules! tk { ($kind:pat) => { Annot{value: $kind, ..} } }

macro_rules! kw { ($kw:pat) => {
    Annot{value: TokenKind::Keyword($kw), ..}
} }

macro_rules! nm { ($nm:pat) => {
    Annot{value: TokenKind::Number($nm), ..}
} }

macro_rules! instr { ($instr:pat) => {
    Annot{ value: TokenKind::Keyword(Keyword::Instr($instr)), .. }
} }

macro_rules! parse_optional_id {
    ($this:ident, $v:expr) => {
        if let tk!(TokenKind::Id(_s)) = &$this.lookahead {
            // let new_s = s.clone();
            // $v.push(Some(Id::Named(new_s)));
            $this.consume()?;
        } else if let tk!(TokenKind::GenSym(_idx)) = &$this.lookahead {
            // $v.push(Some(Id::Anonymous(idx.clone())));
            $this.consume()?;
        } else {
            // $v.push(None);
        }
    }
}

macro_rules! parse_optional_label_id {
    ($this:ident, $v:expr) => {
        if let tk!(TokenKind::Id(s)) = &$this.lookahead {
            let new_s = s.clone();
            $v.insert(0, Some(Id::Named(new_s)));
            $this.consume()?;
        } else {
            $v.push(None);
        }
    }
}

macro_rules! parse_field {
    ($this:ident, $field_type:ident, $f:expr) => {
        if !$this.is_rparen()? {            
            if let tk!(TokenKind::LeftParen) = $this.lookahead {
                $this.consume()?;
            }
            loop {
                if let kw!(Keyword::$field_type) = &$this.lookahead {
                    { $f }
                    if let tk!(TokenKind::LeftParen) = $this.lookahead {
                        let peeked = $this.peek()?;
                        if let kw!(Keyword::$field_type) = peeked {
                            $this.consume()?;

                            continue;
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
    };
}