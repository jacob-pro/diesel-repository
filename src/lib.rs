use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::query_builder::InsertStatement;
use diesel::query_dsl::LoadQuery;
use diesel::Connection;

type Tab<E> = <E as HasTable>::Table;

pub trait CrudRepository<Entity, Id>
where
    Entity: HasTable,
{
    type Conn: Connection;

    fn connection(&self) -> &Self::Conn;
    fn delete(&self, entity: Entity) -> QueryResult<usize>;
    fn delete_by_id(&self, id: Id) -> QueryResult<usize>;
    fn find_by_id(&self, id: Id) -> QueryResult<Option<Entity>>;
    fn find_all(&self) -> QueryResult<Vec<Entity>>;
    fn update(&self, entity: &Entity) -> QueryResult<usize>;

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
            fn delete(&self, entity: $entity) -> QueryResult<usize> {
                use diesel::associations::HasTable;
                diesel::delete(<$entity>::table().find(entity.id())).execute(self.connection())
            }
            fn delete_by_id(&self, id: $key) -> QueryResult<usize> {
                use diesel::associations::HasTable;
                diesel::delete(<$entity>::table().find(id)).execute(self.connection())
            }
            fn find_by_id(&self, id: $key) -> QueryResult<Option<$entity>> {
                use diesel::associations::HasTable;
                <$entity>::table()
                    .find(id)
                    .first::<$entity>(self.connection())
                    .optional()
            }
            fn find_all(&self) -> QueryResult<Vec<$entity>> {
                use diesel::associations::HasTable;
                <$entity>::table().load(self.connection())
            }
            fn update(&self, entity: &$entity) -> QueryResult<usize> {
                diesel::update(entity)
                    .set(entity)
                    .execute(self.connection())
            }
        }
    };
}
