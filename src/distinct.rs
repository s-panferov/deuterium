
use super::expression;

#[derive(Clone)]
pub struct Distinct {
    pub on: Option<Vec<expression::SharedExpression>>
}

impl Distinct {
    pub fn new() -> Distinct {
        Distinct { on: None }
    }

    pub fn on(fields: &[&expression::UntypedExpression]) -> Distinct {
        Distinct { on: Some( 
            fields.iter().map(|f| f.upcast_expression()).collect() 
        )}
    }
}