mod code;
mod option;

use crate::ast::*;
use code::*;

#[derive(Default, Debug, PartialEq)]
struct RuleTable<'a> {
    pair_count: usize,
    patterns: Vec<&'a str>,
    actions: Vec<&'a str>,
}

impl<'a> RuleTable<'a> {
    pub fn append_pair(&mut self, pattern: &'a str, action: &'a str) {
        self.pair_count += 1;
        self.patterns.push(pattern);
        self.actions.push(action);
    }
}

pub struct CodeGen<'a> {
    ast: Root<'a>,
    options: Vec<option::Option>,
    rule_table: RuleTable<'a>,
}

impl<'a> CodeGen<'a> {
    pub fn new(ast: Root<'a>) -> Self {
        CodeGen {
            ast,
            options: Vec::new(),
            rule_table: RuleTable::default(),
        }
    }

    pub fn generate(&mut self) -> String {
        let mut code = String::new();

        // Generate code from definition node
        if let Some(def_node) = &self.ast.definition_node {
            if let Some(code_node) = &def_node.code {
                code.push_str(&format!("/*** Definition Code ***/\n{}\n", code_node.value));
            }
            self.apply_options();
        }

        // Visit rule node and generate rule table
        if let Some(rule_node) = &self.ast.rule_node {
            if let Some(rules) = rule_node.rules.as_ref() {
                for pair in rules {
                    self.rule_table.append_pair(pair.pattern, pair.action);
                }
            }
            code.push_str(&format!("/*** Rule Code ***/\n{}\n", &self.gen_rule_code()));
        }

        // Generate code from usercode node
        if let Some(usercode_node) = &self.ast.usercode_node {
            code.push_str(&format!("/*** User Code ***/\n{}", usercode_node.value));
        }

        code
    }

    fn apply_options(&mut self) {
        if let Some(options) = &self.ast.definition_node.as_ref().unwrap().options {
            for option in options {
                match option.value {
                    "noyywrap" => self.options.push(option::Option::Noyywrap),
                    _ => {}
                }
            }
        }
    }

    fn gen_rule_code(&self) -> String {
        let mut code = String::new();
        code.push_str(PREPARE);
        code.push_str(&self.gen_pattern_array());
        code.push_str(&self.gen_action_function());
        code.push_str(MATCH);
        code.push_str(YYLEX);
        code
    }

    fn gen_pattern_array(&self) -> String {
        let mut code = String::new();
        code.push_str(&format!(
            "#define g_pattern_count {}\n",
            self.rule_table.pair_count
        ));
        code.push_str("char *g_patterns[] = {\n");
        for pattern in &self.rule_table.patterns {
            code.push_str(&format!("  \"{}\",\n", *pattern));
        }
        code.push_str("};\n");
        code.push('\n');
        code
    }

    fn gen_action_function(&self) -> String {
        let mut code = String::new();
        code.push_str("void action(int pattern_index) {\n");
        for (i, action) in self.rule_table.actions.iter().enumerate() {
            code.push_str(&format!(
                "  if (pattern_index == {}) {{\n{}\n  }}\n",
                i, action
            ));
        }
        code.push_str("}\n");
        code
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn codegen() {
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
void helper() {}"#;
        let mut codegen = CodeGen::new(Parser::new(Lexer::new(&source)).parse());
        let rule_table = RuleTable {
            pair_count: 3,
            patterns: vec!["pattern1", "pattern2", "pattern3"],
            actions: vec!["{ action1(); }", "{ action2(); }", "{ action3(); }"],
        };
        let code = codegen.generate();
        assert_eq!(code, target_code());
        assert_eq!(codegen.rule_table, rule_table);
        assert_eq!(codegen.options, vec![option::Option::Noyywrap]);
    }

    fn target_code() -> String {
        format!(
            "{}{}{}{}{}{}",
            r#"/*** Definition Code ***/
    c code block
/*** Rule Code ***/
"#,
            PREPARE,
            r#"#define g_pattern_count 3
char *g_patterns[] = {
  "pattern1",
  "pattern2",
  "pattern3",
};

void action(int pattern_index) {
  if (pattern_index == 0) {
{ action1(); }
  }
  if (pattern_index == 1) {
{ action2(); }
  }
  if (pattern_index == 2) {
{ action3(); }
  }
}
"#,
            MATCH,
            YYLEX,
            r#"
/*** User Code ***/

void helper() {}"#,
        )
    }
}
