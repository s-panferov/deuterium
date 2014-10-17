use serialize::json::Json;
use time::Timespec;

use data_set::SelectDataSet;
use {Select, SelectOnly, SelectAll, From, NamedFrom, DataSetFrom};
use query::{
    RcQuery, 
    RawQuery,
    IsQuery, 
    OrQuery, 
    AndQuery,
    InQuery,
    InRangeQuery, InRangeBounds, ExcludeBoth, IncludeBoth, ExcludeRight, ExcludeLeft,
    InequalityQuery, Inequality, LessThan, LessThanEqual, GreaterThan, GreaterTranEqual,
    ExcludeQuery,
    IsNullQuery
};
use field::{
    Field, 
    FieldDef,
    NamedField, 

    BoolField, BoolComparable,
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

use expression::{RawExpression};

pub trait ToSql {
    fn to_sql(&self) -> String;
}

pub trait QueryToSql {
    fn to_sql(&self, bool) -> String;
}

impl ToSql for SelectDataSet {
    fn to_sql(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}", 
            self.select.to_sql(), 
            self.from.to_sql()
        );

        if self.where_.is_some() {
            sql = format!("{} WHERE {}", sql, self.where_.as_ref().unwrap().to_sql(false))
        }

        format!("{};", sql)
    }
}

impl ToSql for From {
    fn to_sql(&self) -> String {
        match self {
            &NamedFrom(ref from) => {
                from.to_string()
            },
            &DataSetFrom(ref dset) => format!("( {} )", dset.to_sql())
        }
    }
}

impl ToSql for Select {
    fn to_sql(&self) -> String {
        match self {
            &SelectOnly(ref fields) => {
                let names: Vec<String> = fields.iter().map(|f| f.name().to_string()).collect();
                names.connect(", ")
            },
            &SelectAll => "*".to_string()
        }
    }
}

pub trait ToQueryValue {
    fn to_query_value(&self) -> String;
}

macro_rules! to_query_for_field(
    ($f:ty) => (
        impl ToQueryValue for $f  {
            fn to_query_value(&self) -> String { self.name.to_string() }
        }
    )
)

to_query_for_field!(BoolField)
to_query_for_field!(I8Field)
to_query_for_field!(I16Field)
to_query_for_field!(I32Field)
to_query_for_field!(I64Field)
to_query_for_field!(F32Field)
to_query_for_field!(F64Field)
to_query_for_field!(StringField)
to_query_for_field!(ByteListField)
to_query_for_field!(JsonField)
to_query_for_field!(TimespecField)

impl ToQueryValue for bool { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for i8 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for i16 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for i32 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for i64 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for f32 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for f64 { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for String { 
    fn to_query_value(&self) -> String { format!("'{}'", self.to_string()) } 
}
impl ToQueryValue for Vec<u8> { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for Json { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for Timespec { fn to_query_value(&self) -> String { self.to_string() } }
impl ToQueryValue for RawExpression { fn to_query_value(&self) -> String { self.content.to_string() } }

impl<F: ToQueryValue, T: ToQueryValue> QueryToSql for IsQuery<F, T> {
    fn to_sql(&self, negation: bool) -> String {
        let op = if negation { "!=" } else { "=" };
        format!("{} {} {}", self.field.to_query_value(), op, self.value.to_query_value())
    }
}

impl<F: ToQueryValue> QueryToSql for IsNullQuery<F> {
    fn to_sql(&self, negation: bool) -> String {
        let op = if !negation && self.null { "IS NULL" } else { "IS NOT NULL" };
        format!("{} {}", self.field.to_query_value(), op)
    }
}

impl QueryToSql for RcQuery {
    fn to_sql(&self, negation: bool) -> String {
        (**self).to_sql(negation)
    }
}

impl QueryToSql for OrQuery {
    fn to_sql(&self, negation: bool) -> String {
        let left = self.left.to_sql(negation);
        let right = self.right.to_sql(negation);
        if !negation {
            format!("({}) OR ({})", left, right)
        } else {
            format!("({}) AND ({})", left, right)
        }
    }
}

impl QueryToSql for RawQuery {
    fn to_sql(&self, negation: bool) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        format!("{}{}", maybe_not, self.content.to_string())
    }
}

impl QueryToSql for ExcludeQuery {
    fn to_sql(&self, negation: bool) -> String {
        self.query.to_sql(!negation)
    }
}

impl QueryToSql for AndQuery {
    fn to_sql(&self, negation: bool) -> String {
        let left = self.left.to_sql(negation);
        let right = self.right.to_sql(negation);
        if !negation {
            format!("({}) AND ({})", left, right)
        } else {
            format!("({}) OR ({})", left, right)
        }
    }
}

impl<F: ToQueryValue, T: ToQueryValue> QueryToSql for InQuery<F, Vec<T>> {
    fn to_sql(&self, negation: bool) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        let query_values: Vec<String> = self.values.iter().map(|v| v.to_query_value()).collect();
        format!("{} {}IN ({})", self.field.to_query_value(), maybe_not, query_values.connect(", "))
    }
}

impl<F: ToQueryValue, T: ToQueryValue> QueryToSql for InRangeQuery<F, T> {
    fn to_sql(&self, negation: bool) -> String {
        let ref name = self.field.to_query_value();
        let from = self.from.to_query_value(); 
        let to = self.to.to_query_value();
        match self.bounds {
            IncludeBoth => {
                if !negation {
                    format!("{} >= {} AND {} <= {}", name, from, name, to)
                } else {
                    format!("{} < {} OR {} > {}", name, from, name, to)
                }
            },
            ExcludeBoth => {
                if !negation {
                    format!("{} > {} AND {} < {}", name, from, name, to)
                } else {
                    format!("{} <= {} OR {} >= {}", name, from, name, to)
                }
            },
            ExcludeLeft => {
                if !negation {
                    format!("{} > {} AND {} <= {}", name, from, name, to)
                } else {
                    format!("{} <= {} OR {} > {}", name, from, name, to)
                }
            },
            ExcludeRight => {
                if !negation {
                    format!("{} >= {} AND {} < {}", name, from, name, to)
                } else {
                    format!("{} < {} OR {} >= {}", name, from, name, to)
                }
            }
        }
    }
}

impl<F: ToQueryValue, T: ToQueryValue> QueryToSql for InequalityQuery<F, T> {
    fn to_sql(&self, negation: bool) -> String {
        let ref name = self.field.to_query_value();
        let value = self.value.to_query_value();
        match self.inequality {
            LessThan => {
                if !negation {
                    format!("{} < {}", name, value)
                } else {
                    format!("{} >= {}", name, value)
                }
            },
            LessThanEqual => {
                if !negation {
                    format!("{} <= {}", name, value)
                } else {
                    format!("{} > {}", name, value)
                }
            },
            GreaterThan => {
                if !negation {
                    format!("{} > {}", name, value)
                } else {
                    format!("{} <= {}", name, value)
                }
            },
            GreaterTranEqual => {
                if !negation {
                    format!("{} >= {}", name, value)
                } else {
                    format!("{} < {}", name, value)
                }
            }
        }
    }
}