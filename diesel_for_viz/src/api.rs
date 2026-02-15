use crate::{
    database::DbPool,
    entity::users::{NewUser, UserUpdate, Users},
    schema::{self},
};
use diesel::{
    ExpressionMethods, RunQueryDsl, SelectableHelper,
    query_dsl::methods::{FilterDsl, SelectDsl},
};
use serde::{Deserialize, Serialize};
use vidi::{
    Request, RequestExt, Response, ResponseExt, Result,
    types::{Json, Params, State},
};
//index
pub async fn index(_: Request) -> Result<&'static str> {
    Ok("this is Index")
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    name: String,
    age: i32,
}
//create
pub async fn create(mut req: Request) -> Result<Response> {
    let (State(db), Json(cr)) = req.extract::<(State<DbPool>, Json<UserRequest>)>().await?;
    let new_user = NewUser {
        name: cr.name,
        age: cr.age,
    };
    let mut conn = db.get().expect("DePool Error");
    let content = diesel::insert_into(schema::users::dsl::users)
        .values(new_user)
        .returning(Users::as_returning())
        .get_result(&mut conn)
        .expect("Conn Error");
    Ok(Response::json(content)?)
}
#[derive(Debug, Serialize, Deserialize)]
struct UserUpdateRequest {
    name: String,
    age: i32,
}
///update
pub async fn update(mut req: Request) -> Result<Response> {
    let (State(db), Json(cr), Params(id)) = req
        .extract::<(State<DbPool>, Json<UserUpdateRequest>, Params<i32>)>()
        .await?;
    let mut conn = db.get().expect("DbPool IS Error");
    let count = diesel::update(schema::users::dsl::users.filter(schema::users::dsl::id.eq(id)))
        .set(&UserUpdate {
            name: Some(cr.name),
            age: Some(cr.age),
        })
        .execute(&mut conn)
        .unwrap();
    if count == 0 {
        return Ok(Response::json("Conn Error")?);
    }
    Ok(Response::json("Ok")?)
}

///list
#[derive(Debug, Serialize, Deserialize)]
struct ListRequest {
    ids: Vec<i32>,
}
pub async fn list(mut req: Request) -> Result<Response> {
    let (State(db), Json(cr)) = req.extract::<(State<DbPool>, Json<ListRequest>)>().await?;
    let mut conn = db.get().unwrap();
    let data_list = schema::users::dsl::users
        .filter(schema::users::dsl::id.eq_any(cr.ids))
        .select(Users::as_select())
        .load(&mut conn)
        .unwrap();
    if data_list.len() <= 0 {
        return Ok(Response::json("Data Error Is")?);
    }
    Ok(Response::json(data_list)?)
}
///remove
pub async fn remove(mut req: Request) -> Result<Response> {
    let (State(db), Json(cr)) = req.extract::<(State<DbPool>, Json<ListRequest>)>().await?;
    let mut conn = db.get().expect("DePool Error");
    let count =
        diesel::delete(schema::users::dsl::users.filter(schema::users::dsl::id.eq_any(cr.ids)))
            .execute(&mut conn)
            .expect("Conn Error");
    if count == 0 {
        return Ok(Response::json("Conn Error")?);
    }
    Ok(Response::json("Ok")?)
}
