use tracing::subscriber::set_global_default;
use tracing::Subscriber;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

// Register subscriber
// Sink refers to where logs should be written to
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // Skip optional fields
    let skipped_fields = vec![""];

    let formatting_layer = BunyanFormattingLayer::new(name, sink)
        .skip_fields(skipped_fields.into_iter())
        .unwrap();

    Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer)
}

// Register subscriber as global default
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().unwrap();
    set_global_default(subscriber).unwrap();
}
