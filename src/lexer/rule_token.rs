use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ ]+")] // Ignore this regex pattern between tokens
#[logos(skip r"/\*.*?\*/")] // Ignore comments
pub enum RuleToken<'a> {
    #[regex(r"\n([^\s\[]|\[[^\]]+\])+", |lex| &lex.slice()[1..])]
    // any non-blank characters from start of a line
    Pattern(&'a str),

    #[regex(r"(?s)\{[^\}]*}", |lex| lex.slice())] // anything surrounded by bracket
    Action(&'a str),

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
        let rules = r#"
[0-9]+  {
            /* yytext is a string containing the matched text. */
            printf("Saw an integer: %s\n", yytext);
        }

.|\n    {   /* Ignore all other characters. */   }"#;
        let mut lex = RuleToken::lexer(rules);

        token_eq!(lex, RuleToken::Pattern("[0-9]+"));
        token_match!(lex, RuleToken::Action(_));
        token_eq!(lex, RuleToken::Newline);
        token_eq!(lex, RuleToken::Pattern(".|\\n"));
        token_match!(lex, RuleToken::Action(_));
    }
}
