use diesel::associations::HasTable;
use diesel::helper_types::{Find, Limit};
use diesel::prelude::*;
use diesel::query_builder::{
    AsChangeset, DeleteStatement, InsertStatement, IntoUpdateTarget, QueryFragment,
};
use diesel::query_dsl::limit_dsl::LimitDsl;
use diesel::query_dsl::methods::{ExecuteDsl, FindDsl};
use diesel::query_dsl::LoadQuery;
use diesel::Connection;

type Tab<E> = <E as HasTable>::Table;
type Backend<T> = <T as Connection>::Backend;
type DeleteFindStatement<F> =
    DeleteStatement<<F as HasTable>::Table, <F as IntoUpdateTarget>::WhereClause>;

pub trait AbstractRepository<Entity, ID>
where
    Self::Conn: Connection,
{
    type Conn;

    fn connection(&self) -> &Self::Conn;
}

pub trait CrudRepository<'t, Entity: 't, ID>: AbstractRepository<Entity, ID>
where
    Entity: HasTable,
    Tab<Entity>: FindDsl<ID>,
{
    fn delete(&self, entity: Entity) -> QueryResult<usize>
    where
        ID: Clone,
        for<'a> &'a Entity: Identifiable<Id = &'a ID>,
        Find<Tab<Entity>, ID>: IntoUpdateTarget,
        DeleteFindStatement<Find<Tab<Entity>, ID>>: ExecuteDsl<Self::Conn>,
    {
        diesel::delete(QueryDsl::find(Entity::table(), entity.id().clone()))
            .execute(self.connection())
    }

    fn delete_by_id(&self, id: ID) -> QueryResult<usize>
    where
        Find<Tab<Entity>, ID>: IntoUpdateTarget,
        DeleteFindStatement<Find<Tab<Entity>, ID>>: ExecuteDsl<Self::Conn>,
    {
        diesel::delete(QueryDsl::find(Entity::table(), id)).execute(self.connection())
    }

    fn find_by_id(&self, id: ID) -> QueryResult<Option<Entity>>
    where
        Find<Tab<Entity>, ID>: LimitDsl + Table,
        Limit<Find<Tab<Entity>, ID>>: LoadQuery<Self::Conn, Entity>,
    {
        diesel::QueryDsl::find(Entity::table(), id)
            .first::<Entity>(self.connection())
            .optional()
    }

    fn find_all(&self) -> QueryResult<Vec<Entity>>
    where
        Tab<Entity>: LoadQuery<Self::Conn, Entity>,
    {
        Entity::table().load(self.connection())
    }

    fn insert<N>(&self, new_entity: N) -> QueryResult<Entity>
    where
        N: Insertable<Tab<Entity>>,
        InsertStatement<Tab<Entity>, <N as Insertable<Tab<Entity>>>::Values>:
            RunQueryDsl<Self::Conn> + LoadQuery<Self::Conn, Entity>,
    {
        diesel::insert_into(Entity::table())
            .values(new_entity)
            .get_result(self.connection())
    }

    fn update<V>(&self, entity: &'t Entity) -> QueryResult<usize>
    where
        Self::Conn: Connection,
        &'t Entity: Identifiable<Table = Tab<Entity>> + AsChangeset<Target = Tab<Entity>>,
        Tab<Entity>: FindDsl<<&'t Entity as Identifiable>::Id>,
        Find<Tab<Entity>, <&'t Entity as Identifiable>::Id>:
            IntoUpdateTarget<Table = Tab<Entity>, WhereClause = V>,
        V: QueryFragment<Backend<Self::Conn>>,
        <&'t Entity as AsChangeset>::Changeset: QueryFragment<Backend<Self::Conn>>,
        <Tab<Entity> as QuerySource>::FromClause: QueryFragment<Backend<Self::Conn>>,
    {
        diesel::update(entity)
            .set(entity)
            .execute(self.connection())
    }
}
