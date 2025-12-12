use logos::Lexer;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
#[logos(skip r"/\*.*?\*/")] // Ignore comments
pub enum DeclarationToken<'a> {
    #[token("%option")]
    OptionStart,

    #[regex(r"[^\s]+", |lex| lex.slice())]
    Option(&'a str),

    #[regex(r"(?s)%\{\n.*?\n%}", |lex| &lex.slice()[3..lex.slice().len()-3])]
    // skip the %{\n and \n%} and trim whitespace
    CCode(&'a str),
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! token_eq {
        ($lexer:expr, $expected:expr) => {
            assert_eq!($lexer.next(), Some(Ok($expected)));
        };
    }

    macro_rules! token_match {
        ($lexer:expr, $pattern:pat) => {
            assert!(matches!($lexer.next(), Some(Ok($pattern))));
        };
    }

    #[test]
    fn tokenize() {
        let decl = r#"
%option noyywrap
/*** Definition section ***/

%{
    /* C code to be copied verbatim */
    #include <stdio.h>
%}"#;
        let mut lex = DeclarationToken::lexer(decl);
        token_eq!(lex, DeclarationToken::OptionStart);
        token_eq!(lex, DeclarationToken::Option("noyywrap"));
        token_eq!(
            lex,
            DeclarationToken::CCode(
                "    /* C code to be copied verbatim */\n    #include <stdio.h>"
            )
        );
        assert_eq!(lex.next(), None);
    }
}
