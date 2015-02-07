use super::super::expression;
use super::super::field;
use super::super::sql;

use super::{ToAbstractPredicate};

#[derive(Clone)]
pub struct LikePredicate<F, T> {
    pub field: F,
    pub value: T,
    pub case_sensitive: bool
}

pub trait ToLikePredicate<T> {
    fn like<B>(&self, val: B) -> super::SharedPredicate 
        where B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static;

    fn ilike<B>(&self, val: B) -> super::SharedPredicate
        where B: expression::ToExpression<T> + sql::ToPredicateValue + Clone + 'static;
}

impl<F, T> super::Predicate for LikePredicate<F, T> 
    where F: sql::ToPredicateValue,
          T: sql::ToPredicateValue { }

macro_rules! impl_for {
    ($field:ty, $expr:ty) => (
        impl ToLikePredicate<$expr> for $field {
            fn like<B>(&self, val: B) -> super::SharedPredicate 
                where B: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static {
                LikePredicate { field: self.clone(), value: val, case_sensitive: true }.upcast()
            }

            fn ilike<B>(&self, val: B) -> super::SharedPredicate 
                where B: expression::ToExpression<$expr> + sql::ToPredicateValue + Clone + 'static {
                LikePredicate { field: self.clone(), value: val, case_sensitive: false }.upcast()
            } 
        }
    )
}

impl_for!(field::StringField, String);
impl_for!(field::OptionalStringField, Option<String>);
impl_for!(expression::RawExpression, String);