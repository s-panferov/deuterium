
pub use from::{Table, RcTable};
pub use field::{NamedField, Field, RcField};
pub use select_query::{SelectQuery, LimitMany};
pub use expression::{Expression, UntypedExpression, RcExpression};

#[deriving(Clone)]
pub enum InsertValue<T, E: Expression<T>> {
    ExpressionValue(E),
    DefaultValue
}

#[allow(dead_code)]
#[deriving(Clone)]
pub enum Insert<T, V, M> {
    InsertDefaultValues,
    InsertValues(Vec<V>),
    InsertUntypedValues(Vec<Vec<RcExpression>>),
    InsertFromSelect(SelectQuery<T, LimitMany, M>)
}

#[allow(dead_code)]
#[deriving(Clone)]
pub struct InsertQuery<T, V, M> {
    pub into: RcTable,
    pub cols: Option<Vec<RcField>>,
    pub values: Insert<T, V, M>
}

macro_rules! insert(
    ($n:ident, $(($t:ident, $e:ident, $t_:ident)),+) => (
        // FIXME: Make this public after rust#17635
        fn $n<$($t:Clone, $e: Expression<$t> + Clone,)+>(table: &Table, $($t_: &NamedField<$t>,)+) -> InsertQuery<($($t,)+), ($(InsertValue<$t, $e>,)+), M> {
            let mut cols = vec![];
            $(cols.push((*$t_).upcast_field());)+
            let mut query = InsertQuery::new(table);
            query.cols = Some(cols);
            query
        }
    )
)

#[allow(dead_code)]
impl<T: Clone, V: Clone, M: Clone> InsertQuery<T, V, M> {
    
    pub fn new(into: &Table) -> InsertQuery<T, V, M> {
        InsertQuery {
            into: into.upcast_table(),
            cols: None,
            values: InsertDefaultValues
        }
    }

    pub fn push(&mut self, value: V) {
        let mut reassign = false;
        match &self.values {
            &InsertDefaultValues | &InsertFromSelect(_) => {
                reassign = true;
            },
            _ => ()
        }

        if reassign {
            self.values = InsertValues(vec![value])
        } else {
            match &mut self.values {
                &InsertValues(ref mut values) => {
                    values.push(value)
                },
                _ => ()
            }
        }
    }

    pub fn push_untyped(&mut self, values: &[&UntypedExpression]) {
        let mut reassign = false;
        match &self.values {
            &InsertDefaultValues | &InsertFromSelect(_) => {
                reassign = true;
            },
            _ => ()
        }

        let values_vec = values.iter().map(|v| v.upcast()).collect();

        if reassign {
            self.values = InsertUntypedValues(vec![values_vec])
        } else {
            match &mut self.values {
                &InsertUntypedValues(ref mut values) => {
                    values.push(values_vec)
                },
                _ => ()
            }
        }
    }

    pub fn from_select(&self, select: SelectQuery<T, LimitMany, M>) -> InsertQuery<T, V, M> {
        with_clone!(self, query, query.values = InsertFromSelect(select))
    }

    // FIXME: Make this public after rust-lang/rust#17635 and remove after rust-lang/rfcs#376
    insert!(insert_1, (T0, T0Expr, _t0))
    insert!(insert_2, (T0, T0Expr, _t0), (T1, T1Expr, _t1))
    insert!(insert_3, (T0, T0Expr, _t0), (T1, T1Expr, _t1), (T2, T2Expr, _t2))
    insert!(insert_4, (T0, T0Expr, _t0), (T1, T1Expr, _t1), (T2, T2Expr, _t2), (T3, T3Expr, _t3))
    insert!(insert_5, (T0, T0Expr, _t0), (T1, T1Expr, _t1), (T2, T2Expr, _t2), (T3, T3Expr, _t3), (T4, T4Expr, _t4))
    insert!(insert_6, (T0, T0Expr, _t0), (T1, T1Expr, _t1), (T2, T2Expr, _t2), (T3, T3Expr, _t3), (T4, T4Expr, _t4), (T5, T5Expr, _t5))
    insert!(insert_7, (T0, T0Expr, _t0), (T1, T1Expr, _t1), (T2, T2Expr, _t2), (T3, T3Expr, _t3), (T4, T4Expr, _t4), (T5, T5Expr, _t5), (T6, T6Expr, _t6))
    insert!(insert_8, (T0, T0Expr, _t0), (T1, T1Expr, _t1), (T2, T2Expr, _t2), (T3, T3Expr, _t3), (T4, T4Expr, _t4), (T5, T5Expr, _t5), (T6, T6Expr, _t6), (T7, T7Expr, _t7))
    insert!(insert_9, (T0, T0Expr, _t0), (T1, T1Expr, _t1), (T2, T2Expr, _t2), (T3, T3Expr, _t3), (T4, T4Expr, _t4), (T5, T5Expr, _t5), (T6, T6Expr, _t6), (T7, T7Expr, _t7), (T8, T8Expr, _t8))
    insert!(insert_10, (T0, T0Expr, _t0), (T1, T1Expr, _t1), (T2, T2Expr, _t2), (T3, T3Expr, _t3), (T4, T4Expr, _t4), (T5, T5Expr, _t5), (T6, T6Expr, _t6), (T7, T7Expr, _t7), (T8, T8Expr, _t8), (T9, T9Expr, _t9))
    insert!(insert_11, (T0, T0Expr, _t0), (T1, T1Expr, _t1), (T2, T2Expr, _t2), (T3, T3Expr, _t3), (T4, T4Expr, _t4), (T5, T5Expr, _t5), (T6, T6Expr, _t6), (T7, T7Expr, _t7), (T8, T8Expr, _t8), (T9, T9Expr, _t9), (T10, T10Expr, _t10))
    insert!(insert_12, (T0, T0Expr, _t0), (T1, T1Expr, _t1), (T2, T2Expr, _t2), (T3, T3Expr, _t3), (T4, T4Expr, _t4), (T5, T5Expr, _t5), (T6, T6Expr, _t6), (T7, T7Expr, _t7), (T8, T8Expr, _t8), (T9, T9Expr, _t9), (T10, T10Expr, _t10), (T11, T11Expr, _t11))

}

