
use std::sync::Arc;
use to_sql::{FromToSql};
use field::{Field, UntypedField};
use select_query::{SelectQuery, SelectOnly, SelectAll, LimitMany};

pub trait From: FromToSql + Clone { 
    fn upcast(&self) -> RcFrom;

    // FIXME: Unify select_N after [generics](https://github.com/rust-lang/rfcs/issues/376)

    fn select_1<T: Clone>(&self, field: &Field<T>) -> SelectQuery<(T), LimitMany, Self> {
        SelectQuery::new(SelectOnly(vec![field.to_def().clone_with_erase()]), self.upcast())
    }

    fn select_2<T1: Clone, T2: Clone>(&self, field1: &Field<T1>, field2: &Field<T2>) -> SelectQuery<(T1, T2), LimitMany, Self> {
        SelectQuery::new(SelectOnly(vec![field1.to_def().clone_with_erase(), field2.to_def().clone_with_erase()]), self.upcast())
    }

    fn select(&self, fields: &[&UntypedField]) -> SelectQuery<(), LimitMany, Self> {
        SelectQuery::new(SelectOnly(fields.iter().map(|f| f.to_def().clone_with_erase()).collect()), self.upcast())
    }

    fn select_all(&self) -> SelectQuery<(), LimitMany, Self> {
        SelectQuery::new(SelectAll, self.upcast())
    }
}

pub type BoxedFrom = Box<From + Send + Sync>;
pub type RcFrom = Arc<BoxedFrom>;

pub trait Table {
    fn upcast(&self) -> RcTable;
    fn get_table_name(&self) -> &String;
    fn get_table_alias(&self) -> &Option<String>;
}

pub type BoxedTable = Box<Table + Send + Sync>;
pub type RcTable = Arc<BoxedTable>;

#[deriving(Clone)]
pub struct TableDef {
    name: String,
    alias: Option<String>
}

impl TableDef {
    pub fn new(name: String) -> TableDef {
        TableDef { name: name, alias: None }
    }

    pub fn alias(&self, alias: String) -> TableDef {
        let mut table_def = self.clone();
        table_def.alias = Some(alias);
        table_def
    }
}

impl Table for TableDef {
    fn upcast(&self) -> RcTable {
        Arc::new(box self.clone() as BoxedTable)
    }

    fn get_table_name(&self) -> &String {
        &self.name
    }

    fn get_table_alias(&self) -> &Option<String> {
        &self.alias
    }
}

impl From for TableDef { 
    fn upcast(&self) -> RcFrom {
        Arc::new(box self.clone() as BoxedFrom)
    }
}

impl From for RcTable { 
    fn upcast(&self) -> RcFrom {
        Arc::new(box self.clone() as BoxedFrom)
    }
}

#[deriving(Clone)]
pub struct FromSelect<T, L, M> {
    pub select: SelectQuery<T, L, M>,
    pub alias: String 
}

impl<T: Clone, L: Clone, M: Clone> From for FromSelect<T, L, M> {
    fn upcast(&self) -> RcFrom {
        Arc::new(box self.clone() as BoxedFrom)
    }
}
