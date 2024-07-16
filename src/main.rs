use axum::{serve, Router};
use clap::Parser;
use endpoints::endpoints::BaseRouter;
use tokio::net::TcpListener;

mod endpoints;
mod templates;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 'o', long, default_value = "127.0.0.1")]
    host: String,
    #[arg(short, long, default_value = "42069")]
    port: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let host = args.host;
    let port = args.port;

    let full_url = format!("{host}:{port}");

    let app = Router::new().nest("/", BaseRouter::new_router());

    println!("==========================");
    println!("Listening on: {full_url}");
    println!("==========================");

    let listener = TcpListener::bind(full_url).await.unwrap();

    serve(listener, app).await.unwrap();
}
