
use predicate::{Predicate, ToAbstractPredicate, RcPredicate};

use expression::{RawExpr};
use field;

#[derive(Clone)]
pub struct IsNullPredicate<F> {
    pub field: F,
    pub null: bool
}

pub trait ToIsNullPredicate {
    fn is_null(&self) -> RcPredicate;
    fn not_null(&self) -> RcPredicate;
}

impl<F, T> Predicate for InPredicate<F, T> 
    where F: ToPredicateValue,
          T: ToPredicateValue { }

macro_rules! impl_for{
    ($f:ty) => (

        impl Predicate for IsNullPredicate<$f> { }
        impl ToIsNullPredicate for $f {
            fn is_null(&self) -> RcPredicate {
                IsNullPredicate {
                    field: self.clone(),
                    null: true
                }.upcast()
            }

            fn not_null(&self) -> RcPredicate {
                IsNullPredicate {
                    field: self.clone(),
                    null: false
                }.upcast()
            }
        }

    )
}

impl_for!(OptionalBoolField);
impl_for!(OptionalI8Field);
impl_for!(OptionalI16Field);
impl_for!(OptionalI32Field);
impl_for!(OptionalI64Field);
impl_for!(OptionalF32Field);
impl_for!(OptionalF64Field);
impl_for!(OptionalStringField);
impl_for!(OptionalByteListField);
impl_for!(OptionalJsonField);
impl_for!(OptionalTimespecField);
impl_for!(OptionalUuidField);
impl_for!(RawExpr);

