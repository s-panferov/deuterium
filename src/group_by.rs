
use field::{UntypedExpression, RcExpression};

#[deriving(Clone)]
pub struct GroupBy {
    pub by: Vec<RcExpression>
}

impl GroupBy {
    pub fn new(fields: &[&UntypedExpression]) -> GroupBy {
        GroupBy { by: fields.iter().map(|f| f.upcast()).collect() }
    }
}