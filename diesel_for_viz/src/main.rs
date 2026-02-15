#[tokio::main]
async fn main() {
    let _ = diesel_for_viz::router::run::run_service().await;
}
