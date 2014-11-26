
use sql::{SqlContext};
use predicate::{
    RcPredicate, 
    RawPredicate,
    IsPredicate, 
    OrPredicate, 
    AndPredicate,
    InPredicate,
    InRangePredicate, InRangeBounds,
    InequalityPredicate, Inequality,
    ExcludePredicate,
    LikePredicate,
    IsNullPredicate
};

use sql::value::{ToPredicateValue};

pub trait PredicateToSql {
    fn to_sql(&self, bool, &mut SqlContext) -> String;
}

impl<F: ToPredicateValue, T: ToPredicateValue> PredicateToSql for IsPredicate<F, T> {
    fn to_sql(&self, negation: bool, ctx: &mut SqlContext) -> String {
        let op = if negation { "!=" } else { "=" };
        format!("{} {} {}", self.field.to_predicate_value(ctx), op, self.value.to_predicate_value(ctx))
    }
}

impl<F: ToPredicateValue> PredicateToSql for IsNullPredicate<F> {
    fn to_sql(&self, negation: bool, ctx: &mut SqlContext) -> String {
        let op = if !negation && self.null { "IS NULL" } else { "IS NOT NULL" };
        format!("{} {}", self.field.to_predicate_value(ctx), op)
    }
}

impl PredicateToSql for RcPredicate {
    fn to_sql(&self, negation: bool, ctx: &mut SqlContext) -> String {
        (**self).to_sql(negation, ctx)
    }
}

impl PredicateToSql for OrPredicate {
    fn to_sql(&self, negation: bool, ctx: &mut SqlContext) -> String {
        let left = self.left.to_sql(negation, ctx);
        let right = self.right.to_sql(negation, ctx);
        if !negation {
            format!("({}) OR ({})", left, right)
        } else {
            format!("({}) AND ({})", left, right)
        }
    }
}

impl PredicateToSql for RawPredicate {
    fn to_sql(&self, negation: bool, _ctx: &mut SqlContext) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        format!("{}{}", maybe_not, self.content.to_string())
    }
}

impl PredicateToSql for ExcludePredicate {
    fn to_sql(&self, negation: bool, ctx: &mut SqlContext) -> String {
        self.predicate.to_sql(!negation, ctx)
    }
}

impl PredicateToSql for AndPredicate {
    fn to_sql(&self, negation: bool, ctx: &mut SqlContext) -> String {
        let left = self.left.to_sql(negation, ctx);
        let right = self.right.to_sql(negation, ctx);
        format!("({}) AND ({})", left, right)
    }
}

impl<F: ToPredicateValue, T: ToPredicateValue> PredicateToSql for InPredicate<F, T> {
    fn to_sql(&self, negation: bool, ctx: &mut SqlContext) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        let values = self.values.to_predicate_value(ctx);
        format!("{} {}IN ({})", self.field.to_predicate_value(ctx), maybe_not, values)
    }
}

impl<F: ToPredicateValue, T: ToPredicateValue> PredicateToSql for LikePredicate<F, T> {
    fn to_sql(&self, negation: bool, ctx: &mut SqlContext) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        let like = if self.case_sensitive { "LIKE" } else { "ILIKE" };
        let values = self.value.to_predicate_value(ctx);
        format!("{} {}{} {}", self.field.to_predicate_value(ctx), maybe_not, like, values)
    }
}

impl<F: ToPredicateValue, T: ToPredicateValue> PredicateToSql for InRangePredicate<F, T> {
    fn to_sql(&self, negation: bool, ctx: &mut SqlContext) -> String {
        let ref name = self.field.to_predicate_value(ctx);
        let from = self.from.to_predicate_value(ctx); 
        let to = self.to.to_predicate_value(ctx);
        match self.bounds {
            InRangeBounds::IncludeBoth => {
                if !negation {
                    format!("{} >= {} AND {} <= {}", name, from, name, to)
                } else {
                    format!("{} < {} OR {} > {}", name, from, name, to)
                }
            },
            InRangeBounds::ExcludeBoth => {
                if !negation {
                    format!("{} > {} AND {} < {}", name, from, name, to)
                } else {
                    format!("{} <= {} OR {} >= {}", name, from, name, to)
                }
            },
            InRangeBounds::ExcludeLeft => {
                if !negation {
                    format!("{} > {} AND {} <= {}", name, from, name, to)
                } else {
                    format!("{} <= {} OR {} > {}", name, from, name, to)
                }
            },
            InRangeBounds::ExcludeRight => {
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
    fn to_sql(&self, negation: bool, ctx: &mut SqlContext) -> String {
        let ref name = self.field.to_predicate_value(ctx);
        let value = self.value.to_predicate_value(ctx);
        match self.inequality {
            Inequality::LessThan => {
                if !negation {
                    format!("{} < {}", name, value)
                } else {
                    format!("{} >= {}", name, value)
                }
            },
            Inequality::LessThanEqual => {
                if !negation {
                    format!("{} <= {}", name, value)
                } else {
                    format!("{} > {}", name, value)
                }
            },
            Inequality::GreaterThan => {
                if !negation {
                    format!("{} > {}", name, value)
                } else {
                    format!("{} <= {}", name, value)
                }
            },
            Inequality::GreaterThanEqual => {
                if !negation {
                    format!("{} >= {}", name, value)
                } else {
                    format!("{} < {}", name, value)
                }
            }
        }
    }
}
