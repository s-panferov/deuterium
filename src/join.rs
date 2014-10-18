
pub enum ConditionedJoinType {
    InnerJoin,
    FullOuterJoin,
    RightOuterJoin, 
    LeftOuterJoin,
    FullOuterJoin,
    RightJoin,
    LeftJoin
}

pub enum UnconditionedJoinType {
    NaturalJoin,
    NaturalLeftJoin,
    NaturalRight,
    NaturalFull,
    Cross
}

struct ConditionedJoin {
    join_type: ConditionedJoinType
}