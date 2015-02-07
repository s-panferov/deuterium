use super::super::expression;
use super::super::sql;
use super::super::field;

use super::{ToAbstractPredicate};

#[derive(Clone)]
pub struct InPredicate<F, T> {
    pub field: F,
    pub values: T
}

pub trait ToInPredicate<T> {
    fn in_<B>(&self, values: B) -> super::SharedPredicate 
        where B: expression::ToListExpression<T> + sql::ToPredicateValue + Clone + 'static;
}

impl<F, T> super::Predicate for InPredicate<F, T> 
    where F: sql::ToPredicateValue,
          T: sql::ToPredicateValue { }

impl<T> ToInPredicate<T> for field::NamedField<T> where T: sql::ToPredicateValue + Clone {
    fn in_<B: expression::ToListExpression<T> + sql::ToPredicateValue + Clone + 'static>(&self, val: B) -> super::SharedPredicate {
        InPredicate { field: self.clone(), values: val }.upcast()
    }
}

impl<T> ToInPredicate<T> for expression::RawExpr where T: sql::ToPredicateValue + Clone {
    fn in_<B: expression::ToListExpression<T> + sql::ToPredicateValue + Clone + 'static>(&self, val: B) -> super::SharedPredicate {
        InPredicate { field: self.clone(), values: val }.upcast()
    }
}