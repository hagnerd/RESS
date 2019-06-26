use crate::tokenizer::RawToken;
use crate::tokens::{Keyword, Punct};

#[derive(Clone, Debug)]
pub struct LookBehind {
    list: [Option<SmallToken>; 3],
    pointer: u8,
}

impl LookBehind {
    #[inline]
    pub const fn new() -> Self {
        Self {
            list: [None, None, None],
            pointer: 0,
        }
    }
    #[inline]
    pub fn push(&mut self, tok: &RawToken) {
        if self.pointer >= 2 {
            self.pointer = 0;
        } else {
            self.pointer += 1;
        }
        self.list[self.pointer as usize] = Some(tok.into());
    }
    #[inline]
    pub fn last(&self) -> &Option<SmallToken> {
        &self.list[self.pointer as usize]
    }
    #[inline]
    pub fn two(&self) -> &Option<SmallToken> {
        if self.pointer == 0 {
            &self.list[2]
        } else {
            &self.list[(self.pointer - 1) as usize]
        }
    }
    #[inline]
    pub fn three(&self) -> &Option<SmallToken> {
        if self.pointer == 2 {
            &self.list[0]
        } else {
            &self.list[(self.pointer + 1) as usize]
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum MetaToken {
    Keyword(Keyword),
    Punct(Punct),
    Ident,
    Other,
}
#[derive(PartialEq, Clone, Copy, Debug)]
#[repr(transparent)]
pub struct SmallToken(u8);

impl From<&RawToken> for SmallToken {
    fn from(other: &RawToken) -> Self {
        let inner = match other {
            RawToken::Keyword(k) => k.into(),
            RawToken::Punct(p) => p.into(),
            RawToken::Ident => 254,
            _ => 255,
        };
        SmallToken(inner)
    }
}

impl PartialEq<u8> for SmallToken {
    fn eq(&self, other: &u8) -> bool {
        self.0.eq(other)
    }
}

impl PartialOrd<u8> for SmallToken {
    fn partial_cmp(&self, other: &u8) -> Option<::std::cmp::Ordering> {
        self.0.partial_cmp(other)
    }
}

impl ::std::ops::Deref for SmallToken {
    type Target = u8;
    fn deref(&self) -> &u8 {
        &self.0
    }
}

impl SmallToken {
    #[inline]
    pub const fn function() -> u8 {
        16
    }

    #[inline]
    pub fn is_conditional(&self) -> bool {
        if self.0 < 15 {
            false
        } else if self.0 == 15 {
            true
        } else if self.0 < 17 {
            false
        } else if self.0 == 17 {
            true
        } else if self.0 < 39 {
            false
        } else if self.0 < 41 {
            true
        } else {
            false
        }
    }
}

impl Into<u8> for &Keyword {
    fn into(self) -> u8 {
        match self {
            Keyword::Case => 0,
            Keyword::Delete => 1,
            Keyword::In => 2,
            Keyword::InstanceOf => 3,
            Keyword::New => 4,
            Keyword::Return => 5,
            Keyword::Throw => 6,
            Keyword::TypeOf => 7,
            Keyword::Void => 8,
            Keyword::Function => 9,
            Keyword::Await => 10,
            Keyword::Break => 11,
            Keyword::Catch => 12,
            Keyword::Class => 13,
            Keyword::Const => 14,
            Keyword::Continue => 15,
            Keyword::Debugger => 16,
            Keyword::Default => 17,
            Keyword::Do => 18,
            Keyword::Else => 19,
            Keyword::Enum => 20,
            Keyword::Export => 21,
            Keyword::Finally => 22,
            Keyword::For => 23,
            Keyword::If => 24,
            Keyword::Implements => 25,
            Keyword::Import => 26,
            Keyword::Interface => 27,
            Keyword::Let => 28,
            Keyword::Package => 29,
            Keyword::Private => 30,
            Keyword::Protected => 31,
            Keyword::Public => 32,
            Keyword::Static => 33,
            Keyword::Super => 34,
            Keyword::Switch => 35,
            Keyword::This => 36,
            Keyword::Try => 37,
            Keyword::Var => 38,
            Keyword::While => 39,
            Keyword::With => 40,
            Keyword::Yield => 41,
        }
    }
}

impl Into<u8> for &Punct {
    fn into(self) -> u8 {
        match self {
            Punct::CloseBrace => 64,
            Punct::CloseBracket => 65,
            Punct::CloseParen => 66,
            Punct::Ampersand => 67,
            Punct::AmpersandEqual => 68,
            Punct::Asterisk => 69,
            Punct::AsteriskEqual => 70,
            Punct::AtMark => 71,
            Punct::Bang => 72,
            Punct::BangDoubleEqual => 73,
            Punct::BangEqual => 74,
            Punct::Caret => 75,
            Punct::CaretEqual => 76,
            Punct::Colon => 77,
            Punct::Comma => 78,
            Punct::Dash => 79,
            Punct::DoubleDash => 80,
            Punct::DashEqual => 81,
            Punct::DoubleAmpersand => 82,
            Punct::DoubleAsterisk => 83,
            Punct::DoubleAsteriskEqual => 84,
            Punct::DoubleEqual => 85,
            Punct::DoubleGreaterThan => 86,
            Punct::DoubleGreaterThanEqual => 87,
            Punct::DoubleLessThan => 88,
            Punct::DoubleLessThanEqual => 89,
            Punct::DoublePipe => 90,
            Punct::DoublePlus => 91,
            Punct::Equal => 92,
            Punct::ForwardSlash => 93,
            Punct::ForwardSlashEqual => 94,
            Punct::GreaterThan => 95,
            Punct::GreaterThanEqual => 96,
            Punct::LessThan => 97,
            Punct::LessThanEqual => 98,
            Punct::OpenBrace => 99,
            Punct::OpenBracket => 100,
            Punct::OpenParen => 101,
            Punct::Percent => 102,
            Punct::PercentEqual => 103,
            Punct::Pipe => 104,
            Punct::PipeEqual => 105,
            Punct::Plus => 106,
            Punct::PlusEqual => 107,
            Punct::QuestionMark => 108,
            Punct::SemiColon => 109,
            Punct::Tilde => 110,
            Punct::TripleEqual => 111,
            Punct::TripleGreaterThanEqual => 112,
            Punct::TripleGreaterThan => 113,
            Punct::Ellipsis => 114,
            Punct::EqualGreaterThan => 115,
            Punct::Hash => 116,
            Punct::Period => 117,
        }
    }
}

impl From<&RawToken> for MetaToken {
    fn from(other: &RawToken) -> Self {
        match other {
            RawToken::Keyword(k) => MetaToken::Keyword(*k),
            RawToken::Punct(p) => MetaToken::Punct(*p),
            RawToken::Ident => MetaToken::Ident,
            _ => MetaToken::Other,
        }
    }
}



#[cfg(test)]
mod test {
    use super::*;
    use crate::tokens::Punct;

    #[test]
    fn six() {
        let first = RawToken::EoF;
        let second = RawToken::Ident;
        let third = RawToken::Keyword(Keyword::Function);
        let fourth = RawToken::Punct(Punct::Ampersand);
        let fifth = RawToken::Punct(Punct::Bang);
        let sixth = RawToken::Punct(Punct::Caret);
        let mut l = LookBehind::new();
        l.push(&first);
        test(&l, Some((&first).into()), None, None);
        l.push(&second);
        test(&l, Some((&second).into()), Some((&first).into()), None);
        l.push(&third);
        test(&l, Some((&third).into()), Some((&second).into()), Some((&first).into()));
        l.push(&fourth);
        test(&l, Some((&fourth).into()), Some((&third).into()), Some((&second).into()));
        l.push(&fifth);
        test(&l, Some((&fifth).into()), Some((&fourth).into()), Some((&third).into()));
        l.push(&sixth);
        test(&l, Some((&sixth).into()), Some((&fifth).into()), Some((&fourth).into()));
    }

    fn test(l: &LookBehind, first: Option<SmallToken>, second: Option<SmallToken>, third: Option<SmallToken>) {
        println!("{:?}", l);
        assert_eq!(l.last(), &first);
        assert_eq!(l.two(), &second);
        assert_eq!(l.three(), &third);
    }
}

