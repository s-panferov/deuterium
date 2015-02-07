use super::super::sql;
use super::super::expression;
use super::super::field;

use super::{ToSharedPredicate};

#[derive(Clone)]
pub struct IsNullPredicate<F> {
    field: F,
    is_null: bool
}

impl<F> IsNullPredicate<F> {
    pub fn get_field(&self) -> &F { &self.field }
    pub fn is_null(&self) -> bool { self.is_null }
}

pub trait ToIsNullPredicate {
    fn is_null(&self) -> super::SharedPredicate;
    fn not_null(&self) -> super::SharedPredicate;
}

impl<F> super::Predicate for IsNullPredicate<F> where F: sql::ToPredicateValue {}

impl<T> ToIsNullPredicate for field::NamedField<Option<T>> where T: sql::ToPredicateValue + Clone {
    fn is_null(&self) -> super::SharedPredicate {
        IsNullPredicate { field: self.clone(), is_null: true }.upcast()
    }

    fn not_null(&self) -> super::SharedPredicate {
        IsNullPredicate { field: self.clone(), is_null: false }.upcast()
    }
}

impl ToIsNullPredicate for expression::RawExpression {
    fn is_null(&self) -> super::SharedPredicate {
        IsNullPredicate { field: self.clone(), is_null: true }.upcast()
    }

    fn not_null(&self) -> super::SharedPredicate {
        IsNullPredicate { field: self.clone(), is_null: false }.upcast()
    }
}