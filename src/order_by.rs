
use field::{Field, FieldDef};

#[deriving(Clone)]
pub enum Order {
    Asc,
    Desc
}

#[deriving(Clone)]
pub struct OrderBy {
    by: FieldDef<()>,
    order: Order
}

impl OrderBy {
    pub fn by<T: Clone>(field: &Field<T>) -> OrderBy {
        OrderBy {
            by: field.to_def().clone_with_erase(),
            order: Asc
        }
    }

    pub fn reverse_by<T: Clone>(field: &Field<T>) -> OrderBy {
        OrderBy {
            by: field.to_def().clone_with_erase(),
            order: Desc
        }
    }

    pub fn get_by(&self) -> &FieldDef<()> {
        &self.by
    }

    pub fn get_order(&self) -> &Order {
        &self.order
    }
}