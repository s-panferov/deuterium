
use {Select, From, SelectAll, SelectOnly};
use field::{Field};
use query::{RcQuery};

#[deriving(Clone)]
pub struct SelectDataSet {
    pub select: Select,
    pub from: From,
    pub where_: Option<RcQuery>
}

impl SelectDataSet {
 
    pub fn new(select: Select, from: From) -> SelectDataSet {
        SelectDataSet {
            select: select,
            from: from,
            where_: None
        }
    }

    pub fn where_(&self, query: &RcQuery) -> SelectDataSet {
        let mut dset = self.clone();
        dset.where_ = Some(query.clone());
        dset
    }

}
