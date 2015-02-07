use super::from;
use super::predicate;

#[allow(dead_code)]
#[derive(Clone)]
pub enum ConditionedJoinType {
    InnerJoin,
    FullOuterJoin,
    RightOuterJoin, 
    LeftOuterJoin,
    FullJoin,
    RightJoin,
    LeftJoin
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum UnconditionedJoinType {
    NaturalJoin,
    NaturalLeftJoin,
    NaturalRightJoin,
    NaturalFullJoin,
    CrossJoin
}

#[allow(dead_code)]
#[derive(Clone)]
pub enum Join {
    ConditionedJoin {
        join_type: ConditionedJoinType,
        from: from::SharedFrom,
        on: predicate::SharedPredicate
    },

    UnconditionedJoin {
        join_type: UnconditionedJoinType,
        from: from::SharedFrom
    }
}

impl Join {
    pub fn inner_join(from: from::SharedFrom, predicate: predicate::SharedPredicate) -> Join { 
        Join::ConditionedJoin{ join_type: ConditionedJoinType::InnerJoin, from: from, on: predicate } 
    }

    pub fn full_outer_join(from: from::SharedFrom, predicate: predicate::SharedPredicate) -> Join { 
        Join::ConditionedJoin{ join_type: ConditionedJoinType::FullOuterJoin, from: from, on: predicate } 
    }

    pub fn right_outer_join(from: from::SharedFrom, predicate: predicate::SharedPredicate) -> Join { 
        Join::ConditionedJoin{ join_type: ConditionedJoinType::RightOuterJoin, from: from, on: predicate } 
    }

    pub fn left_outer_join(from: from::SharedFrom, predicate: predicate::SharedPredicate) -> Join { 
        Join::ConditionedJoin{ join_type: ConditionedJoinType::LeftOuterJoin, from: from, on: predicate } 
    }

    pub fn full_join(from: from::SharedFrom, predicate: predicate::SharedPredicate) -> Join { 
        Join::ConditionedJoin{ join_type: ConditionedJoinType::FullJoin, from: from, on: predicate } 
    }

    pub fn left_join(from: from::SharedFrom, predicate: predicate::SharedPredicate) -> Join { 
        Join::ConditionedJoin{ join_type: ConditionedJoinType::LeftJoin, from: from, on: predicate } 
    }

    pub fn right_join(from: from::SharedFrom, predicate: predicate::SharedPredicate) -> Join { 
        Join::ConditionedJoin{ join_type: ConditionedJoinType::RightJoin, from: from, on: predicate } 
    }

    pub fn natural_join(from: from::SharedFrom) -> Join { 
        Join::UnconditionedJoin{ join_type: UnconditionedJoinType::NaturalJoin, from: from } 
    }

    pub fn natural_left_join(from: from::SharedFrom) -> Join { 
        Join::UnconditionedJoin{ join_type: UnconditionedJoinType::NaturalLeftJoin, from: from } 
    }

    pub fn natural_right_join(from: from::SharedFrom) -> Join { 
        Join::UnconditionedJoin{ join_type: UnconditionedJoinType::NaturalRightJoin, from: from } 
    }

    pub fn natural_full_join(from: from::SharedFrom) -> Join { 
        Join::UnconditionedJoin{ join_type: UnconditionedJoinType::NaturalFullJoin, from: from } 
    }

    pub fn cross_join(from: from::SharedFrom) -> Join { 
        Join::UnconditionedJoin{ join_type: UnconditionedJoinType::CrossJoin, from: from } 
    }

}