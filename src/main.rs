use financial_monitor::configuration::get_configuration;
use financial_monitor::run;
use sqlx::PgPool;
use std::{fs::File, net::TcpListener};
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let log_file = File::create("logs.json").expect("Failed to create log file");
    let formatting_layer = BunyanFormattingLayer::new("financial_monitor".into(), log_file);
    let console_layer = fmt::layer().pretty();
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(console_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");

    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL не найден");
    tracing::info!("Database URL: {}", database_url);

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
