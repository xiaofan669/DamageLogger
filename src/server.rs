use anyhow::{Context, Result};
use axum::{
    response::{sse::Sse, Html, IntoResponse},
    routing::get,
    Router,
};
use socket_manager::get_instance;
use std::net::SocketAddr;
use tokio::{runtime::Runtime, time::Duration};
use tower_http::cors::{Any, CorsLayer};

use crate::models::packets::Packet;

mod socket_manager;

const SERVER_ADDR: &str = "127.0.0.1:21500";

pub fn start_server() -> Result<()> {
    let rt = Runtime::new().context("Failed to create Tokio runtime")?;

    rt.block_on(async {
        let app = Router::new()
            .route("/", get(root_handler))
            .route("/events", get(sse_handler))
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            );

        let addr: SocketAddr = SERVER_ADDR.parse().expect("Invalid server address");

        log::info!("Server running on http://{}", addr);
        if let Err(e) = axum_server::bind(addr)
            .serve(app.into_make_service())
            .await
            .context("Failed to start server")
        {
            log::error!("Server error: {:?}", e);
        }
    });

    Ok(())
}

async fn sse_handler() -> impl IntoResponse {
    let socket_manager = get_instance();
    let socket_manager = socket_manager.lock().unwrap().clone();
    Sse::new(socket_manager.subscribe()).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("ping"),
    )
}

async fn root_handler() -> impl IntoResponse {
    Html(include_str!("./server/index.html"))
}

pub fn broadcast(packet: Packet) {
    let socket_manager = get_instance();
    let _ = socket_manager.lock().unwrap().broadcast_packet(packet);
}
