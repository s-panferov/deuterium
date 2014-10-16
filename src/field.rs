use std::sync::Arc;

trait Coercer: Send {

}

pub trait Field {
    fn to_def(&self) -> FieldDef;
}

#[deriving(Clone)]
pub struct NamedField<T> {
    pub name: String
}

impl<T> Field for NamedField<T> {
    fn to_def(&self) -> FieldDef {
        FieldDef(self.name.to_string())
    }
}

#[deriving(Clone)]
pub struct FieldDef(String);