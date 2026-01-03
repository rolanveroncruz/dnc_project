#[allow(unused_imports, dead_code)]
mod db;
mod entities;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use dotenvy::dotenv;
use tracing_subscriber::{EnvFilter, };
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use dnc_backend::{build_app, AppState};
use db::check_db;
use opentelemetry::{global, KeyValue,trace::TracerProvider};
use opentelemetry_otlp::{SpanExporter, WithExportConfig};
use opentelemetry_sdk::{trace as sdktrace, Resource};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::{ BatchSpanProcessor};
use migration::{Migrator, MigratorTrait};

fn init_tracer() -> sdktrace::Tracer {
    // 1. Configure the OLTP Exporter
    let exporter=SpanExporter::builder()
        .with_tonic()
        .with_endpoint("http://localhost:4317")
        .build()
        .expect("Failed to create OTLP exporter");

    //2. Build the Resource
    let resource= Resource::builder()
        .with_attributes( vec![
            KeyValue::new("service.name", "dnc_backend"),
            KeyValue::new("service.version", "0.1.0"),
        ])
        .build();

    //3. Set up the Processor
    let processor = BatchSpanProcessor::builder(exporter)
        .build();

    //4. Provider
    let provider = sdktrace::SdkTracerProvider::builder()
        .with_sampler(sdktrace::Sampler::AlwaysOn)
        .with_span_processor(processor)
        .with_resource(resource)
        .build();

    global::set_tracer_provider(provider.clone());
    global::set_text_map_propagator(TraceContextPropagator::new());
    provider.tracer("dnc_backend")

}

use tracing_subscriber::layer::Layer;


#[tokio::main]
async fn main()-> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Load environment variables from .env file

    // 1. Initialize the Tracer and get the telemetry layer
    let tracer=init_tracer();

    //2. Set up the File Appender
    let file_appender = tracing_appender::rolling::daily("logs/", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 3. Define the Layers
    let filter =EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,axum=debug,tower_http=debug,h2=off,hyper=off,tower=off"));

    let telemetry_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(tracing_subscriber::filter::LevelFilter::TRACE);

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_filter(filter.clone());

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_writer  (std::io::stdout)
        .with_ansi(true)
        .with_filter(filter);

    //4. ONE SINGLE INITIALIZATION
    tracing_subscriber::registry()
        .with(telemetry_layer)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    tracing::info!("Tracing initialized. Console, File, and OpenTelemetry are active");


    let database_url=std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Connecting to database: {}", database_url);
    let connection=sea_orm::Database::connect(&database_url).await?;
    Migrator::up(&connection, None)
        .await
        .expect("Failed to run migrations!!");

    let port=std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");



    let the_state= AppState::new().await;
    let db = &the_state.db;
    check_db(&db).await;
    let app=build_app(the_state);
    let addr= SocketAddr::from(([0,0,0,0], port));
    let listener=TcpListener::bind(addr)
        .await
        .expect("Failed to bind");

    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app)
        .await
        .unwrap();
    Ok(())
}