use super::super::sql;
use super::super::expression;
use super::super::field;

use super::{ToSharedPredicate};

#[derive(Clone)]
pub struct IsPredicate<F, T> {
    pub field: F,
    pub value: T
}

pub trait ToIsPredicate<T> {
    fn is<B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static>(&self, val: B) -> super::SharedPredicate;
}

impl<F, T> super::Predicate for IsPredicate<F, T> 
    where F: sql::ToPredicateValue,
          T: sql::ToPredicateValue { }

impl<T> ToIsPredicate<T> for field::NamedField<T> where T: sql::ToPredicateValue + Clone {
    fn is<B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static>(&self, val: B) -> super::SharedPredicate {
        IsPredicate { field: self.clone(), value: val }.upcast()
    }
}

impl<T> ToIsPredicate<T> for expression::RawExpression where T: sql::ToPredicateValue + Clone {
    fn is<B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static>(&self, val: B) -> super::SharedPredicate {
        IsPredicate { field: self.clone(), value: val }.upcast()
    }
}