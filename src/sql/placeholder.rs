use super::super::placeholder;
use super::value::{self, ToPredicateValue};

impl value::ToPredicateValue for placeholder::Placeholder {
    fn to_predicate_value(&self, ctx: &mut super::SqlContext) -> String {
        ctx.expl_indexed_placeholder(self.get_idx());
        ctx.adapter().placeholder(self.get_idx())
    }
}

impl super::ToSql for placeholder::Placeholder {
    fn to_sql(&self, ctx: &mut super::SqlContext) -> String {
        self.to_predicate_value(ctx)
    }
}
