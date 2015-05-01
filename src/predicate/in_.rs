use super::super::expression;
use super::super::sql;
use super::super::field;

use super::{ToSharedPredicate};

#[derive(Clone)]
pub struct InPredicate<F, T> {
    field: F,
    values: T
}

impl<F, T> InPredicate<F, T> {
    pub fn get_field(&self) -> &F { &self.field }
    pub fn get_values(&self) -> &T { &self.values }
}

pub trait ToInPredicate<T> {
    fn in_<B>(&self, values: B) -> super::SharedPredicate
        where B: expression::ToListExpression<T> + sql::ToPredicateValue + Clone + 'static;
}

impl<F, T> super::Predicate for InPredicate<F, T>
    where F: sql::ToPredicateValue,
          T: sql::ToPredicateValue { }

impl<T> ToInPredicate<T> for field::NamedField<T> where T: sql::ToPredicateValue + Clone + 'static {
    fn in_<B: expression::ToListExpression<T> + sql::ToPredicateValue + Clone + 'static>(&self, val: B) -> super::SharedPredicate {
        InPredicate { field: self.clone(), values: val }.upcast()
    }
}

impl<T> ToInPredicate<T> for expression::RawExpression where T: sql::ToPredicateValue + Clone + 'static {
    fn in_<B: expression::ToListExpression<T> + sql::ToPredicateValue + Clone + 'static>(&self, val: B) -> super::SharedPredicate {
        InPredicate { field: self.clone(), values: val }.upcast()
    }
}
