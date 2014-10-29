
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

use sql::{ToSql};

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