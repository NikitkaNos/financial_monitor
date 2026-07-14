use financial_monitor::configuration::get_configuration;
use financial_monitor::run;
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL не найден");
    println!("Database URL: {}", database_url);
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to DB");

    let listener = TcpListener::bind("127.0.0.1:8000")?;
    let server = run(listener, pool).await?;
    server.await?;

    Ok(())
}
