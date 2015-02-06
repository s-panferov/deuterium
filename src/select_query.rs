use std::rc;
use std::mem;

use super::from;
use super::expression::{self, UntypedExpression};
use super::predicate::{self, ToOrPredicate, ToAndPredicate, ToExcludePredicate};
use super::sql;
use super::order_by;
use super::join;
use super::distinct;
use super::group_by;

#[derive(Clone)]
pub enum Select {
    Only(Vec<expression::RcExpression>),
    All
}

pub trait AbstractSelectQuery: sql::ToSql {
    
}

pub trait ToSelectQuery: sql::ToSql {
    fn upcast(&self) -> RcSelectQuery;
}

impl<T> ToSelectQuery for T where T: AbstractSelectQuery + Clone + 'static {
    fn upcast(&self) -> RcSelectQuery {
        rc::Rc::new(Box::new(self.clone()) as BoxedSelectQuery)
    }
}

#[derive(Clone, Copy)]
pub enum SelectFor {
    Update,
    UpdateNoWait,
    Share,
    ShareNoWait
}

#[derive(Clone, Copy)]
pub struct LimitOne;

#[derive(Clone, Copy)]
pub struct LimitTwo;

#[derive(Clone, Copy)]
pub struct LimitMany;

#[derive(Clone, Copy)]
pub struct NoResult;

macro_rules! set_predicate {
    ($s:ident, $getter:ident, $setter:ident, $pr:expr, $w:ident, $new_pr:expr) => ({
        let mut query = $s.clone();
        match $s.$getter() {
            &Some(ref $w) => {
                query.$setter($new_pr);
            },
            &None => {
                query.$setter($pr);
            }
        }
        query
    })
}

macro_rules! predicate_trait {
    (
        $name:ident, 
        $getter:ident, 
        $setter:ident, 
        $unset:ident,
        $implicit_and:ident,
        $explicit_and:ident,
        $or:ident,
        $exclude:ident,
        $and_exclude:ident,
        $or_exclude:ident
    ) => (

        pub trait $name: Clone {
            fn $getter(&self) -> &Option<predicate::RcPredicate>;
            fn $setter(&mut self, predicate::RcPredicate);
            fn $unset(&mut self);

            fn $implicit_and(&self, predicate: predicate::RcPredicate) -> Self {
                set_predicate!(self, $getter, $setter, predicate, w, w.and(predicate))
            }

            fn $or(&self, predicate: predicate::RcPredicate) -> Self {
                set_predicate!(self, $getter, $setter, predicate, w, w.or(predicate))
            }

            fn $explicit_and(&self, predicate: predicate::RcPredicate) -> Self {
                self.$implicit_and(predicate)
            }

            fn $exclude(&self, predicate: predicate::RcPredicate) -> Self {
               set_predicate!(self, $getter, $setter, predicate.exclude(), w, w.and(predicate.exclude()))
            }

            fn $and_exclude(&self, predicate: predicate::RcPredicate) -> Self {
               self.$exclude(predicate)
            }

            fn $or_exclude(&self, predicate: predicate::RcPredicate) -> Self {
               set_predicate!(self, $getter, $setter, predicate.exclude(), w, w.or(predicate.exclude()))
            }
        }

    )
}

predicate_trait!(
    Queryable,
    get_where,
    set_where,
    unset_where,
    where_,
    and,
    or,
    exclude,
    and_exclude,
    or_exclude
);

predicate_trait!(
    HasHaving,
    get_having,
    set_having,
    unset_having,
    having,
    and_having,
    or_having,
    exclude_having,
    and_exclude_having,
    or_exclude_having
);

pub trait Orderable: Clone {
    fn get_order_by_mut(&mut self) -> &mut Vec<order_by::OrderBy>;
    fn set_order_by(&mut self, Vec<order_by::OrderBy>);

    fn order_by(&self, field: &expression::UntypedExpression) -> Self {
        with_clone!(self, query, query.set_order_by(
            vec![order_by::OrderBy::by(field)]
        ))
    }

    fn order_by_fields(&self, fields: &[&expression::UntypedExpression]) -> Self {
        with_clone!(self, query, query.set_order_by(
            fields.iter().map(|f| order_by::OrderBy::by(*f)).collect()
        ))
    }

    fn reverse_by(&self, field: &expression::UntypedExpression) -> Self {
        with_clone!(self, query, query.set_order_by(
            vec![order_by::OrderBy::reverse_by(field)]
        ))
    }

    fn reverse_by_fields(&self, fields: &[&expression::UntypedExpression]) -> Self {
        with_clone!(self, query, query.set_order_by(
            fields.iter().map(|f| order_by::OrderBy::reverse_by(*f)).collect()
        ))
    }

    fn order_append(&self, field: &expression::UntypedExpression) -> Self {
        with_clone!(self, query, query.get_order_by_mut().push(order_by::OrderBy::by(field)))
    }

    fn order_prepend(&self, field: &expression::UntypedExpression) -> Self {
        with_clone!(self, query, query.get_order_by_mut().insert(0, order_by::OrderBy::by(field)))
    }

    fn reverse_append(&self, field: &expression::UntypedExpression) -> Self {
        with_clone!(self, query, query.get_order_by_mut().push(order_by::OrderBy::reverse_by(field)))
    }

    fn reverse_prepend(&self, field: &expression::UntypedExpression) -> Self {
        with_clone!(self, query, query.get_order_by_mut().insert(0, order_by::OrderBy::reverse_by(field)))
    }

    fn unorder(&self) -> Self {
        with_clone!(self, query, query.set_order_by(vec![]))
    }

}

#[derive(Clone)]
pub struct SelectQuery<T, L, M> {
    pub distinct: Option<distinct::Distinct>,
    pub select: Select,
    pub from: from::RcFrom,
    pub joins: Vec<join::Join>,
    pub where_: Option<predicate::RcPredicate>,
    pub group_by: Option<group_by::GroupBy>,
    pub having: Option<predicate::RcPredicate>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
    pub order_by: Vec<order_by::OrderBy>,
    pub for_: Option<SelectFor>,
}

impl<T: Clone, L: Clone, M: Clone> SelectQuery<T, L, M> {
 
    pub fn new(select: Select, from: from::RcFrom) -> SelectQuery<T, L, M> {
        SelectQuery {
            distinct: None,
            select: select,
            from: from,
            joins: vec![],
            where_: None,
            group_by: None,
            having: None,
            limit: None,
            offset: None,
            order_by: vec![],
            for_: None
        }
    }

    pub fn distinct(&self, ) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.distinct = Some(distinct::Distinct::new()))
    }

    pub fn distinct_on(&self, fields: &[&expression::UntypedExpression]) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.distinct = Some(distinct::Distinct::on(fields)))
    }

    pub fn group_by(&self, fields: &[&expression::UntypedExpression]) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.group_by = Some(group_by::GroupBy::new(fields)))
    }

    pub fn limit(&self, limit: usize) -> SelectQuery<T, LimitOne, M> {
        let mut query = self.clone();
        query.limit = Some(limit);
        unsafe{ mem::transmute(query) }
    }

    pub fn first(&self) -> SelectQuery<T, LimitOne, M> {
        let mut query = self.clone();
        query.limit = Some(1);
        unsafe{ mem::transmute(query) }
    }

    pub fn offset(&self, offset: usize) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.offset = Some(offset))
    }

    pub fn alias(&self, alias: &str) -> from::FromSelect<T, L, M> {
        from::FromSelect { select: self.clone(), alias: alias.to_string() }
    }

    pub fn from_as(&self, alias: &str) -> from::FromSelect<T, L, M> {
        self.alias(alias)
    }

    pub fn for_update(&self) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.for_ = Some(SelectFor::Update))
    }    

    pub fn for_update_nowait(&self) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.for_ = Some(SelectFor::UpdateNoWait))
    }

    pub fn for_share(&self) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.for_ = Some(SelectFor::Share))
    }

    pub fn for_share_nowait(&self) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.for_ = Some(SelectFor::ShareNoWait))
    }

    pub fn inner_join(&self, from: &from::From, on: predicate::RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::inner_join(from.upcast_from(), on)))
    }

    pub fn full_outer_join(&self, from: &from::From, on: predicate::RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::full_outer_join(from.upcast_from(), on)))
    }

    pub fn right_outer_join(&self, from: &from::From, on: predicate::RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::right_outer_join(from.upcast_from(), on)))
    }

    pub fn left_outer_join(&self, from: &from::From, on: predicate::RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::left_outer_join(from.upcast_from(), on)))
    }

    pub fn full_join(&self, from: &from::From, on: predicate::RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::full_join(from.upcast_from(), on)))
    }

    pub fn left_join(&self, from: &from::From, on: predicate::RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::left_join(from.upcast_from(), on)))
    }

    pub fn right_join(&self, from: &from::From, on: predicate::RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::right_join(from.upcast_from(), on)))
    }

    pub fn natural_join(&self, from: &from::From) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::natural_join(from.upcast_from())))
    }
    
    pub fn natural_left_join(&self, from: &from::From) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::natural_left_join(from.upcast_from())))
    }
    
    pub fn natural_right_join(&self, from: &from::From) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::natural_right_join(from.upcast_from())))
    }
    
    pub fn natural_full_join(&self, from: &from::From) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::natural_full_join(from.upcast_from())))
    }

    pub fn cross_join(&self, from: &from::From) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(join::Join::cross_join(from.upcast_from())))
    }

    pub fn unjoin(&self) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins = vec![])
    }
}

pub trait Selectable<M: Clone>: from::From {
    // FIXME: Unify select_N after [generics](https://github.com/rust-lang/rfcs/issues/376)

    fn select_1<T: Clone>(&self, field: &expression::Expression<T>) -> SelectQuery<(T,), LimitMany, M> {
        SelectQuery::new(Select::Only(vec![field.upcast_expression()]), self.upcast_from())
    }

    fn select_2<T1: Clone, T2: Clone>(&self, field1: &expression::Expression<T1>, field2: &expression::Expression<T2>) -> SelectQuery<(T1, T2), LimitMany, M> {
        SelectQuery::new(Select::Only(vec![field1.upcast_expression(), field2.upcast_expression()]), self.upcast_from())
    }

    fn select(&self, fields: &[&expression::UntypedExpression]) -> SelectQuery<(), LimitMany, M> {
        SelectQuery::new(Select::Only(fields.iter().map(|f| f.upcast_expression()).collect()), self.upcast_from())
    }

    fn select_all(&self) -> SelectQuery<(), LimitMany, M> {
        SelectQuery::new(Select::All, self.upcast_from())
    }

    fn exists(&self) -> SelectQuery<(), LimitMany, M> {
        SelectQuery::new(Select::Only(vec![expression::RawExpr::new("1").upcast_expression()]), self.upcast_from())
    }
}

impl<T: Clone, L: Clone, M: Clone> Queryable for SelectQuery<T, L, M> { 
    fn get_where(&self) -> &Option<predicate::RcPredicate> { &self.where_ }
    fn set_where(&mut self, predicate: predicate::RcPredicate) { self.where_ = Some(predicate); }
    fn unset_where(&mut self) { self.where_ = None; }
}

impl<T: Clone, L: Clone, M: Clone> HasHaving for SelectQuery<T, L, M> { 
    fn get_having(&self) -> &Option<predicate::RcPredicate> { &self.having }
    fn set_having(&mut self, predicate: predicate::RcPredicate) { self.having = Some(predicate); }
    fn unset_having(&mut self) { self.having = None; }
}

impl<T: Clone, L: Clone, M: Clone> Orderable for SelectQuery<T, L, M> { 
    fn get_order_by_mut(&mut self) -> &mut Vec<order_by::OrderBy> { &mut self.order_by }
    fn set_order_by(&mut self, order_by: Vec<order_by::OrderBy>) { self.order_by = order_by }
}

impl<T: Clone, L: Clone, M: Clone> AbstractSelectQuery for SelectQuery<T, L, M> { }

pub type BoxedSelectQuery = Box<AbstractSelectQuery + 'static>;
pub type RcSelectQuery = rc::Rc<BoxedSelectQuery>;

impl<T: Clone, L: Clone, M: Clone> expression::UntypedExpression for SelectQuery<T, L, M> {
    fn expression_as_sql(&self) -> &sql::ToSql {
                self
    }

    fn upcast_expression(&self) -> expression::RcExpression {
        rc::Rc::new(Box::new(self.clone()) as expression::BoxedExpression)
    }
}

impl<M: Clone, T: Clone> expression::Expression<T> for SelectQuery<(T,), LimitOne, M> { }
impl<M: Clone, T: Clone> expression::ListExpression<T> for SelectQuery<(T,), LimitMany, M> { }

impl<M: Clone, T: Clone> expression::ToExpression<T> for SelectQuery<(T,), LimitOne, M> { }
impl<M: Clone, T: Clone> expression::ToListExpression<T> for SelectQuery<(T,), LimitMany, M> { }
