use std::rc::Rc;

use time::Timespec;
use serialize::json::Json;

use expression::{Expression, UntypedExpression, RcExpression, RawExpr, BoxedExpression};
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

impl Expression<bool> for Placeholder {}
impl Expression<f64> for Placeholder {}
impl Expression<String> for Placeholder {}
impl Expression<Vec<u8>> for Placeholder {}
impl Expression<Timespec> for Placeholder {}
impl Expression<Json> for Placeholder {}
impl Expression<RawExpr> for Placeholder {}