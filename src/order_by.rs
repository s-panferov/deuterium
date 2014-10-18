
use field::{Field};

#[deriving(Clone)]
pub enum Order {
    Asc,
    Desc
}

#[deriving(Clone)]
pub struct OrderBy {
    by: String,
    order: Order
}

impl OrderBy {
    pub fn by<T: Clone>(field: &Field<T>) -> OrderBy {
        OrderBy {
            by: field.to_def().name(),
            order: Asc
        }
    }

    pub fn reverse_by<T: Clone>(field: &Field<T>) -> OrderBy {
        OrderBy {
            by: field.to_def().name(),
            order: Desc
        }
    }

    pub fn get_by(&self) -> &str {
        self.by.as_slice()
    }

    pub fn get_order(&self) -> &Order {
        &self.order
    }
}