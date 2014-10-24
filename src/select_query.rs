
use serialize::json::Json;
use time::Timespec;

use std::sync::Arc;
use std::mem;

use from::{From, RcFrom, FromSelect};
use expression::{Expression, UntypedExpression, RcExpression};
use predicate::{
    RcPredicate,
    ToOrPredicate,
    ToAndPredicate,
    ToExcludePredicate
};

use to_sql::{ToSql};
use order_by::{OrderBy};
use join::{Join};
use distinct::{Distinct};
use group_by::{GroupBy};

use field::{
    I8Comparable,
    I16Comparable,
    I32Comparable,
    I64Comparable,
    F32Comparable,
    F64Comparable,
    StringComparable,
    JsonComparable,
    TimespecComparable,

    I8ComparableList,
    I16ComparableList,
    I32ComparableList,
    I64ComparableList,
    F32ComparableList,
    F64ComparableList,
    StringComparableList,
    JsonComparableList,
    TimespecComparableList
};

#[deriving(Clone)]
pub enum Select {
    SelectOnly(Vec<RcExpression>),
    SelectAll
}

pub trait ToSelectQuery: Send + Sync + ToSql {
    fn upcast(self) -> RcSelectQuery {
        Arc::new(box self as BoxedSelectQuery)
    }
}

#[deriving(Clone)]
pub struct LimitOne;

#[deriving(Clone)]
pub struct LimitTwo;

#[deriving(Clone)]
pub struct LimitMany;

macro_rules! set_predicate(
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
)

macro_rules! predicate_trait(
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
            fn $getter(&self) -> &Option<RcPredicate>;
            fn $setter(&mut self, RcPredicate);
            fn $unset(&mut self);

            fn $implicit_and(&self, predicate: RcPredicate) -> Self {
                set_predicate!(self, $getter, $setter, predicate, w, w.and(predicate))
            }

            fn $or(&self, predicate: RcPredicate) -> Self {
                set_predicate!(self, $getter, $setter, predicate, w, w.or(predicate))
            }

            fn $explicit_and(&self, predicate: RcPredicate) -> Self {
                self.$implicit_and(predicate)
            }

            fn $exclude(&self, predicate: RcPredicate) -> Self {
               set_predicate!(self, $getter, $setter, predicate.exclude(), w, w.and(predicate.exclude()))
            }

            fn $and_exclude(&self, predicate: RcPredicate) -> Self {
               self.$exclude(predicate)
            }

            fn $or_exclude(&self, predicate: RcPredicate) -> Self {
               set_predicate!(self, $getter, $setter, predicate.exclude(), w, w.or(predicate.exclude()))
            }
        }

    )
)

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
)

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
)

macro_rules! with_clone(
    ($slf: ident, $v:ident, $ex:expr) => ({
        let mut $v = $slf.clone();
        $ex;
        $v
    })
)

pub trait Orderable: Clone {
    fn get_order_by_mut(&mut self) -> &mut Vec<OrderBy>;
    fn set_order_by(&mut self, Vec<OrderBy>);

    fn order_by(&self, field: &UntypedExpression) -> Self {
        with_clone!(self, query, query.set_order_by(
            vec![OrderBy::by(field)]
        ))
    }

    fn order_by_fields(&self, fields: &[&UntypedExpression]) -> Self {
        with_clone!(self, query, query.set_order_by(
            fields.iter().map(|f| OrderBy::by(*f)).collect()
        ))
    }

    fn reverse_by(&self, field: &UntypedExpression) -> Self {
        with_clone!(self, query, query.set_order_by(
            vec![OrderBy::reverse_by(field)]
        ))
    }

    fn reverse_by_fields(&self, fields: &[&UntypedExpression]) -> Self {
        with_clone!(self, query, query.set_order_by(
            fields.iter().map(|f| OrderBy::reverse_by(*f)).collect()
        ))
    }

    fn order_append(&self, field: &UntypedExpression) -> Self {
        with_clone!(self, query, query.get_order_by_mut().push(OrderBy::by(field)))
    }

    fn order_prepend(&self, field: &UntypedExpression) -> Self {
        with_clone!(self, query, query.get_order_by_mut().insert(0, OrderBy::by(field)))
    }

    fn reverse_append(&self, field: &UntypedExpression) -> Self {
        with_clone!(self, query, query.get_order_by_mut().push(OrderBy::reverse_by(field)))
    }

    fn reverse_prepend(&self, field: &UntypedExpression) -> Self {
        with_clone!(self, query, query.get_order_by_mut().insert(0, OrderBy::reverse_by(field)))
    }

    fn unorder(&self) -> Self {
        with_clone!(self, query, query.set_order_by(vec![]))
    }

}

#[deriving(Clone)]
pub struct SelectQuery<T, L, M> {
    pub distinct: Option<Distinct>,
    pub select: Select,
    pub from: RcFrom,
    pub joins: Vec<Join>,
    pub where_: Option<RcPredicate>,
    pub group_by: Option<GroupBy>,
    pub having: Option<RcPredicate>,
    pub limit: Option<uint>,
    pub offset: Option<uint>,
    pub order_by: Vec<OrderBy>,
}

impl<T: Clone, L: Clone, M: Clone> SelectQuery<T, L, M> {
 
    pub fn new(select: Select, from: RcFrom) -> SelectQuery<T, L, M> {
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
        }
    }

    pub fn distinct(&self, ) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.distinct = Some(Distinct::new()))
    }

    pub fn distinct_on(&self, fields: &[&UntypedExpression]) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.distinct = Some(Distinct::on(fields)))
    }

    pub fn group_by(&self, fields: &[&UntypedExpression]) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.group_by = Some(GroupBy::new(fields)))
    }

    pub fn limit(&self, limit: uint) -> SelectQuery<T, LimitOne, M> {
        let mut query = self.clone();
        query.limit = Some(limit);
        unsafe{ mem::transmute(query) }
    }

    pub fn first(&self) -> SelectQuery<T, LimitOne, M> {
        let mut query = self.clone();
        query.limit = Some(1);
        unsafe{ mem::transmute(query) }
    }

    pub fn offset(&self, offset: uint) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.offset = Some(offset))
    }

    pub fn alias(&self, alias: &str) -> FromSelect<T, L, M> {
        FromSelect { select: self.clone(), alias: alias.to_string() }
    }

    pub fn from_as(&self, alias: &str) -> FromSelect<T, L, M> {
        self.alias(alias)
    }

    pub fn inner_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::inner_join(from.upcast_from(), on)))
    }

    pub fn full_outer_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::full_outer_join(from.upcast_from(), on)))
    }

    pub fn right_outer_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::right_outer_join(from.upcast_from(), on)))
    }

    pub fn left_outer_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::left_outer_join(from.upcast_from(), on)))
    }

    pub fn full_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::full_join(from.upcast_from(), on)))
    }

    pub fn left_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::left_join(from.upcast_from(), on)))
    }

    pub fn right_join(&self, from: &From, on: RcPredicate) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::right_join(from.upcast_from(), on)))
    }

    pub fn natural_join(&self, from: &From) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::natural_join(from.upcast_from())))
    }
    
    pub fn natural_left_join(&self, from: &From) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::natural_left_join(from.upcast_from())))
    }
    
    pub fn natural_right_join(&self, from: &From) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::natural_right_join(from.upcast_from())))
    }
    
    pub fn natural_full_join(&self, from: &From) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::natural_full_join(from.upcast_from())))
    }

    pub fn cross_join(&self, from: &From) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins.push(Join::cross_join(from.upcast_from())))
    }

    pub fn unjoin(&self) -> SelectQuery<T, L, M> {
        with_clone!(self, query, query.joins = vec![])
    }
}

pub trait Selectable<M: Clone>: From {
    // FIXME: Unify select_N after [generics](https://github.com/rust-lang/rfcs/issues/376)

    fn select_1<T: Clone>(&self, field: &Expression<T>) -> SelectQuery<(T), LimitMany, M> {
        SelectQuery::new(SelectOnly(vec![field.upcast()]), self.upcast_from())
    }

    fn select_2<T1: Clone, T2: Clone>(&self, field1: &Expression<T1>, field2: &Expression<T2>) -> SelectQuery<(T1, T2), LimitMany, M> {
        SelectQuery::new(SelectOnly(vec![field1.upcast(), field2.upcast()]), self.upcast_from())
    }

    fn select(&self, fields: &[&UntypedExpression]) -> SelectQuery<(), LimitMany, M> {
        SelectQuery::new(SelectOnly(fields.iter().map(|f| f.upcast()).collect()), self.upcast_from())
    }

    fn select_all(&self) -> SelectQuery<(), LimitMany, M> {
        SelectQuery::new(SelectAll, self.upcast_from())
    }
}

impl<T: Clone, L: Clone, M: Clone> Queryable for SelectQuery<T, L, M> { 
    fn get_where(&self) -> &Option<RcPredicate> { &self.where_ }
    fn set_where(&mut self, predicate: RcPredicate) { self.where_ = Some(predicate); }
    fn unset_where(&mut self) { self.where_ = None; }
}

impl<T: Clone, L: Clone, M: Clone> HasHaving for SelectQuery<T, L, M> { 
    fn get_having(&self) -> &Option<RcPredicate> { &self.having }
    fn set_having(&mut self, predicate: RcPredicate) { self.having = Some(predicate); }
    fn unset_having(&mut self) { self.having = None; }
}

impl<T: Clone, L: Clone, M: Clone> Orderable for SelectQuery<T, L, M> { 
    fn get_order_by_mut(&mut self) -> &mut Vec<OrderBy> { &mut self.order_by }
    fn set_order_by(&mut self, order_by: Vec<OrderBy>) { self.order_by = order_by }
}

impl<T: Clone, L: Clone, M: Clone> ToSelectQuery for SelectQuery<T, L, M> { }

pub type BoxedSelectQuery = Box<ToSelectQuery + Send + Sync>;
pub type RcSelectQuery = Arc<BoxedSelectQuery>;

impl<M: Clone> I8Comparable for SelectQuery<(i8), LimitOne, M> { }
impl<M: Clone> I16Comparable for SelectQuery<(i16), LimitOne, M> { }
impl<M: Clone> I32Comparable for SelectQuery<(i32), LimitOne, M> { }
impl<M: Clone> I64Comparable for SelectQuery<(i64), LimitOne, M> { }
impl<M: Clone> F32Comparable for SelectQuery<(f32), LimitOne, M> { }
impl<M: Clone> F64Comparable for SelectQuery<(f64), LimitOne, M> { }
impl<M: Clone> StringComparable for SelectQuery<(String), LimitOne, M> { }
impl<M: Clone> JsonComparable for SelectQuery<(Json), LimitOne, M> { }
impl<M: Clone> TimespecComparable for SelectQuery<(Timespec), LimitOne, M> { }

impl<M: Clone> I8ComparableList for SelectQuery<(i8), LimitMany, M> { }
impl<M: Clone> I16ComparableList for SelectQuery<(i16), LimitMany, M> { }
impl<M: Clone> I32ComparableList for SelectQuery<(i32), LimitMany, M> { }
impl<M: Clone> I64ComparableList for SelectQuery<(i64), LimitMany, M> { }
impl<M: Clone> F32ComparableList for SelectQuery<(f32), LimitMany, M> { }
impl<M: Clone> F64ComparableList for SelectQuery<(f64), LimitMany, M> { }
impl<M: Clone> StringComparableList for SelectQuery<(String), LimitMany, M> { }
impl<M: Clone> JsonComparableList for SelectQuery<(Json), LimitMany, M> { }
impl<M: Clone> TimespecComparableList for SelectQuery<(Timespec), LimitMany, M> { }

