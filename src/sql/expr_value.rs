use expression::{
    ExprValue,
    ExpressionValue,
    DefaultValue,
};

use sql::{SqlContext, ToSql};

impl<T> ToSql for ExprValue<T> {
    fn to_sql(&self, ctx: &mut SqlContext) -> String {
        match self {
            &ExpressionValue(ref e) => {
                e.expression_as_sql().to_sql(ctx)
            },
            &DefaultValue => "DEFAULT".to_string()
        }
    } 
}

macro_rules! to_sql_for_insert_tuple(
    ($fmt:expr, $($t:ident, $var:ident),+) => (
        impl<$($t,)+> ToSql for ($(ExprValue<$t>),+,)  {
            fn to_sql(&self, ctx: &mut SqlContext) -> String {
                let &($(ref $var,)+) = self;
                format!($fmt, $($var.to_sql(ctx),)+)
            }
        }

    )
)

impl ToSql for ()  {
    fn to_sql(&self, _ctx: &mut SqlContext) -> String {
        "DEFAULT VALUES".to_string()
    }
}

to_sql_for_insert_tuple!("{}", T1, t1)
to_sql_for_insert_tuple!("{}, {}", T1, t1, T2, t2)
to_sql_for_insert_tuple!("{}, {}, {}", T1, t1, T2, t2, T3, t3)
to_sql_for_insert_tuple!("{}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4)
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5)
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6)
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7)
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7, T8, t8)
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7, T8, t8, T9, t9)
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7, T8, t8, T9, t9, T10, t10)
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7, T8, t8, T9, t9, T10, t10, T11, t11)
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7, T8, t8, T9, t9, T10, t10, T11, t11, T12, t12)