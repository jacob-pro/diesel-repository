#[macro_use]
extern crate diesel_migrations;
embed_migrations!();
#[macro_use]
extern crate diesel;

mod schema;

#[cfg(test)]
mod test {
    use super::schema::*;
    use super::*;
    use diesel::sqlite::SqliteConnection;
    use diesel::Connection;
    use diesel_repository::{implement_crud_repository, CrudRepository};

    #[derive(Queryable, Identifiable, AsChangeset, PartialEq, Debug)]
    pub struct Test {
        id: i32,
        field1: String,
        field2: bool,
    }

    #[derive(Insertable)]
    #[table_name = "tests"]
    pub struct NewTest {
        field1: String,
        field2: bool,
    }

    implement_crud_repository!(TestsRepositoryImpl, Test, i32, SqliteConnection);

    #[test]
    fn test() {
        let conn = SqliteConnection::establish(":memory:").unwrap();
        embedded_migrations::run_with_output(&conn, &mut std::io::stdout()).unwrap();
        let repository = TestsRepositoryImpl::new(&conn);
        repository
            .insert_only(NewTest {
                field1: "Hello world".to_string(),
                field2: true,
            })
            .unwrap();
        let items = repository.find_all().unwrap();
        assert_eq!(items.first().unwrap().field1, "Hello world");
        let mut item = repository
            .find_by_id(items.first().unwrap().id)
            .unwrap()
            .unwrap();
        assert_eq!(&item, items.first().unwrap());
        item.field2 = false;
        repository.update(&item).unwrap();
        let item = repository.find_by_id(item.id).unwrap().unwrap();
        assert_eq!(item.field2, false);
        assert_eq!(repository.count().unwrap(), 1);
        assert_eq!(repository.delete_by_id(item.id).unwrap(), true);
        assert_eq!(repository.delete(item).unwrap(), false);
        assert_eq!(repository.count().unwrap(), 0);
    }
}
