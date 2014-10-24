
use field::{UntypedField, FieldDef};

#[deriving(Clone)]
pub struct Distinct {
    pub on: Option<Vec<FieldDef<()>>>,
}

impl Distinct {
    pub fn new() -> Distinct {
        Distinct { on: None }
    }

    pub fn on(fields: &[&UntypedField]) -> Distinct {
        Distinct { on: Some( 
            fields.iter().map(|f| f.to_def().clone_with_erase()).collect() 
        )}
    }
}