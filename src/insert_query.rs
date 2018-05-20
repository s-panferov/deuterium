use std::mem;
use std::marker;

use super::from;
use super::field;
use super::select_query;
use super::expression;

#[derive(Clone, Debug)]
pub enum InsertValue<T> {
    Value {
        expression: expression::SharedExpression,
        _marker: marker::PhantomData<T>
    },
    Default
}

pub trait ToInsertValue<T> {
    fn to_insert_val(&self) -> InsertValue<T>;
}

impl<T> InsertValue<T> {
    pub fn new(exp: &expression::Expression<T>) -> InsertValue<T> {
        InsertValue::Value {
            expression: exp.upcast_expression(),
            _marker: marker::PhantomData
        }
    }
}

impl<'a, 'b, T> ToInsertValue<T> for &'a (expression::Expression<T> + 'b) {
    fn to_insert_val(&self) -> InsertValue<T> {
        InsertValue::new(*self)
    }
}

#[derive(Clone, Debug)]
pub enum Insert<T, V, M> {
    DefaultValues,
    Values(Vec<V>),
    UntypedValues(Vec<Vec<InsertValue<expression::RawExpression>>>),
    FromSelect(select_query::SelectQuery<T, select_query::LimitMany, M>)
}

#[derive(Clone, Debug)]
pub struct InsertQuery<T, V, M, RT, RL> {
    into: from::SharedTable,
    cols: Option<Vec<field::SharedField>>,
    values: Insert<T, V, M>,
    returning: Option<select_query::Select>,

    _marker_rt: marker::PhantomData<RT>,
    _marker_rl: marker::PhantomData<RL>
}

#[macro_export]
macro_rules! insert {
    ($name:ident, $(($t:ident, $arg:ident)),+) => (
        // FIXME: Make this public after https://github.com/rust-lang/rust/issues/17635
        pub fn $name<$($t:Clone,)+>(&self, $($arg: &NamedField<$t>,)+) -> InsertQuery<($($t,)+), ($(InsertValue<$t>,)+), M, (), ()> {
            let mut cols = vec![];
            $(cols.push((*$arg).upcast_field());)+
            let mut query = InsertQuery::new(self);
            query.cols = Some(cols);
            query
        }
    )
}

macro_rules! insertable {
    () => (
        pub trait Insertable<M: Clone>: from::Table + Sized {
            // FIXME: Rewrite after https://github.com/rust-lang/rfcs/issues/376:
            //        Draft RFC: variadic generics
            // insert!(insert_1, (T0, _t0));
            // insert!(insert_2, (T0, _t0), (T1, _t1));
            // insert!(insert_3, (T0, _t0), (T1, _t1), (T2, _t2));
            // insert!(insert_4, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3));
            // insert!(insert_5, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4));
            // insert!(insert_6, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5));
            // insert!(insert_7, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6));
            // insert!(insert_8, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7));
            // insert!(insert_9, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8));
            // insert!(insert_10, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9));
            // insert!(insert_11, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10));
            // insert!(insert_12, (T0, _t0), (T1, _t1), (T2, _t2), (T3, _t3), (T4, _t4), (T5, _t5), (T6, _t6), (T7, _t7), (T8, _t8), (T9, _t9), (T10, _t10), (T11, _t11));

            fn insert_all(&self) -> InsertQuery<(), (), M, (), ()> {
                 InsertQuery::new(self)
            }

            fn insert_fields(&self, fields: &[&field::Field]) -> InsertQuery<(), (), M, (), ()> {
                let mut cols = vec![];
                for field in fields.iter() {
                    cols.push(field.upcast_field())
                }
                InsertQuery::new_with_cols(self, cols)
            }
        }
    )
}


insertable!();

impl<T: Clone, V: Clone, M: Clone, RT: Clone, RL: Clone> InsertQuery<T, V, M, RT, RL> {
    pub fn new(into: &from::Table) -> InsertQuery<T, V, M, RT, RL> {
        InsertQuery {
            into: into.upcast_table(),
            cols: None,
            values: Insert::DefaultValues,
            returning: None,

            _marker_rt: marker::PhantomData,
            _marker_rl: marker::PhantomData,
        }
    }

    pub fn new_with_cols(into: &from::Table, cols: Vec<field::SharedField>) -> InsertQuery<T, V, M, RT, RL> {
        InsertQuery {
            into: into.upcast_table(),
            cols: Some(cols),
            values: Insert::DefaultValues,
            returning: None,

            _marker_rt: marker::PhantomData,
            _marker_rl: marker::PhantomData,
        }
    }

    pub fn get_into(&self) -> &from::SharedTable { &self.into }
    pub fn get_cols(&self) -> &Option<Vec<field::SharedField>> { &self.cols }
    pub fn get_values(&self) -> &Insert<T, V, M> { &self.values }
    pub fn get_returning(&self) -> &Option<select_query::Select> { &self.returning }

    pub fn push(&mut self, value: V) {

        let mut reassign = false;
        match &self.values {
            &Insert::DefaultValues | &Insert::FromSelect(_) => {
                reassign = true;
            },
            _ => ()
        }

        if reassign {
            self.values = Insert::Values(vec![value])
        } else {
            match &mut self.values {
                &mut Insert::Values(ref mut values) => {
                    values.push(value)
                },
                _ => ()
            }
        }
    }

    pub fn push_untyped(&mut self, values: &[&expression::Expression<expression::RawExpression>]) {
        let mut reassign = false;
        match &self.values {
            &Insert::DefaultValues | &Insert::FromSelect(_) => {
                reassign = true;
            },
            _ => ()
        }

        let values_vec = values.iter().map(|v| v.to_insert_val()).collect();

        if reassign {
            self.values = Insert::UntypedValues(vec![values_vec])
        } else {
            match &mut self.values {
                &mut Insert::UntypedValues(ref mut values) => {
                    values.push(values_vec)
                },
                _ => ()
            }
        }
    }

    pub fn from_select(&self, select: select_query::SelectQuery<T, select_query::LimitMany, M>) -> InsertQuery<T, V, M, RT, RL> {
        with_clone!(self, query, query.values = Insert::FromSelect(select))
    }

}

impl<T: Clone, V: Clone, M: Clone, RT, RL> InsertQuery<T, V, M, RT, RL> {
    pub fn returning_1<T1: Clone>(mut self, field: &expression::Expression<T1>) -> InsertQuery<T, V, M, (T1), select_query::LimitMany> {
        self.returning = Some(select_query::Select::Only(vec![field.upcast_expression()]));
        unsafe{ mem::transmute(self) }
    }

    pub fn returning_2<T1: Clone, T2: Clone>(mut self, field1: &expression::Expression<T1>, field2: &expression::Expression<T2>) -> InsertQuery<T, V, M, (T1, T2), select_query::LimitMany> {
        self.returning = Some(select_query::Select::Only(vec![field1.upcast_expression(), field2.upcast_expression()]));
        unsafe{ mem::transmute(self) }
    }

    pub fn returning(mut self, fields: &[&expression::UntypedExpression]) -> InsertQuery<T, V, M, (), select_query::LimitMany> {
        self.returning = Some(select_query::Select::Only(fields.iter().map(|f| f.upcast_expression()).collect()));
        unsafe{ mem::transmute(self) }
    }

    pub fn returning_all(mut self) -> InsertQuery<T, V, M, (), select_query::LimitMany> {
        self.returning = Some(select_query::Select::All);
        unsafe{ mem::transmute(self) }
    }

    pub fn no_returning(mut self) -> InsertQuery<T, V, M, (), select_query::NoResult> {
        self.returning = None;
        unsafe{ mem::transmute(self) }
    }
}
