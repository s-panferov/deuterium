use super::super::predicate::{
    self,
    is,
    is_null,
    or,
    raw,
    exclude,
    and,
    in_,
    like,
    range,
    inequality
};
use super::value::{self};

pub trait PredicateToSql {
    fn to_sql(&self, negation: bool, context: &mut super::SqlContext) -> String;
}

impl<F: value::ToPredicateValue, T: value::ToPredicateValue> PredicateToSql for is::IsPredicate<F, T> {
    fn to_sql(&self, negation: bool, ctx: &mut super::SqlContext) -> String {
        let op = if negation { "!=" } else { "=" };
        format!("{} {} {}", self.get_field().to_predicate_value(ctx), op, self.get_value().to_predicate_value(ctx))
    }
}

impl<F: value::ToPredicateValue> PredicateToSql for is_null::IsNullPredicate<F> {
    fn to_sql(&self, negation: bool, ctx: &mut super::SqlContext) -> String {
        let op = if !negation && self.is_null() { "IS NULL" } else { "IS NOT NULL" };
        format!("{} {}", self.get_field().to_predicate_value(ctx), op)
    }
}

impl PredicateToSql for predicate::SharedPredicate {
    fn to_sql(&self, negation: bool, ctx: &mut super::SqlContext) -> String {
        (**self).to_sql(negation, ctx)
    }
}

impl PredicateToSql for or::OrPredicate {
    fn to_sql(&self, negation: bool, ctx: &mut super::SqlContext) -> String {
        let left = self.left.to_sql(negation, ctx);
        let right = self.right.to_sql(negation, ctx);
        if !negation {
            format!("({}) OR ({})", left, right)
        } else {
            format!("({}) AND ({})", left, right)
        }
    }
}

impl PredicateToSql for raw::RawPredicate {
    fn to_sql(&self, negation: bool, _ctx: &mut super::SqlContext) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        format!("{}{}", maybe_not, self.content.to_string())
    }
}

impl PredicateToSql for exclude::ExcludePredicate {
    fn to_sql(&self, negation: bool, ctx: &mut super::SqlContext) -> String {
        self.predicate.to_sql(!negation, ctx)
    }
}

impl PredicateToSql for and::AndPredicate {
    fn to_sql(&self, negation: bool, ctx: &mut super::SqlContext) -> String {
        let left = self.left.to_sql(negation, ctx);
        let right = self.right.to_sql(negation, ctx);
        format!("({}) AND ({})", left, right)
    }
}

impl<F: value::ToPredicateValue, T: value::ToPredicateValue> PredicateToSql for in_::InPredicate<F, T> {
    fn to_sql(&self, negation: bool, ctx: &mut super::SqlContext) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        let values = self.get_values().to_predicate_value(ctx);
        format!("{} {}IN ({})", self.get_field().to_predicate_value(ctx), maybe_not, values)
    }
}

impl<F: value::ToPredicateValue, T: value::ToPredicateValue> PredicateToSql for like::LikePredicate<F, T> {
    fn to_sql(&self, negation: bool, ctx: &mut super::SqlContext) -> String {
        let maybe_not = if negation { "NOT " } else { "" };
        let like = if self.is_case_sensitive() { "LIKE" } else { "ILIKE" };
        let values = self.get_value().to_predicate_value(ctx);
        format!("{} {}{} {}", self.get_field().to_predicate_value(ctx), maybe_not, like, values)
    }
}

impl<F: value::ToPredicateValue, T1: value::ToPredicateValue, T2: value::ToPredicateValue> PredicateToSql for range::InRangePredicate<F, T1, T2> {
    fn to_sql(&self, negation: bool, ctx: &mut super::SqlContext) -> String {
        let ref name = self.get_field().to_predicate_value(ctx);
        let from = self.get_from().to_predicate_value(ctx);
        let to = self.get_to().to_predicate_value(ctx);
        match self.get_bounds() {
            &range::InRangeBounds::IncludeBoth => {
                if !negation {
                    format!("{} >= {} AND {} <= {}", name, from, name, to)
                } else {
                    format!("{} < {} OR {} > {}", name, from, name, to)
                }
            },
            &range::InRangeBounds::ExcludeBoth => {
                if !negation {
                    format!("{} > {} AND {} < {}", name, from, name, to)
                } else {
                    format!("{} <= {} OR {} >= {}", name, from, name, to)
                }
            },
            &range::InRangeBounds::ExcludeLeft => {
                if !negation {
                    format!("{} > {} AND {} <= {}", name, from, name, to)
                } else {
                    format!("{} <= {} OR {} > {}", name, from, name, to)
                }
            },
            &range::InRangeBounds::ExcludeRight => {
                if !negation {
                    format!("{} >= {} AND {} < {}", name, from, name, to)
                } else {
                    format!("{} < {} OR {} >= {}", name, from, name, to)
                }
            }
        }
    }
}

impl<F: value::ToPredicateValue, T: value::ToPredicateValue> PredicateToSql for inequality::InequalityPredicate<F, T> {
    fn to_sql(&self, negation: bool, ctx: &mut super::SqlContext) -> String {
        let ref name = self.get_field().to_predicate_value(ctx);
        let value = self.get_value().to_predicate_value(ctx);
        match self.get_inequality() {
            &inequality::Inequality::LessThan => {
                if !negation {
                    format!("{} < {}", name, value)
                } else {
                    format!("{} >= {}", name, value)
                }
            },
            &inequality::Inequality::LessThanEqual => {
                if !negation {
                    format!("{} <= {}", name, value)
                } else {
                    format!("{} > {}", name, value)
                }
            },
            &inequality::Inequality::GreaterThan => {
                if !negation {
                    format!("{} > {}", name, value)
                } else {
                    format!("{} <= {}", name, value)
                }
            },
            &inequality::Inequality::GreaterThanEqual => {
                if !negation {
                    format!("{} >= {}", name, value)
                } else {
                    format!("{} < {}", name, value)
                }
            }
        }
    }
}
