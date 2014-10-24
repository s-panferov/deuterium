
use to_sql::{ToSql};
use std::sync::Arc;

pub trait Expression<T>: UntypedExpression {}
pub trait UntypedExpression {
    fn expression_as_sql(&self) -> &ToSql;
    fn upcast(&self) -> RcExpression;
}

pub type BoxedExpression = Box<UntypedExpression + Send + Sync>;
pub type RcExpression = Arc<BoxedExpression>;