use super::super::insert_query;

impl<T> super::ToSql for insert_query::InsertValue<T> {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        match self {
            &insert_query::InsertValue::Value{ref expression} => {
                expression.expression_as_sql().to_sql(ctx)
            },
            &insert_query::InsertValue::Default => "DEFAULT".to_string()
        }
    } 
}

impl super::ToSql for ()  {
    fn to_sql(&self, _ctx: &mut super::SqlContext) -> String {
        "DEFAULT VALUES".to_string()
    }
}

macro_rules! to_sql_for_insert_tuple {
    ($fmt:expr, $($t:ident, $var:ident),+) => (
        impl<$($t,)+> super::ToSql for ($(insert_query::InsertValue<$t>),+,)  {
            fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
                let &($(ref $var,)+) = self;
                format!($fmt, $($var.to_sql(ctx),)+)
            }
        }

    )
}

to_sql_for_insert_tuple!("{}", T1, t1);
to_sql_for_insert_tuple!("{}, {}", T1, t1, T2, t2);
to_sql_for_insert_tuple!("{}, {}, {}", T1, t1, T2, t2, T3, t3);
to_sql_for_insert_tuple!("{}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4);
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5);
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6);
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7);
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7, T8, t8);
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7, T8, t8, T9, t9);
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7, T8, t8, T9, t9, T10, t10);
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7, T8, t8, T9, t9, T10, t10, T11, t11);
to_sql_for_insert_tuple!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}", T1, t1, T2, t2, T3, t3, T4, t4, T5, t5, T6, t6, T7, t7, T8, t8, T9, t9, T10, t10, T11, t11, T12, t12);