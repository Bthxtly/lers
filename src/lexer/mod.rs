#![allow(unused)]
mod definition_token;
mod rule_token;
mod usercode_token;

pub use definition_token::DefinitionToken;
pub use rule_token::RuleToken;
pub use usercode_token::UsercodeToken;

use logos::Logos;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Definition(DefinitionToken<'a>),
    Rule(RuleToken<'a>),
    Ucode(UsercodeToken<'a>),
    RuleStart,
    RuleEnd,
}

pub struct Lexer<'a> {
    source: &'a str,
    section_idx: usize,
    section_lexers: [Box<dyn Iterator<Item = Result<Token<'a>, ()>> + 'a>; 3],
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        let sections = source.splitn(3, "%%").collect::<Vec<_>>();
        assert!(
            sections.len() == 3,
            "Input must have exactly two '%%' delimiters"
        );

        let definition_iter =
            DefinitionToken::lexer(sections[0]).map(|tok| tok.map(Token::Definition));
        let rule_iter = RuleToken::lexer(sections[1]).map(|tok| tok.map(Token::Rule));
        let ucode_iter = UsercodeToken::lexer(sections[2]).map(|tok| tok.map(Token::Ucode));

        Lexer {
            source,
            section_idx: 0,
            section_lexers: [
                Box::new(definition_iter),
                Box::new(rule_iter),
                Box::new(ucode_iter),
            ],
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.section_lexers[self.section_idx].next() {
            Some(tok) => Some(tok),
            None => {
                self.section_idx += 1;
                match self.section_idx {
                    1 => Some(Ok(Token::RuleStart)),
                    2 => Some(Ok(Token::RuleEnd)),
                    3 => None,
                    _ => unreachable!(),
                }
            }
        }
    }
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
        use Token::*;
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

/* user code */
void helper() {}"#;
        let mut lex = Lexer::new(source);

        token_eq!(lex, Definition(DefinitionToken::OptionStart));
        token_eq!(lex, Definition(DefinitionToken::Option("noyywrap")));
        token_match!(lex, Definition(DefinitionToken::CCode(_)));
        assert_eq!(lex.next(), Some(Ok(RuleStart)));
        token_eq!(lex, Rule(RuleToken::Pattern("pattern1")));
        token_eq!(lex, Rule(RuleToken::Action("{ action1(); }")));
        token_eq!(lex, Rule(RuleToken::Pattern("pattern2")));
        token_eq!(lex, Rule(RuleToken::Action("{ action2(); }")));
        token_eq!(lex, Rule(RuleToken::Pattern("pattern3")));
        token_eq!(lex, Rule(RuleToken::Action("{ action3(); }")));
        assert_eq!(lex.next(), Some(Ok(RuleEnd)));
        token_match!(lex, Ucode(UsercodeToken::CCode(_)));
        assert_eq!(lex.next(), None);
    }
}
