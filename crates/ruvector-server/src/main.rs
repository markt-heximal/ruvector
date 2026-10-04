use clap::Parser;
use ruvector_server::{Config, RuvectorServer};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "ruvector-server")]
#[command(about = "RuVector REST API server")]
#[command(version)]
struct Args {
    /// Host address to bind to
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 6333)]
    port: u16,

    /// Disable CORS
    #[arg(long)]
    no_cors: bool,

    /// Disable compression
    #[arg(long)]
    no_compression: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let args = Args::parse();

    let config = Config {
        host: args.host,
        port: args.port,
        enable_cors: !args.no_cors,
        enable_compression: !args.no_compression,
    };

    println!("Starting RuVector server on {}:{}", config.host, config.port);
    println!("API endpoints:");
    println!("  GET  /health                          - Health check");
    println!("  GET  /ready                           - Readiness check");
    println!("  POST /collections                     - Create collection");
    println!("  GET  /collections                     - List collections");
    println!("  GET  /collections/:name               - Get collection info");
    println!("  DEL  /collections/:name               - Delete collection");
    println!("  PUT  /collections/:name/points        - Upsert points");
    println!("  POST /collections/:name/points/search - Search points");
    println!("  GET  /collections/:name/points/:id    - Get point by ID");

    RuvectorServer::with_config(config).start().await?;

    Ok(())
}
