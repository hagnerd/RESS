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
pub enum SmallToken {
    KeywordCase,
    KeywordDelete,
    KeywordIn,
    KeywordInstanceOf,
    KeywordNew,
    KeywordReturn,
    KeywordThrow,
    KeywordTypeOf,
    KeywordVoid,
    KeywordFunction,
    KeywordAwait,
    KeywordBreak,
    KeywordCatch,
    KeywordClass,
    KeywordConst,
    KeywordContinue,
    KeywordDebugger,
    KeywordDefault,
    KeywordDo,
    KeywordElse,
    KeywordEnum,
    KeywordExport,
    KeywordFinally,
    KeywordFor,
    KeywordIf,
    KeywordImplements,
    KeywordImport,
    KeywordInterface,
    KeywordLet,
    KeywordPackage,
    KeywordPrivate,
    KeywordProtected,
    KeywordPublic,
    KeywordStatic,
    KeywordSuper,
    KeywordSwitch,
    KeywordThis,
    KeywordTry,
    KeywordVar,
    KeywordWhile,
    KeywordWith,
    KeywordYield,
    PunctCloseBrace,
    PunctCloseBracket,
    PunctCloseParen,
    PunctAmpersand,
    PunctAmpersandEqual,
    PunctAsterisk,
    PunctAsteriskEqual,
    PunctAtMark,
    PunctBang,
    PunctBangDoubleEqual,
    PunctBangEqual,
    PunctCaret,
    PunctCaretEqual,
    PunctColon,
    PunctComma,
    PunctDash,
    PunctDoubleDash,
    PunctDashEqual,
    PunctDoubleAmpersand,
    PunctDoubleAsterisk,
    PunctDoubleAsteriskEqual,
    PunctDoubleEqual,
    PunctDoubleGreaterThan,
    PunctDoubleGreaterThanEqual,
    PunctDoubleLessThan,
    PunctDoubleLessThanEqual,
    PunctDoublePipe,
    PunctDoublePlus,
    PunctEqual,
    PunctForwardSlash,
    PunctForwardSlashEqual,
    PunctGreaterThan,
    PunctGreaterThanEqual,
    PunctLessThan,
    PunctLessThanEqual,
    PunctOpenBrace,
    PunctOpenBracket,
    PunctOpenParen,
    PunctPercent,
    PunctPercentEqual,
    PunctPipe,
    PunctPipeEqual,
    PunctPlus,
    PunctPlusEqual,
    PunctQuestionMark,
    PunctSemiColon,
    PunctTilde,
    PunctTripleEqual,
    PunctTripleGreaterThanEqual,
    PunctTripleGreaterThan,
    PunctEllipsis,
    PunctEqualGreaterThan,
    PunctHash,
    PunctPeriod,
    Ident,
    Other,
}

impl From<&RawToken> for SmallToken {
    #[inline]
    fn from(other: &RawToken) -> Self {
        match other {
            RawToken::Keyword(k) => k.into(),
            RawToken::Punct(p) => p.into(),
            RawToken::Ident => SmallToken::Ident,
            _ => SmallToken::Other,
        }
    }
}

impl From<&Keyword> for SmallToken {
    #[inline]
    fn from(other: &Keyword) -> Self {
        match other {
            Keyword::Case => SmallToken::KeywordCase,
            Keyword::Delete => SmallToken::KeywordDelete,
            Keyword::In => SmallToken::KeywordIn,
            Keyword::InstanceOf => SmallToken::KeywordInstanceOf,
            Keyword::New => SmallToken::KeywordNew,
            Keyword::Return => SmallToken::KeywordReturn,
            Keyword::Throw => SmallToken::KeywordThrow,
            Keyword::TypeOf => SmallToken::KeywordTypeOf,
            Keyword::Void => SmallToken::KeywordVoid,
            Keyword::Function => SmallToken::KeywordFunction,
            Keyword::Await => SmallToken::KeywordAwait,
            Keyword::Break => SmallToken::KeywordBreak,
            Keyword::Catch => SmallToken::KeywordCatch,
            Keyword::Class => SmallToken::KeywordClass,
            Keyword::Const => SmallToken::KeywordConst,
            Keyword::Continue => SmallToken::KeywordContinue,
            Keyword::Debugger => SmallToken::KeywordDebugger,
            Keyword::Default => SmallToken::KeywordDefault,
            Keyword::Do => SmallToken::KeywordDo,
            Keyword::Else => SmallToken::KeywordElse,
            Keyword::Enum => SmallToken::KeywordEnum,
            Keyword::Export => SmallToken::KeywordExport,
            Keyword::Finally => SmallToken::KeywordFinally,
            Keyword::For => SmallToken::KeywordFor,
            Keyword::If => SmallToken::KeywordIf,
            Keyword::Implements => SmallToken::KeywordImplements,
            Keyword::Import => SmallToken::KeywordImport,
            Keyword::Interface => SmallToken::KeywordInterface,
            Keyword::Let => SmallToken::KeywordLet,
            Keyword::Package => SmallToken::KeywordPackage,
            Keyword::Private => SmallToken::KeywordPrivate,
            Keyword::Protected => SmallToken::KeywordProtected,
            Keyword::Public => SmallToken::KeywordPublic,
            Keyword::Static => SmallToken::KeywordStatic,
            Keyword::Super => SmallToken::KeywordSuper,
            Keyword::Switch => SmallToken::KeywordSwitch,
            Keyword::This => SmallToken::KeywordThis,
            Keyword::Try => SmallToken::KeywordTry,
            Keyword::Var => SmallToken::KeywordVar,
            Keyword::While => SmallToken::KeywordWhile,
            Keyword::With => SmallToken::KeywordWith,
            Keyword::Yield => SmallToken::KeywordYield,
        }
    }
}

impl From<&Punct> for SmallToken {
    #[inline]
    fn from(other: &Punct) -> Self {
        match other {
            Punct::CloseBrace => SmallToken::PunctCloseBrace,
            Punct::CloseBracket => SmallToken::PunctCloseBracket,
            Punct::CloseParen => SmallToken::PunctCloseParen,
            Punct::Ampersand => SmallToken::PunctAmpersand,
            Punct::AmpersandEqual => SmallToken::PunctAmpersandEqual,
            Punct::Asterisk => SmallToken::PunctAsterisk,
            Punct::AsteriskEqual => SmallToken::PunctAsteriskEqual,
            Punct::AtMark => SmallToken::PunctAtMark,
            Punct::Bang => SmallToken::PunctBang,
            Punct::BangDoubleEqual => SmallToken::PunctBangDoubleEqual,
            Punct::BangEqual => SmallToken::PunctBangEqual,
            Punct::Caret => SmallToken::PunctCaret,
            Punct::CaretEqual => SmallToken::PunctCaretEqual,
            Punct::Colon => SmallToken::PunctColon,
            Punct::Comma => SmallToken::PunctComma,
            Punct::Dash => SmallToken::PunctDash,
            Punct::DoubleDash => SmallToken::PunctDoubleDash,
            Punct::DashEqual => SmallToken::PunctDashEqual,
            Punct::DoubleAmpersand => SmallToken::PunctDoubleAmpersand,
            Punct::DoubleAsterisk => SmallToken::PunctDoubleAsterisk,
            Punct::DoubleAsteriskEqual => SmallToken::PunctDoubleAsteriskEqual,
            Punct::DoubleEqual => SmallToken::PunctDoubleEqual,
            Punct::DoubleGreaterThan => SmallToken::PunctDoubleGreaterThan,
            Punct::DoubleGreaterThanEqual => SmallToken::PunctDoubleGreaterThanEqual,
            Punct::DoubleLessThan => SmallToken::PunctDoubleLessThan,
            Punct::DoubleLessThanEqual => SmallToken::PunctDoubleLessThanEqual,
            Punct::DoublePipe => SmallToken::PunctDoublePipe,
            Punct::DoublePlus => SmallToken::PunctDoublePlus,
            Punct::Equal => SmallToken::PunctEqual,
            Punct::ForwardSlash => SmallToken::PunctForwardSlash,
            Punct::ForwardSlashEqual => SmallToken::PunctForwardSlashEqual,
            Punct::GreaterThan => SmallToken::PunctGreaterThan,
            Punct::GreaterThanEqual => SmallToken::PunctGreaterThanEqual,
            Punct::LessThan => SmallToken::PunctLessThan,
            Punct::LessThanEqual => SmallToken::PunctLessThanEqual,
            Punct::OpenBrace => SmallToken::PunctOpenBrace,
            Punct::OpenBracket => SmallToken::PunctOpenBracket,
            Punct::OpenParen => SmallToken::PunctOpenParen,
            Punct::Percent => SmallToken::PunctPercent,
            Punct::PercentEqual => SmallToken::PunctPercentEqual,
            Punct::Pipe => SmallToken::PunctPipe,
            Punct::PipeEqual => SmallToken::PunctPipeEqual,
            Punct::Plus => SmallToken::PunctPlus,
            Punct::PlusEqual => SmallToken::PunctPlusEqual,
            Punct::QuestionMark => SmallToken::PunctQuestionMark,
            Punct::SemiColon => SmallToken::PunctSemiColon,
            Punct::Tilde => SmallToken::PunctTilde,
            Punct::TripleEqual => SmallToken::PunctTripleEqual,
            Punct::TripleGreaterThanEqual => SmallToken::PunctTripleGreaterThanEqual,
            Punct::TripleGreaterThan => SmallToken::PunctTripleGreaterThan,
            Punct::Ellipsis => SmallToken::PunctEllipsis,
            Punct::EqualGreaterThan => SmallToken::PunctEqualGreaterThan,
            Punct::Hash => SmallToken::PunctHash,
            Punct::Period => SmallToken::PunctPeriod,
        }
    }
}

impl SmallToken {
    #[inline]
    pub fn is_punct(&self) -> bool {
        match self {
            SmallToken::PunctCloseBrace
            | SmallToken::PunctCloseBracket
            | SmallToken::PunctCloseParen
            | SmallToken::PunctAmpersand
            | SmallToken::PunctAmpersandEqual
            | SmallToken::PunctAsterisk
            | SmallToken::PunctAsteriskEqual
            | SmallToken::PunctAtMark
            | SmallToken::PunctBang
            | SmallToken::PunctBangDoubleEqual
            | SmallToken::PunctBangEqual
            | SmallToken::PunctCaret
            | SmallToken::PunctCaretEqual
            | SmallToken::PunctColon
            | SmallToken::PunctComma
            | SmallToken::PunctDash
            | SmallToken::PunctDoubleDash
            | SmallToken::PunctDashEqual
            | SmallToken::PunctDoubleAmpersand
            | SmallToken::PunctDoubleAsterisk
            | SmallToken::PunctDoubleAsteriskEqual
            | SmallToken::PunctDoubleEqual
            | SmallToken::PunctDoubleGreaterThan
            | SmallToken::PunctDoubleGreaterThanEqual
            | SmallToken::PunctDoubleLessThan
            | SmallToken::PunctDoubleLessThanEqual
            | SmallToken::PunctDoublePipe
            | SmallToken::PunctDoublePlus
            | SmallToken::PunctEqual
            | SmallToken::PunctForwardSlash
            | SmallToken::PunctForwardSlashEqual
            | SmallToken::PunctGreaterThan
            | SmallToken::PunctGreaterThanEqual
            | SmallToken::PunctLessThan
            | SmallToken::PunctLessThanEqual
            | SmallToken::PunctOpenBrace
            | SmallToken::PunctOpenBracket
            | SmallToken::PunctOpenParen
            | SmallToken::PunctPercent
            | SmallToken::PunctPercentEqual
            | SmallToken::PunctPipe
            | SmallToken::PunctPipeEqual
            | SmallToken::PunctPlus
            | SmallToken::PunctPlusEqual
            | SmallToken::PunctQuestionMark
            | SmallToken::PunctSemiColon
            | SmallToken::PunctTilde
            | SmallToken::PunctTripleEqual
            | SmallToken::PunctTripleGreaterThanEqual
            | SmallToken::PunctTripleGreaterThan
            | SmallToken::PunctEllipsis
            | SmallToken::PunctEqualGreaterThan
            | SmallToken::PunctHash
            | SmallToken::PunctPeriod => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_keyword(&self) -> bool {
        match self {
            SmallToken::KeywordCase
            | SmallToken::KeywordDelete
            | SmallToken::KeywordIn
            | SmallToken::KeywordInstanceOf
            | SmallToken::KeywordNew
            | SmallToken::KeywordReturn
            | SmallToken::KeywordThrow
            | SmallToken::KeywordTypeOf
            | SmallToken::KeywordVoid
            | SmallToken::KeywordFunction
            | SmallToken::KeywordAwait
            | SmallToken::KeywordBreak
            | SmallToken::KeywordCatch
            | SmallToken::KeywordClass
            | SmallToken::KeywordConst
            | SmallToken::KeywordContinue
            | SmallToken::KeywordDebugger
            | SmallToken::KeywordDefault
            | SmallToken::KeywordDo
            | SmallToken::KeywordElse
            | SmallToken::KeywordEnum
            | SmallToken::KeywordExport
            | SmallToken::KeywordFinally
            | SmallToken::KeywordFor
            | SmallToken::KeywordIf
            | SmallToken::KeywordImplements
            | SmallToken::KeywordImport
            | SmallToken::KeywordInterface
            | SmallToken::KeywordLet
            | SmallToken::KeywordPackage
            | SmallToken::KeywordPrivate
            | SmallToken::KeywordProtected
            | SmallToken::KeywordPublic
            | SmallToken::KeywordStatic
            | SmallToken::KeywordSuper
            | SmallToken::KeywordSwitch
            | SmallToken::KeywordThis
            | SmallToken::KeywordTry
            | SmallToken::KeywordVar
            | SmallToken::KeywordWhile
            | SmallToken::KeywordWith
            | SmallToken::KeywordYield => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_function(&self) -> bool {
        match self {
            SmallToken::KeywordFunction => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_conditional(&self) -> bool {
        match self {
            SmallToken::KeywordIf
            | SmallToken::KeywordFor
            | SmallToken::KeywordWhile
            | SmallToken::KeywordWith => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_ident(&self) -> bool {
        match self {
            SmallToken::Ident => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_this(&self) -> bool {
        match self {
            SmallToken::KeywordThis => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_close_bracket(&self) -> bool {
        match self {
            SmallToken::PunctCloseBracket => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_close_brace(&self) -> bool {
        match self {
            SmallToken::PunctCloseBrace => true,
            _ => false,
        }
    }
    #[inline]
    pub fn is_close_paren(&self) -> bool {
        match self {
            SmallToken::PunctCloseParen => true,
            _ => false,
        }
    }
    #[inline]
    pub fn indicates_expr(&self) -> bool {
        match self {
            SmallToken::PunctOpenParen
            | SmallToken::PunctOpenBrace
            | SmallToken::PunctOpenBracket
            | SmallToken::PunctEqual
            | SmallToken::PunctPlusEqual
            | SmallToken::PunctDashEqual
            | SmallToken::PunctAsteriskEqual
            | SmallToken::PunctDoubleAsteriskEqual
            | SmallToken::PunctForwardSlashEqual
            | SmallToken::PunctPercentEqual
            | SmallToken::PunctDoubleLessThanEqual
            | SmallToken::PunctDoubleGreaterThanEqual
            | SmallToken::PunctTripleGreaterThanEqual
            | SmallToken::PunctAmpersandEqual
            | SmallToken::PunctPipeEqual
            | SmallToken::PunctCaretEqual
            | SmallToken::PunctComma
            | SmallToken::PunctPlus
            | SmallToken::PunctDash
            | SmallToken::PunctAsterisk
            | SmallToken::PunctDoubleAsterisk
            | SmallToken::PunctForwardSlash
            | SmallToken::PunctPercent
            | SmallToken::PunctDoublePlus
            | SmallToken::PunctDoubleDash
            | SmallToken::PunctDoubleLessThan
            | SmallToken::PunctDoubleGreaterThan
            | SmallToken::PunctTripleGreaterThan
            | SmallToken::PunctAmpersand
            | SmallToken::PunctPipe
            | SmallToken::PunctCaret
            | SmallToken::PunctBang
            | SmallToken::PunctTilde
            | SmallToken::PunctDoubleAmpersand
            | SmallToken::PunctDoublePipe
            | SmallToken::PunctQuestionMark
            | SmallToken::PunctColon
            | SmallToken::PunctTripleEqual
            | SmallToken::PunctDoubleEqual
            | SmallToken::PunctGreaterThanEqual
            | SmallToken::PunctLessThanEqual
            | SmallToken::PunctLessThan
            | SmallToken::PunctGreaterThan
            | SmallToken::PunctBangEqual
            | SmallToken::PunctBangDoubleEqual
            | SmallToken::KeywordCase
            | SmallToken::KeywordDelete
            | SmallToken::KeywordIn
            | SmallToken::KeywordInstanceOf
            | SmallToken::KeywordNew
            | SmallToken::KeywordReturn
            | SmallToken::KeywordThrow
            | SmallToken::KeywordTypeOf
            | SmallToken::KeywordVoid => true,
            _ => false,
        }
    }
}

impl From<&RawToken> for MetaToken {
    #[inline]
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

