
use std::rc::Rc;

use sql::{PredicateToSql};

// pub use self::raw::{RawPredicate};
pub use self::is::{IsPredicate, ToIsPredicate};
pub use self::is_null::{IsNullPredicate, ToIsNullPredicate};
pub use self::or::{OrPredicate, ToOrPredicate};
pub use self::and::{AndPredicate, ToAndPredicate};
pub use self::exclude::{ExcludePredicate, ToExcludePredicate};
// pub use self::like::{LikePredicate, ToLikePredicate};
pub use self::in_::{
    InPredicate, ToInPredicate
};

pub use self::range::{
    InRangePredicate, ToInRangePredicate,
    InRangeBounds
};

// pub use self::inequality::{
//     InequalityPredicate, ToInequalityPredicate,
//     Inequality
// };

pub mod is;
pub mod is_null;
pub mod or;
pub mod in_;
pub mod range;
pub mod and;
// pub mod inequality;
pub mod exclude;
// pub mod like;
// pub mod raw;

pub trait Predicate: PredicateToSql { 

}

pub trait ToAbstractPredicate { 
    fn upcast(&self) -> RcPredicate;
}

impl<T> ToAbstractPredicate for T where T: Predicate + Clone + 'static {
    fn upcast(&self) -> RcPredicate {
        Rc::new(Box::new(self.clone()) as BoxedPredicate)
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

pub type BoxedPredicate = Box<Predicate + 'static>;
pub type RcPredicate = Rc<BoxedPredicate>;