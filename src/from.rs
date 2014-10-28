
use std::sync::Arc;
use to_sql::{FromToSql};
use select_query::{SelectQuery, Selectable};

use field::{NamedField, Field};
use insert_query::{InsertQuery, Insertable, InsertValue};

pub trait From { 
    fn as_sql(&self) -> &FromToSql;
    fn upcast_from(&self) -> RcFrom;
}

pub type BoxedFrom = Box<From + Send + Sync>;
pub type RcFrom = Arc<BoxedFrom>;

pub trait Table: Clone {
    fn upcast_table(&self) -> RcTable;
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

// FIXME: Remove after all stuff in InsertQuery will be fixed
macro_rules! insert(
    ($name:ident, $(($t:ident, $arg:ident)),+) => (
        #[doc(hidden)]
        fn $name<$($t:Clone,)+>(&self, $($arg: &NamedField<$t>,)+) -> InsertQuery<($($t,)+), ($(InsertValue<$t>,)+), ()> {
            let mut cols = vec![];
            $(cols.push((*$arg).upcast_field());)+
            InsertQuery::new_with_cols(self, cols)
        }
    )
)

#[allow(dead_code)]
impl TableDef {
    pub fn new(name: &str) -> TableDef {
        TableDef { name: name.to_string(), alias: None }
    }

    pub fn new_with_alias(name: &str, alias: &str) -> TableDef {
        TableDef { name: name.to_string(), alias: Some(alias.to_string()) }
    }

    pub fn alias(&self, alias: &str) -> TableDef {
        let mut table_def = self.clone();
        table_def.alias = Some(alias.to_string());
        table_def
    }

    // FIXME: Remove after all stuff in InsertQuery will be fixed
    insert!(insert_1, (T0, _t0))
    #[doc(hidden)]
    pub fn insert_1_for_test(&self, name: &NamedField<String>) -> InsertQuery<(String,), (InsertValue<String>,), ()> {
        self.insert_1(name)
    }
}

impl Table for TableDef {
    fn upcast_table(&self) -> RcTable {
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
    fn as_sql(&self) -> &FromToSql {
        self
    }

    fn upcast_from(&self) -> RcFrom {
        Arc::new(box self.clone() as BoxedFrom)
    }
}

impl Selectable<()> for TableDef {}
impl Insertable<()> for TableDef {}

#[deriving(Clone)]
pub struct FromSelect<T, L, M> {
    pub select: SelectQuery<T, L, M>,
    pub alias: String 
}

impl<T: Clone, L: Clone, M: Clone> From for FromSelect<T, L, M> {
    fn as_sql(&self) -> &FromToSql {
        self
    }

    fn upcast_from(&self) -> RcFrom {
        Arc::new(box self.clone() as BoxedFrom)
    }
}

impl<T: Clone, L: Clone, M: Clone> Selectable<M> for FromSelect<T, L, M> {}

