
use query::Query;
use field::NamedField;
use to_sql::ToSql;

#[deriving(Send, Clone)]
pub struct IsQuery<F, T> {
    pub field: F,
    pub value: T
}

pub trait ToIsQuery<F, T> {
    fn is(&self, val: T) -> IsQuery<F, T>;
}

impl<T: Clone> ToIsQuery<NamedField<T>, T> for NamedField<T> {
    fn is(&self, val: T) -> IsQuery<NamedField<T>, T> {
        IsQuery {
            field: self.clone(),
            value: val
        }
    }
}

impl Query for IsQuery<NamedField<String>, String> { }