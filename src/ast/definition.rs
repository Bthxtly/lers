#[derive(Debug, Default, PartialEq)]
pub struct DefinitionNode<'a> {
    pub options: Option<Vec<OptionNode<'a>>>,
    pub code: Option<CodeNode<'a>>,
    pub definitions: Option<Vec<DefinitionPairNode<'a>>>,
}

#[derive(Debug, PartialEq)]
pub struct OptionNode<'a> {
    pub value: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct CodeNode<'a> {
    pub value: &'a str,
}

#[derive(Debug, PartialEq)]
pub struct DefinitionPairNode<'a> {
    pub name: &'a str,
    pub definition: &'a str,
}

impl<'a> From<&'a str> for OptionNode<'a> {
    fn from(value: &'a str) -> Self {
        OptionNode { value }
    }
}

impl<'a> From<&'a str> for CodeNode<'a> {
    fn from(value: &'a str) -> Self {
        CodeNode { value }
    }
}
