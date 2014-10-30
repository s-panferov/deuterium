use serialize::json::Json;
use time::Timespec;

use sql::{SqlContext, ToSql};

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


pub trait ToPredicateValue {
    fn to_predicate_value(&self, ctx: &mut SqlContext) -> String;
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

impl ToPredicateValue for bool { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for i8 { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for i16 { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for i32 { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for i64 { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for f32 { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for f64 { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for int { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for uint { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for String { 
    fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { format!("'{}'", self) } 
}
impl ToPredicateValue for Vec<u8> { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for Json { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for Timespec { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.to_string() } }
impl ToPredicateValue for RawExpr { fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String { self.content.to_string() } }

macro_rules! extended_impl(
    ($t:ty) => (
        impl ToSql for $t { fn to_sql(&self, ctx: &mut SqlContext) -> String { self.to_predicate_value(ctx) } }

        impl ToPredicateValue for Option<$t> { 
            fn to_predicate_value(&self, ctx: &mut SqlContext) -> String { 
                match self {
                    &Some(ref predicate) => predicate.to_predicate_value(ctx),
                    &None => "NULL".to_string()
                }
            }
        }

        impl ToSql for Option<$t> {
            fn to_sql(&self, ctx: &mut SqlContext) -> String { 
                self.to_predicate_value(ctx) 
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
extended_impl!(int)
extended_impl!(uint)
extended_impl!(String)
extended_impl!(Vec<u8>)
extended_impl!(Json)
extended_impl!(Timespec)
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

