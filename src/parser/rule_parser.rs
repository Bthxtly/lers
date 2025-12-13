mod code;

use code::MATCH;
use code::PREPARE;

pub struct RuleParser<'a> {
    pair_count: usize,
    patterns: Vec<&'a str>,
    actions: Vec<&'a str>,
}

impl<'a> RuleParser<'a> {
    fn new() -> Self {
        RuleParser {
            pair_count: 0,
            patterns: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn add_pattern(&mut self, pattern: &'a str) {
        self.patterns.push(pattern);
        self.pair_count += 1;
    }

    pub fn add_action(&mut self, action: &'a str) {
        self.actions.push(action);
    }

    pub fn gen_code(&self) -> String {
        let mut code = String::new();
        code.push_str(PREPARE);
        code.push_str(&self.gen_pattern_array());
        code.push_str(&self.gen_action_function());
        code.push_str(MATCH);
        code.push_str(&self.gen_yylex_function());
        code
    }

    fn gen_pattern_array(&self) -> String {
        let mut array_code = String::new();
        array_code.push_str(&format!("#define g_pattern_count {}\n", self.pair_count));
        array_code.push_str("char *g_patterns[] = {\n");
        for pattern in self.patterns.iter() {
            array_code.push_str(&format!("  \"{}\",\n", pattern));
        }
        array_code.push_str("};\n");
        array_code.push('\n');
        array_code
    }

    fn gen_action_function(&self) -> String {
        let mut func_code = String::new();
        func_code.push_str("void action(int pattern_index) {\n");
        for (i, action) in self.actions.iter().enumerate() {
            func_code.push_str(&format!(
                "  if (pattern_index == {}) {{\n{}\n  }}\n",
                i, action
            ));
        }
        func_code.push_str("}\n");
        func_code
    }

    fn gen_yylex_function(&self) -> String {
        let mut func_code = String::new();
        func_code.push_str("int yylex() {\n");
        func_code.push_str("  read_file(\"test.txt\");\n");
        func_code.push_str("  match();\n");
        func_code.push_str("  return 0;\n");
        func_code.push_str("}\n");
        func_code
    }
}

impl Default for RuleParser<'_> {
    fn default() -> Self {
        Self::new()
    }
}
