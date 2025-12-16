mod definition;
mod rule;
mod usercode;

pub use definition::*;
pub use rule::*;
pub use usercode::*;

#[derive(Debug, PartialEq)]
pub struct Root<'a> {
    pub definition_node: Option<DefinitionNode<'a>>,
    pub rule_node: Option<RuleNode<'a>>,
    pub usercode_node: Option<UserCodeNode<'a>>,
}
