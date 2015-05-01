use std::rc;
use time;
use serialize::json;

use super::expression;
use super::sql;

#[derive(Clone, Copy)]
pub struct Placeholder {
    idx: u8
}

impl Placeholder {
    pub fn new(idx: u8) -> Placeholder {
        Placeholder { idx: idx }
    }

    pub fn get_idx(&self) -> u8 {
        self.idx
    }
}

impl expression::UntypedExpression for Placeholder {
    fn expression_as_sql(&self) -> &sql::ToSql {
        self
    }

    fn upcast_expression(&self) -> expression::SharedExpression {
        rc::Rc::new(Box::new(self.clone()) as expression::BoxedExpression)
    }
}

impl expression::ToExpression<bool> for Placeholder {}
impl expression::ToExpression<f64> for Placeholder {}
impl expression::ToExpression<String> for Placeholder {}
impl expression::ToExpression<Vec<u8>> for Placeholder {}
impl expression::ToExpression<time::Timespec> for Placeholder {}
impl expression::ToExpression<json::Json> for Placeholder {}
impl expression::ToExpression<expression::RawExpression> for Placeholder {}
