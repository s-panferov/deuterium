
use to_sql::{ToSql};
use std::sync::Arc;

use serialize::json::Json;
use time::Timespec;

pub trait Expression<T>: UntypedExpression {}
pub trait UntypedExpression {
    fn expression_as_sql(&self) -> &ToSql;
    fn upcast_expression(&self) -> RcExpression;
}

pub type BoxedExpression = Box<UntypedExpression + Send + Sync>;
pub type RcExpression = Arc<BoxedExpression>;

macro_rules! impl_for(
    ($t:ty) => (
        impl UntypedExpression for $t {
            fn expression_as_sql(&self) -> &ToSql {
                self
            }

            fn upcast_expression(&self) -> RcExpression {
                Arc::new(box self.clone() as BoxedExpression)
            }
        }

        impl Expression<$t> for $t {
            
        }
    )
)

impl_for!(bool)
impl_for!(i8)
impl_for!(i16)
impl_for!(i32)
impl_for!(i64)
impl_for!(f32)
impl_for!(f64)
impl_for!(String)
impl_for!(Vec<u8>)
impl_for!(Json)
impl_for!(Timespec)