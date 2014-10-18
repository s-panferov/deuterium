
use predicate::{Predicate, RcPredicate};

use field::{
    I8Field, I8ComparableList,
    I16Field, I16ComparableList,
    I32Field, I32ComparableList,
    I64Field, I64ComparableList,
    F32Field, F32ComparableList,
    F64Field, F64ComparableList,
    StringField, StringComparableList,
    TimespecField, TimespecComparableList
};

#[deriving(Send, Clone)]
pub struct InPredicate<F, T> {
    pub field: F,
    pub values: T
}

pub trait ToInPredicate<F, T> {
    fn within(&self, val: T) -> RcPredicate;
}

macro_rules! within_methods(
    ($v:ty) => (
        fn within(&self, values: $v) -> RcPredicate {
            InPredicate {
                field: self.clone(),
                values: values
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ident) => (
        impl<T: $v> Predicate for InPredicate<$field, T> { }

        impl<T: $v> ToInPredicate<$field, T> for $field {
            within_methods!(T)   
        }
    )
)

impl_for!(I8Field, I8ComparableList)
impl_for!(I16Field, I16ComparableList)
impl_for!(I32Field, I32ComparableList)
impl_for!(I64Field, I64ComparableList)
impl_for!(F32Field, F32ComparableList)
impl_for!(F64Field, F64ComparableList)
impl_for!(StringField, StringComparableList)
impl_for!(TimespecField, TimespecComparableList)