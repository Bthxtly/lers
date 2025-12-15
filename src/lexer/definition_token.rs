use logos::Lexer;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")] // Ignore this regex pattern between tokens
#[logos(skip r"/\*.*?\*/")] // Ignore comments
pub enum DefinitionToken<'a> {
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
        let definition = r#"
%option noyywrap
/*** Definition section ***/

%{
    /* C code to be copied verbatim */
    #include <stdio.h>
%}"#;
        let mut lex = DefinitionToken::lexer(definition);
        token_eq!(lex, DefinitionToken::OptionStart);
        token_eq!(lex, DefinitionToken::Option("noyywrap"));
        token_eq!(
            lex,
            DefinitionToken::CCode(
                "    /* C code to be copied verbatim */\n    #include <stdio.h>"
            )
        );
        assert_eq!(lex.next(), None);
    }
}
