use crate::api::index;
use crate::api::*;
use crate::database;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use vidi::{Result, Router, serve, types::State};

pub async fn run_service() -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;
    let state = database::create_db_pool();
    //addr
    println!("listening on http://{addr}");
    //router
    let app = Router::new()
        .get("/", index)
        .get("/create", create)
        .get("/list", list)
        .put("/update/:id", update)
        .delete("/remove", remove)
        .with(State(state));
    //err
    if let Err(e) = serve(listener, app).await {
        println!("{e}");
    }
    //()
    Ok(())
}
