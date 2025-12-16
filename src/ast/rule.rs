#[derive(Debug, PartialEq)]
pub struct RuleNode<'a> {
    // does a lex file contain no rules?
    pub rules: Option<Vec<RulePairNode<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct RulePairNode<'a> {
    pub pattern: &'a str,
    pub action: &'a str,
}
