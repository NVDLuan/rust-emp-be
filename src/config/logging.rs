use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, fmt};
use tracing_appender::{rolling, non_blocking};

pub fn init_logging() {
    // Create a rolling file appender that creates a new file daily
    let file_appender = rolling::daily("logs", "emp-be.log");
    let (non_blocking_appender, _guard) = non_blocking(file_appender);
    
    // Create a file layer for file logging
    let file_layer = fmt::layer()
        .with_writer(non_blocking_appender)
        .with_ansi(false)
        .with_target(false)
        .with_thread_ids(true)
        .with_thread_names(true);

    // Create a console layer for console logging
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_ansi(true)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    // Set up environment filter
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "emp_be=info,tower_http=debug,actix_web=info".into());

    // Initialize the subscriber with both console and file logging
    tracing_subscriber::registry()
        .with(env_filter)
        .with(console_layer)
        .with(file_layer)
        .init();

    // Log startup message
    tracing::info!("🚀 Logging system initialized successfully");
    tracing::info!("📁 Log files will be written to: logs/emp-be.log");
}
