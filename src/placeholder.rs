use std::sync::Arc;

use time::Timespec;
use serialize::json::Json;

use expression::{UntypedExpression, RcExpression, RawExpr, BoxedExpression, ToExpression};
use to_sql::{ToSql};

#[deriving(Clone)]
pub struct Placeholder {
    pub idx: uint
}

impl Placeholder {
    pub fn new(idx: uint) -> Placeholder {
        Placeholder { idx: idx }
    }
}

impl UntypedExpression for Placeholder {
    fn expression_as_sql(&self) -> &ToSql {
        self
    }

    fn upcast_expression(&self) -> RcExpression {
        Arc::new(box self.clone() as BoxedExpression)
    }
}

impl ToExpression<bool> for Placeholder {}
impl ToExpression<i8> for Placeholder {}
impl ToExpression<i16> for Placeholder {}
impl ToExpression<i32> for Placeholder {}
impl ToExpression<i64> for Placeholder {}
impl ToExpression<f32> for Placeholder {}
impl ToExpression<f64> for Placeholder {}
impl ToExpression<String> for Placeholder {}
impl ToExpression<Vec<u8>> for Placeholder {}
impl ToExpression<Timespec> for Placeholder {}
impl ToExpression<Json> for Placeholder {}
impl ToExpression<RawExpr> for Placeholder {}