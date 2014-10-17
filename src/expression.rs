
#[deriving(Clone)]
pub struct RawExpression {
    pub content: String
}

impl RawExpression {
    pub fn new(content: String) -> RawExpression { 
        RawExpression {
            content: content
        }
    }
}