use logos::Lexer;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ ]+")] // Ignore this regex pattern between tokens
#[logos(skip r"/\*.*?\*/")] // Ignore comments
pub enum DefinitionToken<'a> {
    #[token("%option")]
    OptionStart,

    #[regex(r"[A-Za-z]+", |lex| lex.slice())]
    Identifier(&'a str),

    #[regex(r"(?s)%\{\n.*?\n%}", |lex| &lex.slice()[3..lex.slice().len()-3])]
    // skip the %{\n and \n%} and trim whitespace
    CCode(&'a str),

    #[regex(r"\n[A-Za-z_][A-Za-z0-9_]*", |lex| &lex.slice()[1..])] // skip the leading newline
    Name(&'a str),

    #[regex(r"([^\s\[]|\[[^\]]+\])+", |lex| lex.slice(), priority = 1)]
    Pattern(&'a str),

    #[token("\n")]
    Newline,
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
%}

digit   [0-9]
number  {digit}+"#;
        let mut lex = DefinitionToken::lexer(definition);
        token_eq!(lex, DefinitionToken::Newline);
        token_eq!(lex, DefinitionToken::OptionStart);
        token_eq!(lex, DefinitionToken::Identifier("noyywrap"));
        token_eq!(lex, DefinitionToken::Newline);
        token_eq!(lex, DefinitionToken::Newline);
        token_eq!(lex, DefinitionToken::Newline);
        let code = "    /* C code to be copied verbatim */\n    #include <stdio.h>";
        token_eq!(lex, DefinitionToken::CCode(code));
        token_eq!(lex, DefinitionToken::Newline);
        token_eq!(lex, DefinitionToken::Name("digit"));
        token_eq!(lex, DefinitionToken::Pattern("[0-9]"));
        token_eq!(lex, DefinitionToken::Name("number"));
        token_eq!(lex, DefinitionToken::Pattern("{digit}+"));
        assert_eq!(lex.next(), None);
    }
}
