
use std::sync::Arc;

use to_sql::{PredicateToSql};

pub use self::raw::{RawPredicate};
pub use self::is::{IsPredicate, ToIsPredicate};
pub use self::is_null::{IsNullPredicate, ToIsNullPredicate};
pub use self::or::{OrPredicate, ToOrPredicate};
pub use self::and::{AndPredicate, ToAndPredicate};
pub use self::exclude::{ExcludePredicate, ToExcludePredicate};
pub use self::like::{LikePredicate, ToLikePredicate};
pub use self::in_::{
    InPredicate, ToInPredicate
};

pub use self::range::{
    InRangePredicate, ToInRangePredicate,
    InRangeBounds, ExcludeBoth, IncludeBoth, ExcludeRight, ExcludeLeft
};

pub use self::inequality::{
    InequalityPredicate, ToInequalityPredicate,
    Inequality, LessThan, LessThanEqual, GreaterThan, GreaterThanEqual
};

pub mod is;
pub mod is_null;
pub mod or;
pub mod in_;
pub mod range;
pub mod and;
pub mod inequality;
pub mod exclude;
pub mod like;
pub mod raw;

pub trait Predicate: Sync + Send + PredicateToSql { 
    fn upcast(self) -> RcPredicate {
        Arc::new(box self as BoxedPredicate)
    }
}

impl ToOrPredicate for RcPredicate {
    fn or(&self, predicate: RcPredicate) -> RcPredicate {
        OrPredicate{ left: self.clone(), right: predicate }.upcast()
    }
}

impl ToAndPredicate for RcPredicate {
    fn and(&self, predicate: RcPredicate) -> RcPredicate {
        AndPredicate{ left: self.clone(), right: predicate }.upcast()
    }
}

pub type BoxedPredicate = Box<Predicate + Send + Sync>;
pub type RcPredicate = Arc<BoxedPredicate>;