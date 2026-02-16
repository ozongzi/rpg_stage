#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    rpg_stage::run().await;
}
