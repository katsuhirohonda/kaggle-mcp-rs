#[cfg(test)]
pub fn init_test_logger() {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
    
    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "kaggle_mcp_rs=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().with_test_writer())
        .try_init();
}