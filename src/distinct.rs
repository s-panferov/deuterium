
use super::expression;

#[derive(Clone)]
pub struct Distinct {
    on: Option<Vec<expression::SharedExpression>>
}

impl Distinct {
    pub fn new() -> Distinct {
        Distinct { on: None }
    }

    pub fn get_on(&self) -> &Option<Vec<expression::SharedExpression>> { &self.on }

    pub fn on(fields: &[&expression::UntypedExpression]) -> Distinct {
        Distinct { on: Some( 
            fields.iter().map(|f| f.upcast_expression()).collect() 
        )}
    }
}