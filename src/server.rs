use axum::{
    response::{IntoResponse, Sse},
    routing::get,
    Router,
};
use axum_embed::ServeEmbed;
use futures_util::stream::{self, Stream};
use rust_embed::RustEmbed;
use serde_json::json;
use socketioxide::{extract::SocketRef, SocketIo};
use std::{
    net::SocketAddr,
    str::FromStr,
    sync::{LazyLock, OnceLock},
};
use tokio::{runtime::Runtime, sync::broadcast::Sender};
use tower_http::cors::{Any, CorsLayer};

use crate::models::packets::Packet;

const SERVER_ADDR: &str = "127.0.0.1:1305";

static SOCKET_IO: OnceLock<SocketIo> = OnceLock::new();
static RUNTIME: LazyLock<Runtime> =
    LazyLock::new(|| Runtime::new().expect("Failed to create Tokio runtime"));
static TX: LazyLock<Sender<Packet>> = LazyLock::new(|| {
    let (tx, _) = tokio::sync::broadcast::channel(100);
    tx
});

#[derive(RustEmbed, Clone)]
#[folder = "assets/"]
struct Assets;

pub fn start_server() {
    RUNTIME.block_on(async {
        let (socket_io_layer, io) = SocketIo::new_layer();
        io.ns("/", on_connect);
        SOCKET_IO.set(io).unwrap();

        let static_assets_layer = ServeEmbed::<Assets>::new();

        let app = Router::new()
            .route("/events", get(sse_handler))
            .nest_service("/static", static_assets_layer)
            .layer(socket_io_layer)
            .layer(
                CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any),
            );

        // HTTP
        axum_server::bind(SocketAddr::from_str(SERVER_ADDR).unwrap())
            .serve(app.into_make_service())
            .await
            .expect("Failed to start server");
    });
}

async fn sse_handler() -> impl IntoResponse {
    fn subscribe() -> impl Stream<Item = Result<axum::response::sse::Event, anyhow::Error>> {
        let rx = TX.subscribe();
        stream::unfold(rx, |mut rx| async move {
            match rx.recv().await {
                Ok(data) => Some((
                    Ok(axum::response::sse::Event::default()
                        .json_data(json!({
                            "type": data.name(),
                            "data": data.payload()
                        }))
                        .unwrap()),
                    rx,
                )),
                Err(_) => None,
            }
        })
    }

    Sse::new(subscribe())
}

fn on_connect(socket: SocketRef) {
    let packet = Packet::Connected {
        version: env!("TARGET_BUILD").to_string(),
    };
    socket.emit(packet.name(), &packet.payload()).ok();
}

pub fn broadcast(packet: Packet) {
    RUNTIME.spawn(async move {
        let io = SOCKET_IO.get().unwrap();
        io.broadcast()
            .emit(&packet.name(), &packet.payload())
            .await
            .unwrap();

        if packet.name() == "OnStatChange" {
            return;
        }

        let _ = TX.send(packet);
    });
}
