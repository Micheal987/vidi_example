use crate::schema;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize)]
#[diesel(table_name = schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub name: String,
    pub age: i32,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = schema::users)]
pub struct NewUser {
    pub name: String,
    pub age: i32,
}

#[derive(AsChangeset, Debug)]
#[diesel(table_name = schema::users)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub age: Option<i32>,
}
