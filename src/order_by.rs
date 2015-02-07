use expression;

#[derive(Clone)]
pub enum Order {
    Asc,
    Desc
}

#[derive(Clone)]
pub struct OrderBy {
    by: expression::SharedExpression,
    order: Order
}

impl OrderBy {
    pub fn by(expression: &expression::UntypedExpression) -> OrderBy {
        OrderBy {
            by: expression.upcast_expression(),
            order: Order::Asc
        }
    }

    pub fn reverse_by(expression: &expression::UntypedExpression) -> OrderBy {
        OrderBy {
            by: expression.upcast_expression(),
            order: Order::Desc
        }
    }

    pub fn get_by(&self) -> &expression::SharedExpression {
        &self.by
    }

    pub fn get_order(&self) -> &Order {
        &self.order
    }
}