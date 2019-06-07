use ress::{
    RefToken,
    Boolean,
    Comment,
    Ident,
    Keyword,
    Number,
    Punct,
    RegEx,
    StringLit,
    Template,
};
lazy_static! {
    pub static ref ES5: Vec<RefToken<'static>> = vec![
        RefToken::Comment(Comment::new_multi_line("/* this file contains all grammatical productions in ECMA-262 edition 5.1 ** * **/")),
        RefToken::Comment(Comment::new_single_line("// whitespace")),
        RefToken::Ident("tab".into()),
        RefToken::Punct(Punct::Colon),
        RefToken::Keyword(Keyword::For),
        RefToken::Punct(Punct::OpenParen),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::CloseParen),
        RefToken::Keyword(Keyword::Break),
        RefToken::Ident("tab".into()),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Ident("verticalTab".into()),
        RefToken::Punct(Punct::Colon),
        RefToken::Keyword(Keyword::For),
        RefToken::Punct(Punct::OpenParen),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::CloseParen),
        RefToken::Keyword(Keyword::Break),
        RefToken::Ident("verticalTab".into()),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Ident("formFeed".into()),
        RefToken::Punct(Punct::Colon),
        RefToken::Keyword(Keyword::For),
        RefToken::Punct(Punct::OpenParen),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::CloseParen),
        RefToken::Keyword(Keyword::Break),
        RefToken::Ident("formFeed".into()),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Ident("space".into()),
        RefToken::Punct(Punct::Colon),
        RefToken::Keyword(Keyword::For),
        RefToken::Punct(Punct::OpenParen),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::CloseParen),
        RefToken::Keyword(Keyword::Break),
        RefToken::Ident("space".into()),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Ident("nbsp".into()),
        RefToken::Punct(Punct::Colon),
        RefToken::Keyword(Keyword::For),
        RefToken::Punct(Punct::OpenParen),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::CloseParen),
        RefToken::Keyword(Keyword::Break),
        RefToken::Ident("nbsp".into()),
        RefToken::Punct(Punct::SemiColon),
        
        RefToken::Ident("bom".into()),
        RefToken::Punct(Punct::Colon),
        RefToken::Keyword(Keyword::For),
        RefToken::Punct(Punct::OpenParen),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::CloseParen),
        RefToken::Keyword(Keyword::Break),
        RefToken::Ident("bom".into()),
        RefToken::Punct(Punct::SemiColon),
        
        RefToken::Comment(Comment::new_single_line("// line terminators")),
        RefToken::Ident("lineFeed".into()),
        RefToken::Punct(Punct::Colon),
        RefToken::Numeric("0".into()),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Ident("carriageReturn".into()),
        RefToken::Punct(Punct::Colon),
        RefToken::Numeric("0".into()),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::SemiColon),        

        RefToken::Ident("carriageReturnLineFeed".into()),
        RefToken::Punct(Punct::Colon),
        RefToken::Numeric("0".into()),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::SemiColon),        

        RefToken::Ident("lineSeparator".into()),
        RefToken::Punct(Punct::Colon),
        RefToken::Numeric("0".into()),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::SemiColon),        

        RefToken::Ident("paragraphSeparator".into()),
        RefToken::Punct(Punct::Colon),
        RefToken::Numeric("0".into()),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Comment(Comment::new_single_line("// identifier names")),
        RefToken::Keyword(Keyword::Var),
        RefToken::Ident("$".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("_".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident(r"\u0078".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("x$".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("x_".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident(r"x\u0030".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("xa".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("x0".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("x0a".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("x0123456789".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("qwertyuiopasdfghjklzxcvbnm".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("QWERTYUIOPASDFGHJKLZXCVBNM".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Comment(Comment::new_single_line("// a representative sample of unicode letters and numbers")),
        RefToken::Keyword(Keyword::Var),
        RefToken::Ident("œ一".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("ǻ둘".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("ɤ〩".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("φ".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("ﬁⅷ".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("ユニコード".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Ident("x‌‍".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Null,
        RefToken::Punct(Punct::SemiColon),
        RefToken::Boolean(Boolean::True),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Boolean(Boolean::False),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("00".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("1234567890".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("01234567".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("0.".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("0.00".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("10.00".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric(".0".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric(".00".into()),
        RefToken::Numeric("0e0".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("0E0".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("0.e0".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("0.00e+0".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric(".00e-0".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("0x0".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("0X0".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("0x0123456789abcdefABCDEF".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Numeric("2e308".into()),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::double("")),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::double("'")),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::double(r#"\'\"\\\b\f\n\r\t\v\0"#)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::double(r"\1\00\400\000")),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::double(r"\x01\x23\x45\x67\x89\xAB\xCD\xEF")),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::double(r"\u0123\u4567\u89AB\uCDEF")),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::double(r"\
")),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::single("")),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::single("\"")),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::single(r#"\'\"\\\b\f\n\r\t\v\0"#)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::single(r"\1\00\400\000")),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::single(r"\x01\x23\x45\x67\x89\xAB\xCD\xEF")),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::single(r"\u0123\u4567\u89AB\uCDEF")),
        RefToken::Punct(Punct::SemiColon),
        RefToken::String(StringLit::single(r"\
")),
        RefToken::Punct(Punct::SemiColon),

        RefToken::RegEx(RegEx::from_parts("x", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts("|", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts("|||", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(r"$\b\B", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts("(?=(?!(?:(.))))", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(r"a.\f\n\r\t\v\0\[\-\/\\\x00\u0000", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(r"\ca\cb\cc\cd\ce\cf\cg\ch\ci\cj\ck\cl\cm\cn\co\cp\cq\cr\cs\ct\cu\cv\cw\cx\cy\cz", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(r"\cA\cB\cC\cD\cE\cF\cG\cH\cI\cJ\cK\cL\cM\cN\cO\cP\cQ\cR\cS\cT\cU\cV\cW\cX\cY\cZ", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts("[a-z-]", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(r"[^\b\-^]", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(r"[/\]\\]", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".", Some("i"))),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".", Some("g"))),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".", Some("m"))),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".", Some("igm"))),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".*", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".*?", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".+", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".+?", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".?", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".??", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".{0}", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".{0,}", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts(".{0,0}", None)),
        RefToken::Punct(Punct::SemiColon),
        RefToken::RegEx(RegEx::from_parts("x", None)),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Keyword(Keyword::This),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Ident("x".into()),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Punct(Punct::OpenBracket),
        RefToken::Punct(Punct::CloseBracket),
        RefToken::Punct(Punct::SemiColon),
        
        RefToken::Punct(Punct::OpenBracket),
        RefToken::Punct(Punct::Comma),
        RefToken::Punct(Punct::CloseBracket),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Punct(Punct::OpenBracket),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::CloseBracket),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Punct(Punct::OpenBracket),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Punct(Punct::CloseBracket),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Punct(Punct::OpenBracket),
        RefToken::Punct(Punct::Comma),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::CloseBracket),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Punct(Punct::OpenBracket),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::CloseBracket),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Punct(Punct::OpenBracket),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::CloseBracket),
        RefToken::Punct(Punct::SemiColon),

        RefToken::Punct(Punct::OpenBracket),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Punct(Punct::CloseBracket),
        RefToken::Punct(Punct::SemiColon),
        RefToken::Punct(Punct::OpenBracket),
        
        RefToken::Punct(Punct::OpenBracket),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::Comma),
        RefToken::Punct(Punct::Comma),
        RefToken::Numeric("0".into()),
        RefToken::Punct(Punct::CloseBracket),
        RefToken::Punct(Punct::SemiColon),
        
        RefToken::Punct(Punct::OpenBracket),
        RefToken::Punct(Punct::Comma),
        RefToken::Punct(Punct::Comma),
        RefToken::Punct(Punct::CloseBracket),
        RefToken::Punct(Punct::SemiColon),

    ];
}