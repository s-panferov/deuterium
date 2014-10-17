

use serialize::json::Json;
use time::Timespec;

use query::Query;
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
use to_sql::ToSql;

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
    fn lt(&self, val: T) -> InequalityQuery<F, T>;
    fn lte(&self, val: T) -> InequalityQuery<F, T>;
    fn gt(&self, val: T) -> InequalityQuery<F, T>;
    fn gte(&self, val: T) -> InequalityQuery<F, T>;
}

impl<T: Clone> ToInequalityQuery<NamedField<T>, T> for NamedField<T> {
    fn lt(&self, val: T) -> InequalityQuery<NamedField<T>, T> {
        InequalityQuery {
            field: self.clone(),
            value: val,
            inequality: LessThan
        }
    }

    fn lte(&self, val: T) -> InequalityQuery<NamedField<T>, T> {
        InequalityQuery {
            field: self.clone(),
            value: val,
            inequality: LessThanEqual
        }
    }

    fn gt(&self, val: T) -> InequalityQuery<NamedField<T>, T> {
        InequalityQuery {
            field: self.clone(),
            value: val,
            inequality: GreaterThan
        }
    }

    fn gte(&self, val: T) -> InequalityQuery<NamedField<T>, T> {
        InequalityQuery {
            field: self.clone(),
            value: val,
            inequality: LessThanEqual
        }
    }
}

impl Query for InequalityQuery<BoolField, bool> { }
impl Query for InequalityQuery<I8Field, i8> { }
impl Query for InequalityQuery<I16Field, i16> { }
impl Query for InequalityQuery<I32Field, i32> { }
impl Query for InequalityQuery<I64Field, i64> { }
impl Query for InequalityQuery<F32Field, f32> { }
impl Query for InequalityQuery<F64Field, f64> { }
impl Query for InequalityQuery<TimespecField, Timespec> { }