use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};

#[derive(Clone, Debug)]
pub struct AppSate {
    pub db: Pool<ConnectionManager<PgConnection>>,
}
impl AppSate {
    pub fn new(db: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self { db }
    }
}
