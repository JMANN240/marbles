use std::{fmt::Debug, fs::Permissions, os::unix::fs::PermissionsExt, path::PathBuf};

use axum::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    routing::{get, post},
    serve::Listener,
};
use clap::{Args, Parser};
use dotenvy::dotenv;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::SqlitePool;
use tokio::net::{TcpListener, UnixListener};
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::api::{marbles::get_marbles, register::post_register, token::post_token};

pub mod api;
pub mod util;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    listen: Listen,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Listen {
    #[arg(short, long, group = "listen")]
    port: Option<u16>,

    #[arg(short, long, group = "listen")]
    uds: Option<PathBuf>,
}

#[derive(Clone)]
pub struct AppState {
    key: Hmac<Sha256>,
    pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    dotenv().unwrap();

    if let Some(port) = cli.listen.port {
        serve_with_listener(TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap()).await;
    } else if let Some(path) = cli.listen.uds {
        let _ = tokio::fs::remove_file(&path).await;
        tokio::fs::create_dir_all(path.parent().unwrap())
            .await
            .unwrap();

        let listener = UnixListener::bind(path.clone()).unwrap();

        tokio::fs::set_permissions(path, Permissions::from_mode(0o775))
            .await
            .unwrap();

        serve_with_listener(listener).await;
    }
}

async fn serve_with_listener<L>(listener: L)
where
    L: Listener,
    L::Addr: Debug,
{
    let pool = SqlitePool::connect(
        &std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set"),
    )
    .await
    .unwrap();

    let state = AppState {
        key: Hmac::new_from_slice(
            std::env::var("KEY")
                .expect("KEY environment variable not set")
                .as_bytes(),
        )
        .unwrap(),
        pool,
    };

    let app = axum::Router::new()
        .route("/marbles", get(get_marbles))
        .route("/register", post(post_register))
        .route("/token", post(post_token))
        .nest_service("/images", ServeDir::new("ball_images"))
        .layer(CorsLayer::permissive().allow_headers([AUTHORIZATION, CONTENT_TYPE]))
        .with_state(state);

    axum::serve(listener, app).await.unwrap();
}
