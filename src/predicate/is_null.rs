use super::super::sql;
use super::super::expression;
use super::super::field;

use super::{ToSharedPredicate};

#[derive(Clone)]
pub struct IsNullPredicate<F> {
    pub field: F,
    pub null: bool
}

pub trait ToIsNullPredicate {
    fn is_null(&self) -> super::SharedPredicate;
    fn not_null(&self) -> super::SharedPredicate;
}

impl<F> super::Predicate for IsNullPredicate<F> where F: sql::ToPredicateValue {}

impl<T> ToIsNullPredicate for field::NamedField<Option<T>> where T: sql::ToPredicateValue + Clone {
    fn is_null(&self) -> super::SharedPredicate {
        IsNullPredicate { field: self.clone(), null: true }.upcast()
    }

    fn not_null(&self) -> super::SharedPredicate {
        IsNullPredicate { field: self.clone(), null: false }.upcast()
    }
}

impl ToIsNullPredicate for expression::RawExpression {
    fn is_null(&self) -> super::SharedPredicate {
        IsNullPredicate { field: self.clone(), null: true }.upcast()
    }

    fn not_null(&self) -> super::SharedPredicate {
        IsNullPredicate { field: self.clone(), null: false }.upcast()
    }
}