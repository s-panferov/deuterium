
use serialize::json::Json;
use time::Timespec;

use to_sql::{ToPredicateValue};
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

#[deriving(Clone)]
pub struct RawExpression {
    pub content: String
}

impl RawExpression {
    pub fn new(content: String) -> RawExpression { 
        RawExpression {
            content: content
        }
    }
}

pub trait RawExpressionComparable: Send + Clone + Sync + ToPredicateValue { }

impl RawExpressionComparable for bool {}
impl RawExpressionComparable for i8 {}
impl RawExpressionComparable for i16 {}
impl RawExpressionComparable for i32 {}
impl RawExpressionComparable for i64 {}
impl RawExpressionComparable for f32 {}
impl RawExpressionComparable for f64 {}
impl RawExpressionComparable for Vec<u8> {}
impl RawExpressionComparable for String {}
impl RawExpressionComparable for Json {}
impl RawExpressionComparable for Timespec {}
impl RawExpressionComparable for BoolField {} 
impl RawExpressionComparable for I8Field {} 
impl RawExpressionComparable for I16Field {} 
impl RawExpressionComparable for I32Field {} 
impl RawExpressionComparable for I64Field {} 
impl RawExpressionComparable for F32Field {} 
impl RawExpressionComparable for F64Field {} 
impl RawExpressionComparable for StringField {} 
impl RawExpressionComparable for JsonField {} 
impl RawExpressionComparable for ByteListField {} 
impl RawExpressionComparable for TimespecField {} 
impl RawExpressionComparable for RawExpression {} 