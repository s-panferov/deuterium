
use placeholder::{
    Placeholder
};

use sql::{ToSql};
use sql::value::{ToPredicateValue};

impl ToPredicateValue for Placeholder {
    fn to_predicate_value(&self) -> String {
        format!("${}", self.idx)
    }
}

impl ToSql for Placeholder {
    fn to_sql(&self) -> String {
        self.to_predicate_value()
    }
}
