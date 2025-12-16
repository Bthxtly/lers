#[derive(Debug, PartialEq)]
pub struct UserCodeNode<'a> {
    pub value: &'a str,
}

impl<'a> From<&'a str> for UserCodeNode<'a> {
    fn from(value: &'a str) -> Self {
        UserCodeNode { value }
    }
}
