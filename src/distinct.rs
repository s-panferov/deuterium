
use field::{UntypedExpression, RcExpression};

#[deriving(Clone)]
pub struct Distinct {
    pub on: Option<Vec<RcExpression>>
}

impl Distinct {
    pub fn new() -> Distinct {
        Distinct { on: None }
    }

    pub fn on(fields: &[&UntypedExpression]) -> Distinct {
        Distinct { on: Some( 
            fields.iter().map(|f| f.upcast()).collect() 
        )}
    }
}