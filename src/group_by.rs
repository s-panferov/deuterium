
use field::{UntypedField, FieldDef};

#[deriving(Clone)]
pub struct GroupBy {
    pub by: Vec<FieldDef<()>>
}

impl GroupBy {
    pub fn new(fields: &[&UntypedField]) -> GroupBy {
        GroupBy { by: fields.iter().map(|f| f.to_def().clone_with_erase()).collect() }
    }
}