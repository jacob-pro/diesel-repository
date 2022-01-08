use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use diesel::query_dsl::LoadQuery;
use diesel::Connection;

type Tab<E> = <E as HasTable>::Table;
type Backend<C> = <C as Connection>::Backend;

pub trait CrudRepository<Entity, Id>
where
    Entity: HasTable,
{
    type Conn: Connection;

    fn connection(&self) -> &Self::Conn;
    fn delete(&self, entity: Entity) -> QueryResult<bool>;
    fn delete_by_id(&self, id: Id) -> QueryResult<bool>;
    fn find_by_id(&self, id: Id) -> QueryResult<Option<Entity>>;
    fn find_all(&self) -> QueryResult<Vec<Entity>>;
    fn update(&self, entity: &Entity) -> QueryResult<usize>;
    fn count(&self) -> QueryResult<u64>;

    /// Insert and return the row (only supported on some databases)
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

    /// Insert without returning the row
    fn insert_only<N>(&self, new_entity: N) -> QueryResult<usize>
    where
        N: diesel::Insertable<Tab<Entity>>,
        Tab<Entity>: Insertable<Tab<Entity>>,
        <N as diesel::Insertable<Tab<Entity>>>::Values: diesel::insertable::CanInsertInSingleQuery<Backend<Self::Conn>>
            + diesel::query_builder::QueryFragment<Backend<Self::Conn>>,
        <Tab<Entity> as diesel::QuerySource>::FromClause:
            diesel::query_builder::QueryFragment<Backend<Self::Conn>>,
    {
        diesel::insert_into(Entity::table())
            .values(new_entity)
            .execute(self.connection())
    }
}

#[macro_export]
macro_rules! implement_crud_repository {
    ( $name:ident, $entity:ty, $key:ty, $conn:ty ) => {
        pub struct $name<'l>(&'l $conn);

        impl<'l> $name<'l> {
            pub fn new(connection: &'l $conn) -> Self {
                Self(connection)
            }
        }

        impl diesel_repository::CrudRepository<$entity, $key> for $name<'_> {
            type Conn = $conn;

            fn connection(&self) -> &Self::Conn {
                &self.0
            }
            fn delete(&self, entity: $entity) -> diesel::QueryResult<bool> {
                use diesel::associations::HasTable;
                use diesel::prelude::*;
                diesel::delete(<$entity>::table().find(entity.id()))
                    .execute(self.connection())
                    .map(|affected| {
                        assert!(affected <= 1);
                        affected > 0
                    })
            }
            fn delete_by_id(&self, id: $key) -> diesel::QueryResult<bool> {
                use diesel::associations::HasTable;
                use diesel::prelude::*;
                diesel::delete(<$entity>::table().find(id))
                    .execute(self.connection())
                    .map(|affected| {
                        assert!(affected <= 1);
                        affected > 0
                    })
            }
            fn find_by_id(&self, id: $key) -> diesel::QueryResult<Option<$entity>> {
                use diesel::associations::HasTable;
                use diesel::prelude::*;
                <$entity>::table()
                    .find(id)
                    .first::<$entity>(self.connection())
                    .optional()
            }
            fn find_all(&self) -> diesel::QueryResult<Vec<$entity>> {
                use diesel::associations::HasTable;
                use diesel::prelude::*;
                <$entity>::table().load(self.connection())
            }
            fn update(&self, entity: &$entity) -> diesel::QueryResult<usize> {
                use diesel::prelude::*;
                diesel::update(entity)
                    .set(entity)
                    .execute(self.connection())
            }
            fn count(&self) -> diesel::QueryResult<u64> {
                use diesel::associations::HasTable;
                use std::convert::TryFrom;
                use diesel::prelude::*;
                <$entity>::table()
                    .count()
                    .first(self.connection())
                    .map(|x: i64| u64::try_from(x).unwrap())
            }
        }
    };
}
