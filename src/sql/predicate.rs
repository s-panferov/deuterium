
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

use sql::value::{ToPredicateValue};

pub trait PredicateToSql {
    fn to_sql(&self, bool) -> String;
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
