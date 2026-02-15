use crate::entity::users::{NewUser, Users};
use crate::schema;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn create_db_pool() -> DbPool {
    let db_url = "postgres://postgres:123456@localhost:5432/rust4";
    let manager: ConnectionManager<PgConnection> = ConnectionManager::<PgConnection>::new(db_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}
pub fn create_user(new_user: NewUser) {
    let mut conn: diesel::r2d2::PooledConnection<ConnectionManager<PgConnection>> =
        create_db_pool().get().unwrap();
    let content = diesel::insert_into(schema::users::dsl::users)
        .values(&new_user)
        .returning(Users::as_returning())
        .get_result(&mut conn)
        .unwrap();
    println!("成功插入: ID = {}", content.id);
}
