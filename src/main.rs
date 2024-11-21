use dotenv::dotenv;
use tracing_subscriber;
use untitled::infrastructure::system::database::pool;
use untitled::infrastructure::system::server::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    dotenv().ok();

    let pool = pool().await?;

    run(pool).await
}