use std::rc::Rc;

use time::Timespec;
use serialize::json::Json;

use expression::{ToExpression, UntypedExpression, RcExpression, RawExpr, BoxedExpression};
use sql::{ToSql};

#[derive(Clone, Copy)]
pub struct Placeholder {
    pub idx: usize
}

impl Placeholder {
    pub fn new(idx: usize) -> Placeholder {
        Placeholder { idx: idx }
    }
}

impl UntypedExpression for Placeholder {
    fn expression_as_sql(&self) -> &ToSql {
        self
    }

    fn upcast_expression(&self) -> RcExpression {
        Rc::new(Box::new(self.clone()) as BoxedExpression)
    }
}

impl ToExpression<bool> for Placeholder {}
impl ToExpression<f64> for Placeholder {}
impl ToExpression<String> for Placeholder {}
impl ToExpression<Vec<u8>> for Placeholder {}
impl ToExpression<Timespec> for Placeholder {}
impl ToExpression<Json> for Placeholder {}
impl ToExpression<RawExpr> for Placeholder {}