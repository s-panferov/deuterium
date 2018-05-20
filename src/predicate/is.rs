use super::super::sql;
use super::super::expression;
use super::super::field;

use super::ToSharedPredicate;

#[derive(Clone, Debug)]
pub struct IsPredicate<F, T> {
    field: F,
    value: T,
}

impl<F, T> IsPredicate<F, T> {
    pub fn get_field(&self) -> &F { &self.field }
    pub fn get_value(&self) -> &T { &self.value }
}

pub trait ToIsPredicate<T> {
    fn is<B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static>(&self, val: B) -> super::SharedPredicate;
}

impl<F, T> super::Predicate for IsPredicate<F, T>
    where F: sql::ToPredicateValue,
          T: sql::ToPredicateValue { }

impl<T> ToIsPredicate<T> for field::NamedField<T> where T: sql::ToPredicateValue + Clone + 'static {
    fn is<B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static>(&self, val: B) -> super::SharedPredicate {
        IsPredicate { field: self.clone(), value: val }.upcast()
    }
}

impl<T> ToIsPredicate<T> for expression::RawExpression where T: sql::ToPredicateValue + Clone + 'static {
    fn is<B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static>(&self, val: B) -> super::SharedPredicate {
        IsPredicate { field: self.clone(), value: val }.upcast()
    }
}
