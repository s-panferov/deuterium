use serialize::json::Json;
use time::Timespec;

use select_query::{SelectQuery, RcSelectQuery};
use {Select, SelectOnly, SelectAll};
use predicate::{
    RcPredicate, 
    RawPredicate,
    IsPredicate, 
    OrPredicate, 
    AndPredicate,
    InPredicate,
    InRangePredicate, ExcludeBoth, IncludeBoth, ExcludeRight, ExcludeLeft,
    InequalityPredicate, LessThan, LessThanEqual, GreaterThan, GreaterTranEqual,
    ExcludePredicate,
    IsNullPredicate
};
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

use order_by::{OrderBy, Asc, Desc};
use expression::{RawExpression};

use from::{TableDef, FromSelect};

pub trait QueryToSql {
    fn to_final_sql(&self) -> String;
}

pub trait ToSql {
    fn to_sql(&self) -> String;
}

pub trait PredicateToSql {
    fn to_sql(&self, bool) -> String;
}

pub trait FromToSql {
    fn to_from_sql(&self) -> String;
}

impl ToSql for OrderBy {
    fn to_sql(&self) -> String {
        format!("{} {}", self.get_by(), match self.get_order() {
            &Asc => "ASC",
            &Desc => "DESC"
        })
    }
}

impl FromToSql for TableDef {
    fn to_from_sql(&self) -> String {
        match self.get_alias() {
            &Some(ref alias) => format!("{} AS {}", self.get_name(), alias),
            &None => format!("{}", self.get_name()),
        }
    }
}

impl<T, L> FromToSql for FromSelect<T, L> {
    fn to_from_sql(&self) -> String {
        format!("({}) as {}", self.select.to_sql(), self.alias.to_string())
    }
}

impl<T, L> ToSql for SelectQuery<T, L> {
    fn to_sql(&self) -> String {
        let mut sql = format!("SELECT {} FROM {}", 
            self.select.to_sql(), 
            self.from.to_from_sql()
        );

        if self.where_.is_some() {
            sql = format!("{} WHERE {}", sql, self.where_.as_ref().unwrap().to_sql(false))
        }

        if !self.order_by.is_empty() {
            let orders: Vec<String> = self.order_by.iter().map(|ord| ord.to_sql()).collect();
            sql = format!("{} ORDER BY {}", sql, orders.connect(", "))
        }

        if self.limit.is_some() {
            sql = format!("{} LIMIT {}", sql, self.limit.unwrap())
        }

        if self.offset.is_some() {
            sql = format!("{} OFFSET {}", sql, self.offset.unwrap())
        }

        sql
    }
}

impl<T, L> QueryToSql for SelectQuery<T, L> {
    fn to_final_sql(&self) -> String {
        format!("{};", self.to_sql())
    }
}

impl ToSql for RcSelectQuery {
    fn to_sql(&self) -> String {
        (**self).to_sql()
    }
}

impl ToSql for Select {
    fn to_sql(&self) -> String {
        match self {
            &SelectOnly(ref fields) => {
                fields.connect(", ")
            },
            &SelectAll => "*".to_string()
        }
    }
}

pub trait ToPredicateValue {
    fn to_predicate_value(&self) -> String;
}

macro_rules! to_predicate_for_field(
    ($f:ty) => (
        impl ToPredicateValue for $f  {
            fn to_predicate_value(&self) -> String { self.name.to_string() }
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

impl ToPredicateValue for bool { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for i8 { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for i16 { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for i32 { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for i64 { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for f32 { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for f64 { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for String { 
    fn to_predicate_value(&self) -> String { format!("'{}'", self.to_string()) } 
}
impl ToPredicateValue for Vec<u8> { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for Json { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for Timespec { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for RawExpression { fn to_predicate_value(&self) -> String { self.content.to_string() } }

impl<T: ToPredicateValue> ToPredicateValue for Vec<T> {
    fn to_predicate_value(&self) -> String { 
        let values: Vec<String> = self.iter().map(|v| v.to_predicate_value()).collect();
        values.connect(", ")
    }  
}

impl<T, L> ToPredicateValue for SelectQuery<T, L> {
    fn to_predicate_value(&self) -> String { self.to_sql() }   
}

impl<F: ToPredicateValue, T: ToPredicateValue> PredicateToSql for IsPredicate<F, T> {
    fn to_sql(&self, negation: bool) -> String {
        let op = if negation { "!=" } else { "=" };
        format!("{} {} {}", self.field.to_predicate_value(), op, self.value.to_predicate_value())
    }
}

impl<F: ToPredicateValue> PredicateToSql for IsNullPredicate<F> {
    fn to_sql(&self, negation: bool) -> String {
        let op = if !negation && self.null { "IS NULL" } else { "IS NOT NULL" };
        format!("{} {}", self.field.to_predicate_value(), op)
    }
}

impl PredicateToSql for RcPredicate {
    fn to_sql(&self, negation: bool) -> String {
        (**self).to_sql(negation)
    }
}

impl PredicateToSql for OrPredicate {
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

impl PredicateToSql for RawPredicate {
    fn to_sql(&self, negation: bool) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        format!("{}{}", maybe_not, self.content.to_string())
    }
}

impl PredicateToSql for ExcludePredicate {
    fn to_sql(&self, negation: bool) -> String {
        self.predicate.to_sql(!negation)
    }
}

impl PredicateToSql for AndPredicate {
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

impl<F: ToPredicateValue, T: ToPredicateValue> PredicateToSql for InPredicate<F, T> {
    fn to_sql(&self, negation: bool) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        let values = self.values.to_predicate_value();
        format!("{} {}IN ({})", self.field.to_predicate_value(), maybe_not, values)
    }
}

impl<F: ToPredicateValue, T: ToPredicateValue> PredicateToSql for InRangePredicate<F, T> {
    fn to_sql(&self, negation: bool) -> String {
        let ref name = self.field.to_predicate_value();
        let from = self.from.to_predicate_value(); 
        let to = self.to.to_predicate_value();
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

impl<F: ToPredicateValue, T: ToPredicateValue> PredicateToSql for InequalityPredicate<F, T> {
    fn to_sql(&self, negation: bool) -> String {
        let ref name = self.field.to_predicate_value();
        let value = self.value.to_predicate_value();
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