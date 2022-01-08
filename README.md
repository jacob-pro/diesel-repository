# Diesel Repository

[![Build status](https://github.com/jacob-pro/diesel-repository/actions/workflows/rust.yml/badge.svg)](https://github.com/jacob-pro/diesel-repository/actions)
![maintenance-status](https://img.shields.io/badge/maintenance-experimental-blue.svg)

An experimental attempt at using the repository pattern with Diesel.

## Example

```rust
#[derive(Debug, Queryable, Identifiable, AsChangeset)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

// UserRepositoryImpl = Generated struct name
// User               = The Entity type
// i32                = The Key type
// PgConnection       = The database connection type
implement_crud_repository!(UserRepositoryImpl, User, i32, PgConnection);

fn your_function(conn: &PgConnection) {
    let repository = UserRepositoryImpl::new(&conn);
    let user: Option<User> = repository.find_by_id(5).expect("Database error");
}
```
