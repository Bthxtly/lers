use crate::lexer::{AuxiliaryToken, DeclarationToken, Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(source),
        }
    }
}

impl Iterator for Parser<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        for token in &mut self.lexer {
            match token.unwrap() {
                Token::Decl(decl) => match decl {
                    DeclarationToken::OptionStart => {} // do nothing for now
                    DeclarationToken::Option(_) => {}   // do nothing for now
                    DeclarationToken::CCode(code) => {
                        return Some(code.to_string());
                    }
                },
                Token::Rule(_rule) => {
                    // skip for now
                }
                Token::Auxi(aux) => match aux {
                    AuxiliaryToken::CCode(code) => {
                        return Some(code.to_string());
                    }
                },
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let source = r#"
            %option noyywrap
            /* comment */
            %{
                c code block
            %}

            %%

            pattern1    { action1(); }
            pattern2    { action2(); }
            pattern3    { action3(); }

            %%

            /* auxiliary code */
            void helper() {}
            "#;
        let mut parser = Parser::new(source);
        // declaration
        assert_eq!(
            parser.next().as_deref(),
            Some("\n                c code block\n            ")
        );

        // rule
        // skipped

        // auxiliary
        assert_eq!(
            parser.next().as_deref(),
            Some(
                "\n\n            /* auxiliary code */\n            void helper() {}\n            "
            )
        );

        // end
        assert_eq!(parser.next(), None);
    }
}
