#![allow(unused)]
mod auxiliary_token;
mod declaration_token;
mod rule_token;

pub use auxiliary_token::AuxiliaryToken;
pub use declaration_token::DeclarationToken;
pub use rule_token::RuleToken;

use logos::Logos;

#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Decl(DeclarationToken<'a>),
    Rule(RuleToken<'a>),
    Auxi(AuxiliaryToken<'a>),
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

        let decl_iter = DeclarationToken::lexer(sections[0]).map(|tok| tok.map(Token::Decl));
        let rule_iter = RuleToken::lexer(sections[1]).map(|tok| tok.map(Token::Rule));
        let auxi_iter = AuxiliaryToken::lexer(sections[2]).map(|tok| tok.map(Token::Auxi));

        Lexer {
            source,
            section_idx: 0,
            section_lexers: [
                Box::new(decl_iter),
                Box::new(rule_iter),
                Box::new(auxi_iter),
            ],
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, ()>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.section_lexers[self.section_idx].next() {
                Some(tok) => return Some(tok),
                None => {
                    self.section_idx += 1;
                    match self.section_idx {
                        1 => return Some(Ok(Token::RuleStart)),
                        2 => return Some(Ok(Token::RuleEnd)),
                        3 => return None,
                        _ => unreachable!(),
                    }
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

/* auxiliary code */
void helper() {}"#;
        let mut lex = Lexer::new(source);

        token_eq!(lex, Decl(DeclarationToken::OptionStart));
        token_eq!(lex, Decl(DeclarationToken::Option("noyywrap")));
        token_match!(lex, Decl(DeclarationToken::CCode(_)));
        assert_eq!(lex.next(), Some(Ok(RuleStart)));
        token_eq!(lex, Rule(RuleToken::Pattern("pattern1")));
        token_eq!(lex, Rule(RuleToken::Action("{ action1(); }")));
        token_eq!(lex, Rule(RuleToken::Pattern("pattern2")));
        token_eq!(lex, Rule(RuleToken::Action("{ action2(); }")));
        token_eq!(lex, Rule(RuleToken::Pattern("pattern3")));
        token_eq!(lex, Rule(RuleToken::Action("{ action3(); }")));
        assert_eq!(lex.next(), Some(Ok(RuleEnd)));
        token_match!(lex, Auxi(AuxiliaryToken::CCode(_)));
        assert_eq!(lex.next(), None);
    }
}
