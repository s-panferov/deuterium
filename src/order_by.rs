
use field::{RcExpression, UntypedExpression};

#[deriving(Clone)]
pub enum Order {
    Asc,
    Desc
}

#[deriving(Clone)]
pub struct OrderBy {
    by: RcExpression,
    order: Order
}

impl OrderBy {
    pub fn by(expression: &UntypedExpression) -> OrderBy {
        OrderBy {
            by: expression.upcast(),
            order: Asc
        }
    }

    pub fn reverse_by(expression: &UntypedExpression) -> OrderBy {
        OrderBy {
            by: expression.upcast(),
            order: Desc
        }
    }

    pub fn get_by(&self) -> &RcExpression {
        &self.by
    }

    pub fn get_order(&self) -> &Order {
        &self.order
    }
}