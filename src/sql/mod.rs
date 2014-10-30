
pub use self::predicate::{PredicateToSql};
pub use self::value::{ToPredicateValue};
pub use self::from::{FromToSql};

pub mod delete;
pub mod distinct;
pub mod expr_value;
pub mod field;
pub mod function;
pub mod group_by;
pub mod insert;
pub mod join;
pub mod order_by;
pub mod placeholder;
pub mod predicate;
pub mod select;
pub mod from;
pub mod update;
pub mod value;

pub trait QueryToSql {
    fn to_final_sql(&self, &mut SqlContext) -> String;
}

pub trait ToSql {
    fn to_sql(&self, ctx: &mut SqlContext) -> String;
}

#[allow(dead_code)]
pub struct SqlContext {
    impl_placeholders: uint,
    expl_placeholders: uint,
    placeholder_data: Vec<Box<ToPredicateValue + Send + Sync>>
}

#[allow(dead_code)]
impl SqlContext {
    pub fn new() -> SqlContext {
        SqlContext {
            impl_placeholders: 0,
            expl_placeholders: 0,
            placeholder_data: vec![]
        }
    }
}
