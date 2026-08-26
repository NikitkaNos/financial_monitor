use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::Layer;
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt};
/// Создаёт подписчика (subscriber) для логирования.
///
/// # Аргументы
/// - `name`: имя приложения (используется в Bunyan-логах)...
/// - `env_filter`: уровень логирования (например, "info", "debug", "warn").
/// - `log_file`: опциональный файл для записи логов в JSON-формате.
pub fn get_subscriber(
    name: String,
    env_filter: String,
    log_file: Option<std::fs::File>,
) -> impl Subscriber + Sync + Send {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let mut layers = Vec::new();

    // Если передан файл, добавляем Bunyan-слой (JSON)
    if let Some(file) = log_file {
        let bunyan_layer = BunyanFormattingLayer::new(name.clone(), file);
        layers.push(JsonStorageLayer.boxed());
        layers.push(bunyan_layer.boxed());
    }

    // Добавляем консольный слой (красивый вывод)
    let console_layer = tracing_subscriber::fmt::layer()
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .pretty()
        .boxed();
    layers.push(console_layer);

    Registry::default().with(env_filter).with(layers)
}

/// Инициализирует глобального подписчика.
pub fn init_subscriber(subscriber: impl Subscriber + Sync + Send) {
    // Перенаправляем логи из `log` в `tracing`
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
