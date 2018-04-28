use serde_json;
use chrono;
use uuid;
use std::fmt;
#[cfg(feature = "postgres")] use postgres;

use expression;
use field;

use super::ToSql;

pub trait ToPredicateValue: fmt::Debug {
    fn to_predicate_value(&self, ctx: &mut super::SqlContext) -> String;
}

// Trait to connect Deuterium and rust-postgres
#[cfg(feature = "postgres")]
pub trait AsPostgresValue: postgres::types::ToSql + Sized {
    fn as_postgres_value(&self) -> &postgres::types::ToSql {
        self
    }
}

#[macro_export]
macro_rules! to_predicate_for_field {
    ($f:ty) => (
        impl ToPredicateValue for $f  {
            fn to_predicate_value(&self, ctx: &mut super::SqlContext) -> String { self.to_sql(ctx) }
        }
    )
}

impl<T: Clone + fmt::Debug> ToPredicateValue for field::NamedField<T> {
    fn to_predicate_value(&self, ctx: &mut super::SqlContext) -> String { self.to_sql(ctx) }
}

macro_rules! raw_value_to_predicate_value {
    ($t:ty) => (
        #[cfg(feature = "postgres")]
        impl AsPostgresValue for $t {}

        impl ToPredicateValue for $t {
            fn to_predicate_value(&self, ctx: &mut super::SqlContext) -> String {
                ctx.hold(Box::new(self.clone()))
            }
        }
    )
}

raw_value_to_predicate_value!(bool);
raw_value_to_predicate_value!(i8);
raw_value_to_predicate_value!(i16);
raw_value_to_predicate_value!(i32);
raw_value_to_predicate_value!(i64);
raw_value_to_predicate_value!(f32);
raw_value_to_predicate_value!(f64);
raw_value_to_predicate_value!(String);
raw_value_to_predicate_value!(Vec<u8>);
raw_value_to_predicate_value!(serde_json::Value);
raw_value_to_predicate_value!(chrono::NaiveDateTime);
raw_value_to_predicate_value!(uuid::Uuid);

impl ToPredicateValue for expression::RawExpression {
    fn to_predicate_value(&self, _ctx: &mut super::SqlContext) -> String {
        self.content.to_string()
    }
}

macro_rules! extended_impl {
    ($t:ty) => (
        impl ToSql for $t { fn to_sql(&self, ctx: &mut super::SqlContext) -> String { self.to_predicate_value(ctx) } }
        impl ToSql for Option<$t> { fn to_sql(&self, ctx: &mut super::SqlContext) -> String { self.to_predicate_value(ctx) } }

        impl ToPredicateValue for Option<$t> {
            fn to_predicate_value(&self, ctx: &mut super::SqlContext) -> String {
                match self {
                    &Some(ref predicate) => predicate.to_predicate_value(ctx),
                    &None => "NULL".to_string()
                }
            }
        }
    )
}

extended_impl!(bool);
extended_impl!(i8);
extended_impl!(i16);
extended_impl!(i32);
extended_impl!(i64);
extended_impl!(f32);
extended_impl!(f64);
extended_impl!(String);
extended_impl!(Vec<u8>);
extended_impl!(serde_json::Value);
extended_impl!(chrono::NaiveDateTime);
extended_impl!(uuid::Uuid);


extended_impl!(expression::RawExpression);

impl<T: ToPredicateValue> ToPredicateValue for Vec<T> {
    fn to_predicate_value(&self, ctx: &mut super::SqlContext) -> String {
        let values: Vec<String> = self.iter().map(|v| v.to_predicate_value(ctx)).collect();
        values.join(", ")
    }
}

impl<T: ToPredicateValue> ToSql for Vec<T> {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        self.to_predicate_value(ctx)
    }
}
