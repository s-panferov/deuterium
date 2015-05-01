use super::super::join;

impl super::ToSql for join::ConditionedJoinType {
    fn to_sql(&self, _ctx: &mut super::SqlContext) -> String {
        match self {
            &join::ConditionedJoinType::InnerJoin => "INNER JOIN",
            &join::ConditionedJoinType::FullOuterJoin => "FULL OUTER JOIN",
            &join::ConditionedJoinType::RightOuterJoin => "RIGHT OUTER JOIN",
            &join::ConditionedJoinType::LeftOuterJoin => "LEFT OUTER JOIN",
            &join::ConditionedJoinType::FullJoin => "FULL JOIN",
            &join::ConditionedJoinType::RightJoin => "RIGHT JOIN",
            &join::ConditionedJoinType::LeftJoin => "LEFT JOIN",
        }.to_string()
    }
}

impl super::ToSql for join::UnconditionedJoinType {
    fn to_sql(&self, _ctx: &mut super::SqlContext) -> String {
        match self {
            &join::UnconditionedJoinType::NaturalJoin => "NATURAL JOIN",
            &join::UnconditionedJoinType::NaturalLeftJoin => "NATURAL LEFT JOIN",
            &join::UnconditionedJoinType::NaturalRightJoin => "NATURAL RIGHT JOIN",
            &join::UnconditionedJoinType::NaturalFullJoin => "NATURAL FULL JOIN",
            &join::UnconditionedJoinType::CrossJoin => "CROSS JOIN",
        }.to_string()
    }
}

impl super::ToSql for join::Join {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        match self {
            &join::Join::ConditionedJoin{ref join_type, ref from, ref on} => {
                format!("{} {} ON {}", join_type.to_sql(ctx), from.as_sql().to_from_sql(ctx), on.to_sql(false, ctx))
            },
            &join::Join::UnconditionedJoin{ref join_type, ref from} => {
                format!("{} {}", join_type.to_sql(ctx), from.as_sql().to_from_sql(ctx))
            }
        }
    }
}
