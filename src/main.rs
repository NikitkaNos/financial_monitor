use financial_monitor::configuration::get_configuration;
use financial_monitor::run;
use financial_monitor::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::{fs::File, net::TcpListener};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. Настраиваем логирование
    let log_file = File::create("logs.json").expect("Failed to create log file");
    let subscriber = get_subscriber("financial_monitor".into(), "info".into(), Some(log_file));
    init_subscriber(subscriber);

    // 2. Загружаем .env
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL не найден");
    tracing::info!("Database URL: {}", database_url);

    // 3. Загружаем конфигурацию и подключаемся к БД
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to DB");

    // 4. Запускаем сервер
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    let server = run(listener, pool).await?;
    server.await?;

    Ok(())
}
