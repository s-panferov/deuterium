use super::expression;

#[derive(Clone)]
pub struct GroupBy {
    pub by: Vec<expression::RcExpression>
}

impl GroupBy {
    pub fn new(fields: &[&expression::UntypedExpression]) -> GroupBy {
        GroupBy { by: fields.iter().map(|f| f.upcast_expression()).collect() }
    }
}