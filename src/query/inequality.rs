

use serialize::json::Json;
use time::Timespec;

use query::{Query, RcQuery};
use expression::RawExpression;
use field::{
    NamedField,

    BoolField,
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
    StringField,
    ByteListField,
    JsonField,
    TimespecField,
};
use to_sql::{ToQueryValue};

#[deriving(Clone)]
pub enum Inequality {
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterTranEqual
}

#[deriving(Send, Clone)]
pub struct InequalityQuery<F, T> {
    pub field: F,
    pub value: T,
    pub inequality: Inequality
}

pub trait ToInequalityQuery<F, T> {
    fn lt(&self, val: T) -> RcQuery;
    fn lte(&self, val: T) -> RcQuery;
    fn gt(&self, val: T) -> RcQuery;
    fn gte(&self, val: T) -> RcQuery;
}

macro_rules! inequality_methods(
    ($v:ty) => (
        fn lt(&self, val: $v) -> RcQuery {
            InequalityQuery {
                field: self.clone(),
                value: val,
                inequality: LessThan
            }.upcast()
        }

        fn lte(&self, val: $v) -> RcQuery {
            InequalityQuery {
                field: self.clone(),
                value: val,
                inequality: LessThanEqual
            }.upcast()
        }

        fn gt(&self, val: $v) -> RcQuery {
            InequalityQuery {
                field: self.clone(),
                value: val,
                inequality: GreaterThan
            }.upcast()
        }

        fn gte(&self, val: $v) -> RcQuery {
            InequalityQuery {
                field: self.clone(),
                value: val,
                inequality: LessThanEqual
            }.upcast()
        }
    )
)

macro_rules! impl_for(
    ($field:ty, $v:ty) => (
        impl Query for InequalityQuery<$field, $v> { }

        impl ToInequalityQuery<$field, $v> for $field {
            inequality_methods!($v)    
        }
    )
)

impl_for!(I8Field, i8)
impl_for!(I16Field, i16)
impl_for!(I32Field, i32)
impl_for!(I64Field, i64)
impl_for!(F32Field, f32)
impl_for!(F64Field, f64)
impl_for!(TimespecField, Timespec)

impl Query for InequalityQuery<RawExpression, RawExpression> { }
impl ToInequalityQuery<RawExpression, RawExpression> for RawExpression {
    inequality_methods!(RawExpression)    
}