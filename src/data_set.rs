
use {Select, From, NotDefinedFrom, SelectAll, SelectOnly};
use field::{Field};
use query::{RcQuery};

#[deriving(Clone)]
pub struct DataSet {
    from: From,
    select: Select,
    query: Option<RcQuery>
}

impl DataSet {
 
    pub fn new() -> DataSet {
        DataSet {
            from: NotDefinedFrom,
            select: SelectAll,
            query: None
        }
    }

    pub fn select(&self, only: &[&Field]) -> DataSet {
        let mut dset = self.clone();
        dset.select = SelectOnly(only.iter().map(|field| field.to_def()).collect());
        dset
    }

    pub fn select_all(&self) -> DataSet {
        let mut dset = self.clone();
        dset.select = SelectAll;
        dset
    }

    pub fn where_(&self, query: &RcQuery) -> DataSet {
        let mut dset = self.clone();
        dset.query = Some(query.clone());
        dset
    }

}
