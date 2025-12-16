use crate::ast::{
    CodeNode, DefinitionNode, OptionNode, Root, RuleNode, RulePairNode, UserCodeNode,
};
use crate::lexer::{DefinitionToken, Lexer, RuleToken, Token, UsercodeToken};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let lexer = Lexer::new(source);
        Parser {
            lexer,
            current_token: None,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next().and_then(|res| res.ok());
    }

    pub fn parse(&mut self) -> Root<'a> {
        let mut definition_node: Option<DefinitionNode<'_>> = None;
        let mut rule_node: Option<RuleNode<'_>> = None;
        let mut usercode_node: Option<UserCodeNode<'_>> = None;
        self.advance();

        while let Some(token) = &self.current_token {
            match token {
                Token::Definition(definition) => match definition {
                    DefinitionToken::OptionStart => {
                        if let Some(definition_node) = &mut definition_node {
                            definition_node.options = Some(self.parse_options());
                        } else {
                            definition_node = Some(DefinitionNode::default());
                            definition_node.as_mut().unwrap().options = Some(self.parse_options())
                        }
                    }
                    DefinitionToken::CCode(code) => {
                        if let Some(definition_node) = &mut definition_node {
                            definition_node.code = Some(CodeNode::from(*code));
                        } else {
                            definition_node = Some(DefinitionNode::default());
                            definition_node.as_mut().unwrap().code = Some(CodeNode::from(*code))
                        }
                        self.advance();
                    }
                    DefinitionToken::Option(_) => unreachable!("consumed by parse_option"),
                },
                Token::Rule(_) => rule_node = Some(self.parse_rules()),
                Token::Ucode(ucode) => match ucode {
                    UsercodeToken::CCode(code) => {
                        usercode_node = Some(UserCodeNode::from(*code));
                        self.advance();
                    }
                },
                Token::RuleStart => {
                    self.advance();
                }
                Token::RuleEnd => {
                    self.advance();
                }
            }
        }

        Root {
            definition_node,
            rule_node,
            usercode_node,
        }
    }

    fn parse_options(&mut self) -> Vec<OptionNode<'a>> {
        let mut options: Vec<OptionNode> = Vec::new();
        self.advance(); // skip the %option token
        while let Some(Token::Definition(DefinitionToken::Option(option))) = self.current_token {
            options.push(OptionNode::from(option));
            self.advance();
        }
        options
    }

    fn parse_rules(&mut self) -> RuleNode<'a> {
        let mut rules: Vec<RulePairNode<'a>> = Vec::new();
        while let Some(Token::Rule(RuleToken::Pattern(pattern))) = self.current_token {
            self.advance();
            if let Some(Token::Rule(RuleToken::Action(action))) = self.current_token {
                rules.push(RulePairNode { pattern, action });
                self.advance();
            } else {
                panic!("Expected action after pattern");
            }
        }
        RuleNode { rules: Some(rules) }
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

/* user code */
void helper() {}"#;
        let mut parser = Parser::new(source);
        let target_ast = Root {
            definition_node: Some(DefinitionNode {
                options: Some(vec![OptionNode { value: "noyywrap" }]),
                code: Some(CodeNode {
                    value: "    c code block",
                }),
                definitions: None,
            }),
            rule_node: Some(RuleNode {
                rules: Some(vec![
                    RulePairNode {
                        pattern: "pattern1",
                        action: "{ action1(); }",
                    },
                    RulePairNode {
                        pattern: "pattern2",
                        action: "{ action2(); }",
                    },
                    RulePairNode {
                        pattern: "pattern3",
                        action: "{ action3(); }",
                    },
                ]),
            }),
            usercode_node: Some(UserCodeNode {
                value: "\n\n/* user code */\nvoid helper() {}",
            }),
        };
        assert_eq!(parser.parse(), target_ast);
    }
}
