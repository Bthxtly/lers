use crate::ast::{
    CodeNode, DefinitionNode, DefinitionPairNode, OptionNode, Root, RuleNode, RulePairNode,
    UserCodeNode,
};
use crate::lexer::{DefinitionToken, Lexer, RuleToken, Token, UsercodeToken};
use std::collections::HashMap;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Option<Token<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
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
                Token::Definition(_) => definition_node = Some(self.parse_definitions()),
                Token::Rule(_) => rule_node = Some(self.parse_rules()),
                Token::Ucode(_) => usercode_node = Some(self.parse_usercode()),
            }
        }

        let mut root = Root {
            definition_node,
            rule_node,
            usercode_node,
        };

        Self::apply_definition(&mut root);

        root
    }

    // region: parse definition
    fn parse_definitions(&mut self) -> DefinitionNode<'a> {
        let mut definition_node = DefinitionNode::default();
        while let Some(Token::Definition(definition)) = &self.current_token {
            match definition {
                DefinitionToken::OptionStart => {
                    definition_node.options = Some(self.parse_options());
                }
                DefinitionToken::Name(_) => {
                    definition_node.definitions = Some(self.parse_definition_pairs());
                }
                DefinitionToken::CCode(code) => {
                    definition_node.code = Some(CodeNode::from(*code));
                    self.advance();
                }
                DefinitionToken::Newline => {
                    // ignore
                    self.advance();
                }
                DefinitionToken::Identifier(_) => unreachable!("consumed by parse_options"),
                DefinitionToken::Pattern(_) => unreachable!("consumed by parse_definition_pairs"),
            }
        }
        definition_node
    }

    fn parse_options(&mut self) -> Vec<OptionNode<'a>> {
        let mut options: Vec<OptionNode> = Vec::new();
        self.advance(); // skip the %option token
        while let Some(Token::Definition(DefinitionToken::Identifier(option))) = self.current_token
        {
            options.push(OptionNode::from(option));
            self.advance();
        }
        options
    }

    fn parse_definition_pairs(&mut self) -> Vec<DefinitionPairNode<'a>> {
        let mut definitions: Vec<DefinitionPairNode<'a>> = Vec::new();
        while let Some(Token::Definition(DefinitionToken::Name(name))) = self.current_token {
            self.advance();
            if let Some(Token::Definition(DefinitionToken::Pattern(definition))) =
                self.current_token
            {
                definitions.push(DefinitionPairNode { name, definition });
                self.advance();
            } else {
                panic!("Expected pattern after name");
            }
        }
        definitions
    }
    // endregion

    fn parse_rules(&mut self) -> RuleNode<'a> {
        let mut rules: Vec<RulePairNode<'a>> = Vec::new();
        loop {
            if let Some(Token::Rule(RuleToken::Newline)) = self.current_token {
                self.advance();
                continue;
            }
            if let Some(Token::Rule(RuleToken::Pattern(pattern))) = self.current_token {
                self.advance();
                if let Some(Token::Rule(RuleToken::Action(action))) = self.current_token {
                    self.advance();
                    rules.push(RulePairNode { pattern, action });
                    continue;
                } else {
                    panic!("Expected action after pattern");
                }
            }
            break;
        }
        RuleNode { rules: Some(rules) }
    }

    fn parse_usercode(&mut self) -> UserCodeNode<'a> {
        let mut usercode_node = UserCodeNode::default();
        while let Some(Token::Ucode(ucode)) = &self.current_token {
            match ucode {
                UsercodeToken::CCode(code) => {
                    usercode_node = UserCodeNode::from(*code);
                    self.advance();
                    self.advance();
                }
            }
        }
        usercode_node
    }

    // apply name-definition pairs in ast.definition_node.definitions to ast.rule_node.rules
    fn apply_definition(root: &mut Root) {
        // Build name -> definition map
        let definitions = if let Some(def_node) = &root.definition_node {
            if let Some(defs) = &def_node.definitions {
                let mut map = HashMap::new();
                for def in defs {
                    map.insert(def.name, def.definition);
                }
                map
            } else {
                return;
            }
        } else {
            return;
        };

        // Helper to recursively expand a pattern
        fn expand_pattern<'a>(pattern: &'a str, defs: &HashMap<&'a str, &'a str>) -> String {
            let mut result = String::from(pattern);
            let mut changed = true;
            while changed {
                changed = false;
                let mut new_result = String::new();
                let mut i = 0;
                while i < result.len() {
                    if result[i..].starts_with('{') {
                        if let Some(end) = result[i..].find('}') {
                            let name = &result[i + 1..i + end];
                            if let Some(def) = defs.get(name) {
                                new_result.push_str(def);
                                i += end + 1;
                                changed = true;
                                continue;
                            }
                        }
                    }
                    new_result.push(result.as_bytes()[i] as char);
                    i += 1;
                }
                result = new_result;
            }
            result
        }

        // apply to rules
        if let Some(rule_node) = &mut root.rule_node {
            if let Some(rules) = &mut rule_node.rules {
                for rule in rules.iter_mut() {
                    let expanded = expand_pattern(rule.pattern, &definitions);
                    rule.pattern = Box::leak(expanded.into_boxed_str());
                }
            }
        }
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

digit       [0-9]
number      {digit}+
name3       pattern3

%%

{digit}+    { action1(); }
{number}    { action2(); }
pattern3    { action3(); }

%%

/* user code */
void helper() {}"#;
        let lexer = Lexer::new(&source);
        let mut parser = Parser::new(lexer);
        let target_ast = Root {
            definition_node: Some(DefinitionNode {
                options: Some(vec![OptionNode { value: "noyywrap" }]),
                code: Some(CodeNode {
                    value: "    c code block",
                }),
                definitions: Some(vec![
                    DefinitionPairNode {
                        name: "digit",
                        definition: "[0-9]",
                    },
                    DefinitionPairNode {
                        name: "number",
                        definition: "{digit}+",
                    },
                    DefinitionPairNode {
                        name: "name3",
                        definition: "pattern3",
                    },
                ]),
            }),
            rule_node: Some(RuleNode {
                rules: Some(vec![
                    RulePairNode {
                        pattern: "[0-9]+",
                        action: "{ action1(); }",
                    },
                    RulePairNode {
                        pattern: "[0-9]+",
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
