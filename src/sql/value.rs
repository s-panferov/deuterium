use serialize::json::Json;
use time::Timespec;

use sql::{SqlContext, ToSql};

#[cfg(feature = "raw_expr")]
use expression::{RawExpr};
use field::{
    BoolField,
    I8Field,
    I16Field,
    I32Field,
    I64Field,
    F32Field,
    F64Field,
    StringField,
    ByteListField,
    JsonField,
    TimespecField,
};

#[cfg(feature = "postgres")]
use postgres;

pub trait ToPredicateValue {
    fn to_predicate_value(&self, ctx: &mut SqlContext) -> String;
}

// Trait to connect Deuterium and rust-postgres
#[cfg(feature = "postgres")]
pub trait AsPostgresValue: postgres::types::ToSql {
    fn as_postgres_value(&self) -> &postgres::types::ToSql {
        self
    }
}

macro_rules! to_predicate_for_field(
    ($f:ty) => (
        impl ToPredicateValue for $f  {
            fn to_predicate_value(&self, ctx: &mut SqlContext) -> String { self.to_sql(ctx) }
        }
    )
)

to_predicate_for_field!(BoolField)
to_predicate_for_field!(I8Field)
to_predicate_for_field!(I16Field)
to_predicate_for_field!(I32Field)
to_predicate_for_field!(I64Field)
to_predicate_for_field!(F32Field)
to_predicate_for_field!(F64Field)
to_predicate_for_field!(StringField)
to_predicate_for_field!(ByteListField)
to_predicate_for_field!(JsonField)
to_predicate_for_field!(TimespecField)

macro_rules! raw_value_to_predicate_value(
    ($t:ty) => (
        #[cfg(feature = "postgres")]
        impl AsPostgresValue for $t {}

        impl ToPredicateValue for $t { 
            fn to_predicate_value(&self, ctx: &mut SqlContext) -> String { 
                ctx.hold(box self.clone())
            }
        }
    )
)

raw_value_to_predicate_value!(bool)
raw_value_to_predicate_value!(i8)
raw_value_to_predicate_value!(i16)
raw_value_to_predicate_value!(i32)
raw_value_to_predicate_value!(i64)
raw_value_to_predicate_value!(f32)
raw_value_to_predicate_value!(f64)
raw_value_to_predicate_value!(String)
raw_value_to_predicate_value!(Vec<u8>)
raw_value_to_predicate_value!(Json)
raw_value_to_predicate_value!(Timespec)
#[cfg(feature = "raw_expr")]
raw_value_to_predicate_value!(RawExpr)

macro_rules! extended_impl(
    ($t:ty) => (
        impl ToSql for $t { fn to_sql(&self, ctx: &mut SqlContext) -> String { self.to_predicate_value(ctx) } }
        impl ToSql for Option<$t> { fn to_sql(&self, ctx: &mut SqlContext) -> String { self.to_predicate_value(ctx) } }

        impl ToPredicateValue for Option<$t> { 
            fn to_predicate_value(&self, ctx: &mut SqlContext) -> String { 
                match self {
                    &Some(ref predicate) => predicate.to_predicate_value(ctx),
                    &None => "NULL".to_string()
                }
            }
        }
    )
)

extended_impl!(bool)
extended_impl!(i8)
extended_impl!(i16)
extended_impl!(i32)
extended_impl!(i64)
extended_impl!(f32)
extended_impl!(f64)
extended_impl!(String)
extended_impl!(Vec<u8>)
extended_impl!(Json)
extended_impl!(Timespec)

#[cfg(feature = "raw_expr")]
extended_impl!(RawExpr)

impl<T: ToPredicateValue> ToPredicateValue for Vec<T> {
    fn to_predicate_value(&self, ctx: &mut SqlContext) -> String { 
        let values: Vec<String> = self.iter().map(|v| v.to_predicate_value(ctx)).collect();
        values.connect(", ")
    }  
}

impl<T: ToPredicateValue> ToSql for Vec<T> {
    fn to_sql(&self, ctx: &mut SqlContext) -> String { 
        self.to_predicate_value(ctx)
    }  
}

