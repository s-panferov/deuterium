use serialize::json::Json;
use time::Timespec;

use select_query::{
    SelectQuery, RcSelectQuery,
    SelectFor,
    SelectForUpdate,
    SelectForUpdateNoWait,
    SelectForShare,
    SelectForShareNoWait
};

use insert_query::{
    InsertQuery, 
    Insert,
    InsertDefaultValues,
    InsertValues,
    InsertUntypedValues,
    InsertFromSelect
};

use {Select, SelectOnly, SelectAll};

use predicate::{
    RcPredicate, 
    RawPredicate,
    IsPredicate, 
    OrPredicate, 
    AndPredicate,
    InPredicate,
    InRangePredicate, ExcludeBoth, IncludeBoth, ExcludeRight, ExcludeLeft,
    InequalityPredicate, LessThan, LessThanEqual, GreaterThan, GreaterThanEqual,
    ExcludePredicate,
    LikePredicate,
    IsNullPredicate
};

use field::{
    RcField,
    NamedField,
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
use raw_expression::{RawExpression};
use from::{Table, TableDef, FromSelect};
use distinct::{Distinct};
use group_by::{GroupBy};
use join::{
    Join, 
    ConditionedJoin, 
    UnconditionedJoin,
    ConditionedJoinType,
    UnconditionedJoinType,
    InnerJoin,
    FullOuterJoin,
    RightOuterJoin, 
    LeftOuterJoin,
    FullJoin,
    RightJoin,
    LeftJoin,
    NaturalJoin,
    NaturalLeftJoin,
    NaturalRightJoin,
    NaturalFullJoin,
    CrossJoin
};

use function::{
    Sum, SumArg,
    Min, MinArg,
    Max, MaxArg,
    Avg, AvgArg,
    Count, CountArg,
    CountAll
};

use expression::{
    ExprValue,
    ExpressionValue,
    DefaultValue,
};

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
        format!("{} {}", self.get_by().expression_as_sql().to_sql(), match self.get_order() {
            &Asc => "ASC",
            &Desc => "DESC"
        })
    }
}

impl ToSql for ConditionedJoinType {
    fn to_sql(&self) -> String {
        match self {
            &InnerJoin => "INNER JOIN",
            &FullOuterJoin => "FULL OUTER JOIN",
            &RightOuterJoin => "RIGHT OUTER JOIN",
            &LeftOuterJoin => "LEFT OUTER JOIN",
            &FullJoin => "FULL JOIN",
            &RightJoin => "RIGHT JOIN",
            &LeftJoin => "LEFT JOIN",
        }.to_string()
    }
}

impl ToSql for UnconditionedJoinType {
    fn to_sql(&self) -> String {
        match self {
            &NaturalJoin => "NATURAL JOIN",
            &NaturalLeftJoin => "NATURAL LEFT JOIN",
            &NaturalRightJoin => "NATURAL RIGHT JOIN",
            &NaturalFullJoin => "NATURAL FULL JOIN",
            &CrossJoin => "CROSS JOIN",
        }.to_string()
    }
}

impl ToSql for Join {
    fn to_sql(&self) -> String {
        match self {
            &ConditionedJoin(ref join_type, ref from, ref on) => {
                format!("{} {} ON {}", join_type.to_sql(), from.as_sql().to_from_sql(), on.to_sql(false))
            },
            &UnconditionedJoin(ref join_type, ref from) => {
                format!("{} {}", join_type.to_sql(), from.as_sql().to_from_sql())
            }
        }
    }
}

impl ToSql for Distinct {
    fn to_sql(&self) -> String {
        match &self.on {
            &None => "DISTINCT".to_string(),
            &Some(ref on) if on.is_empty() => "DISTINCT".to_string(),
            &Some(ref on) => {
                let defs: Vec<String> = on.iter().map(|f| f.expression_as_sql().to_sql()).collect();
                format!("DISTINCT ON ({})", defs.connect(", "))
            }
        }
    }
}

impl ToSql for GroupBy {
    fn to_sql(&self) -> String {
        if !self.by.is_empty() {
            let defs: Vec<String> = self.by.iter().map(|f| f.expression_as_sql().to_sql()).collect();
            format!(" GROUP BY {}", defs.connect(", "))
        } else {
            String::new()
        }
    }
}

impl FromToSql for TableDef {
    fn to_from_sql(&self) -> String {
        let name = self.get_table_name();
        match self.get_table_alias() {
            &Some(ref alias) => format!("{} AS {}", name, alias),
            &None => format!("{}", name),
        }
    }
}

impl<T, L, M> FromToSql for FromSelect<T, L, M> {
    fn to_from_sql(&self) -> String {
        format!("({}) as {}", self.select.to_sql(), self.alias.to_string())
    }
}

impl ToSql for SelectFor {
    fn to_sql(&self) -> String {
        match self {
            &SelectForUpdate => "FOR UPDATE",
            &SelectForUpdateNoWait => "FOR UPDATE NOWAIT",
            &SelectForShare => "FOR SHARE",
            &SelectForShareNoWait => "FOR SHARE NOWAIT",
        }.to_string()
    }
}

impl<T, L, M> ToSql for SelectQuery<T, L, M> {
    fn to_sql(&self) -> String {
        let mut sql = "SELECT".to_string();

        if self.distinct.is_some() {
            sql = format!("{} {}", sql, self.distinct.as_ref().unwrap().to_sql());
        }

        sql = format!("{} {} FROM {}", 
            sql,
            self.select.to_sql(), 
            self.from.as_sql().to_from_sql()
        );

        if !self.joins.is_empty() {
            let joins: Vec<String> = self.joins.iter().map(|join| join.to_sql()).collect();
            sql = format!("{} {}", sql, joins.connect(" "))
        }

        if self.where_.is_some() {
            sql = format!("{} WHERE {}", sql, self.where_.as_ref().unwrap().to_sql(false));
        }

        if self.group_by.is_some() {
            sql = format!("{}{}", sql, self.group_by.as_ref().unwrap().to_sql());
        }

        if self.having.is_some() {
            sql = format!("{} HAVING {}", sql, self.having.as_ref().unwrap().to_sql(false));
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

        if self.for_.is_some() {
            sql = format!("{} {}", sql, self.for_.unwrap().to_sql())
        }

        sql
    }
}

impl<T, L, M> QueryToSql for SelectQuery<T, L, M> {
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
                let defs: Vec<String> = fields.iter().map(|f| f.expression_as_sql().to_sql()).collect();
                defs.connect(", ")
            },
            &SelectAll => "*".to_string()
        }
    }
}

impl<T: Clone> ToSql for NamedField<T> {
    fn to_sql(&self) -> String {
        let ref name = self.name;
        match &self.qual {
            &Some(ref qual) => format!("{}.{}", qual, name),
            &None => name.to_string()
        }
    }
}

impl ToSql for RcField {
    fn to_sql(&self) -> String {
        let ref name = self.name();
        match &self.qual() {
            &Some(ref qual) => format!("{}.{}", qual, name),
            &None => name.to_string()
        }
    }
}

pub trait ToPredicateValue {
    fn to_predicate_value(&self) -> String;
}

macro_rules! to_predicate_for_field(
    ($f:ty) => (
        impl ToPredicateValue for $f  {
            fn to_predicate_value(&self) -> String { self.to_sql() }
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
impl ToPredicateValue for int { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for uint { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for &'static str {  
    fn to_predicate_value(&self) -> String { format!("'{}'", self) } 
}
impl ToPredicateValue for String { 
    fn to_predicate_value(&self) -> String { format!("'{}'", self) } 
}
impl ToPredicateValue for Vec<u8> { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for Json { fn to_predicate_value(&self) -> String { self.to_string() } }
impl ToPredicateValue for Timespec { fn to_predicate_value(&self) -> String { self.to_string() } }

impl ToSql for bool { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for i8 { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for i16 { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for i32 { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for i64 { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for f32 { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for f64 { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for int { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for uint { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for &'static str { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for String { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for Vec<u8> { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for Json { fn to_sql(&self) -> String { self.to_predicate_value() } }
impl ToSql for Timespec { fn to_sql(&self) -> String { self.to_predicate_value() } }

impl ToPredicateValue for RawExpression { fn to_predicate_value(&self) -> String { self.content.to_string() } }

impl<T: ToPredicateValue> ToPredicateValue for Vec<T> {
    fn to_predicate_value(&self) -> String { 
        let values: Vec<String> = self.iter().map(|v| v.to_predicate_value()).collect();
        values.connect(", ")
    }  
}

impl<T, L, M> ToPredicateValue for SelectQuery<T, L, M> {
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
        format!("({}) AND ({})", left, right)
    }
}

impl<F: ToPredicateValue, T: ToPredicateValue> PredicateToSql for InPredicate<F, T> {
    fn to_sql(&self, negation: bool) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        let values = self.values.to_predicate_value();
        format!("{} {}IN ({})", self.field.to_predicate_value(), maybe_not, values)
    }
}

impl<F: ToPredicateValue, T: ToPredicateValue> PredicateToSql for LikePredicate<F, T> {
    fn to_sql(&self, negation: bool) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        let like = if self.case_sensitive { "LIKE" } else { "ILIKE" };
        let values = self.value.to_predicate_value();
        format!("{} {}{} {}", self.field.to_predicate_value(), maybe_not, like, values)
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
            GreaterThanEqual => {
                if !negation {
                    format!("{} >= {}", name, value)
                } else {
                    format!("{} < {}", name, value)
                }
            }
        }
    }
}

macro_rules! agg_to_sql(
    ($foo:ident, $foo_arg:ident, $fmt:expr) => (
        impl<R, T, E> ToSql for $foo<R, T, E> where R: Clone, T: Clone, E: $foo_arg<R, T> {
            fn to_sql(&self) -> String {
                format!($fmt, self.expression.expression_as_sql().to_sql())
            }    
        }
    )
)

agg_to_sql!(Sum, SumArg, "SUM({})")
agg_to_sql!(Min, MinArg, "MIN({})")
agg_to_sql!(Max, MaxArg, "MAX({})")
agg_to_sql!(Avg, AvgArg, "AVG({})")
agg_to_sql!(Count, CountArg, "COUNT({})")

impl ToSql for CountAll {
    fn to_sql(&self) -> String {
        "COUNT(*)".to_string()
    }    
}

impl<T> ToSql for ExprValue<T> {
    fn to_sql(&self) -> String {
        match self {
            &ExpressionValue(ref e) => {
                e.expression_as_sql().to_sql()
            },
            &DefaultValue => "DEFAULT".to_string()
        }
    } 
}

macro_rules! to_sql_for_insert_tuple(
    ($fmt:expr, $($t:ident, $var:ident),+) => (
        impl<$($t,)+> ToSql for ($(ExprValue<$t>),+,)  {
            fn to_sql(&self) -> String {
                let &($(ref $var,)+) = self;
                format!($fmt, $($var.to_sql(),)+)
            }
        }

    )
)

impl ToSql for ()  {
    fn to_sql(&self) -> String {
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

impl<T: Clone, V: ToSql, M: Clone> ToSql for Insert<T, V, M> {
    fn to_sql(&self) -> String {
        match self {
            &InsertDefaultValues => {
                format!("DEFAULT VALUES")
            },
            &InsertValues(ref rows) => {
                let rows_str: Vec<String> = rows.iter().map(|row| { format!("({})", row.to_sql()) }).collect();
                format!("VALUES\n    {}", rows_str.connect(",\n    "))
            },
            &InsertUntypedValues(ref rows) => {
                let rows_str: Vec<String> = rows.iter().map(|row| {
                    let values_str: Vec<String> = row.iter().map(|v| v.to_sql()).collect();
                    format!("({})", values_str.connect(", "))
                }).collect();
                format!("VALUES\n    {}", rows_str.connect(",\n    "))    
            },
            &InsertFromSelect(ref select) => {
                select.to_sql()
            }
        }
    }
}

impl<T: Clone, V: Clone+ToSql, M: Clone> ToSql for InsertQuery<T, V, M> {
    fn to_sql(&self) -> String {
        let mut sql = format!("INSERT INTO {}", self.get_into().get_table_name());

        let maybe_cols = self.get_cols().as_ref();
        if maybe_cols.is_some() {
            let cols = maybe_cols.unwrap();

            if !cols.is_empty() {
                let cols_str: Vec<String> = cols.iter().map(|col| col.to_sql()).collect();
                sql = format!("{} ({})", sql, cols_str.connect(", "))
            }
        }

        format!("{} {}", sql, self.get_values().to_sql())
    }
}

impl<T: Clone, V: Clone+ToSql, M: Clone> QueryToSql for InsertQuery<T, V, M> {
    fn to_final_sql(&self) -> String {
        format!("{};", self.to_sql())
    }
}
