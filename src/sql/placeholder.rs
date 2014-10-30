
use placeholder::{
    Placeholder
};

use sql::{SqlContext, ToSql};
use sql::value::{ToPredicateValue};

impl ToPredicateValue for Placeholder {
    fn to_predicate_value(&self, _ctx: &mut SqlContext) -> String {
        format!("${}", self.idx)
    }
}

impl ToSql for Placeholder {
    fn to_sql(&self, ctx: &mut SqlContext) -> String {
        self.to_predicate_value(ctx)
    }
}
