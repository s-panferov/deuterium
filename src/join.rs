
use from::{RcFrom};
use predicate::{RcPredicate};

#[allow(dead_code)]
#[deriving(Clone)]
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
#[deriving(Clone)]
pub enum UnconditionedJoinType {
    NaturalJoin,
    NaturalLeftJoin,
    NaturalRightJoin,
    NaturalFullJoin,
    CrossJoin
}

#[allow(dead_code)]
#[deriving(Clone)]
pub enum Join {
    ConditionedJoin {
        join_type: ConditionedJoinType,
        from: RcFrom,
        on: RcPredicate
    },

    UnconditionedJoin {
        join_type: UnconditionedJoinType,
        from: RcFrom
    }
}

impl Join {
    pub fn inner_join(from: RcFrom, predicate: RcPredicate) -> Join { 
        ConditionedJoin{ join_type: InnerJoin, from: from, on: predicate } 
    }

    pub fn full_outer_join(from: RcFrom, predicate: RcPredicate) -> Join { 
        ConditionedJoin{ join_type: FullOuterJoin, from: from, on: predicate } 
    }

    pub fn right_outer_join(from: RcFrom, predicate: RcPredicate) -> Join { 
        ConditionedJoin{ join_type: RightOuterJoin, from: from, on: predicate } 
    }

    pub fn left_outer_join(from: RcFrom, predicate: RcPredicate) -> Join { 
        ConditionedJoin{ join_type: LeftOuterJoin, from: from, on: predicate } 
    }

    pub fn full_join(from: RcFrom, predicate: RcPredicate) -> Join { 
        ConditionedJoin{ join_type: FullJoin, from: from, on: predicate } 
    }

    pub fn left_join(from: RcFrom, predicate: RcPredicate) -> Join { 
        ConditionedJoin{ join_type: LeftJoin, from: from, on: predicate } 
    }

    pub fn right_join(from: RcFrom, predicate: RcPredicate) -> Join { 
        ConditionedJoin{ join_type: RightJoin, from: from, on: predicate } 
    }

    pub fn natural_join(from: RcFrom) -> Join { 
        UnconditionedJoin{ join_type: NaturalJoin, from: from } 
    }

    pub fn natural_left_join(from: RcFrom) -> Join { 
        UnconditionedJoin{ join_type: NaturalLeftJoin, from: from } 
    }

    pub fn natural_right_join(from: RcFrom) -> Join { 
        UnconditionedJoin{ join_type: NaturalRightJoin, from: from } 
    }

    pub fn natural_full_join(from: RcFrom) -> Join { 
        UnconditionedJoin{ join_type: NaturalFullJoin, from: from } 
    }

    pub fn cross_join(from: RcFrom) -> Join { 
        UnconditionedJoin{ join_type: CrossJoin, from: from } 
    }

}