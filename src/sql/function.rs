
use function::{
    Sum, SumArg,
    Min, MinArg,
    Max, MaxArg,
    Avg, AvgArg,
    Count, CountArg,
    CountAll
};

use sql::{SqlContext, ToSql};

macro_rules! agg_to_sql(
    ($foo:ident, $foo_arg:ident, $fmt:expr) => (
        impl<R, T, E> ToSql for $foo<R, T, E> where R: Clone, T: Clone, E: $foo_arg<R, T> {
            fn to_sql(&self, ctx: &mut SqlContext) -> String {
                format!($fmt, self.expression.expression_as_sql().to_sql(ctx))
            }    
        }
    )
)

agg_to_sql!(Sum, SumArg, "SUM({})")
agg_to_sql!(Min, MinArg, "MIN({})")
agg_to_sql!(Max, MaxArg, "MAX({})")
agg_to_sql!(Avg, AvgArg, "AVG({})")
agg_to_sql!(Count, CountArg, "COUNT({})")

impl ToSql for CountAll {
    fn to_sql(&self, _ctx: &mut SqlContext) -> String {
        "COUNT(*)".to_string()
    }    
}
