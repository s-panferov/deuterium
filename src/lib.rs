
use std::rc::Rc;

#[deriving(Clone)]
pub enum From {
    NotDefinedFrom,
    DataSetFrom(Box<DataSet>),
    NameFrom(String)
}

#[deriving(Clone)]
pub enum Select {
    SelectOnly(Vec<RcField>),
    SelectAll
}

#[deriving(Clone)]
pub struct DataSet {
    from: From,
    select: Select
}

pub trait Field: Send + Sync + Clone {
    fn name(&self) -> &str;
}

pub type BoxedField = Box<Field>;
pub type RcField = Rc<BoxedField>;

impl DataSet {
 
    pub fn new() -> DataSet {
        DataSet {
            from: NotDefinedFrom,
            select: SelectAll
        }
    }

    pub fn select(&self, only: Vec<RcField>) -> DataSet {
        let mut dset = self.clone();
        dset.select = SelectOnly(only);
        dset
    }

    pub fn select_all(&self) -> DataSet {
        let mut dset = self.clone();
        dset.select = SelectAll;
        dset
    }

}

#[deriving(Clone)]
struct StringField;

impl Field for StringField {
    fn name(&self) -> &str {
        return "Name";
    }
}

#[test]
fn it_works() {

}
