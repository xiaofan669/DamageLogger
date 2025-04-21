use anyhow::Result;
use axum::response::sse::Event;
use futures_util::stream::{self, Stream};
use std::sync::{Arc, LazyLock, Mutex};
use tokio::sync::broadcast::{self, Sender};

use crate::models::packets::Packet;

#[derive(Clone)]
pub struct SocketManager {
    tx: Sender<Vec<u8>>,
}

impl SocketManager {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        SocketManager { tx }
    }

    pub fn broadcast_packet(&self, packet: Packet) -> Result<()> {
        self.tx.send(packet.to_bytes())?;
        Ok(())
    }

    pub fn subscribe(&self) -> impl Stream<Item = Result<Event, anyhow::Error>> {
        let rx = self.tx.subscribe();
        stream::unfold(rx, |mut rx| async move {
            match rx.recv().await {
                Ok(data) => Some((
                    Ok(Event::default().data(String::from_utf8_lossy(&data).to_string())),
                    rx,
                )),
                Err(_) => None,
            }
        })
    }
}

static SOCKET_MANAGER: LazyLock<Arc<Mutex<SocketManager>>> =
    LazyLock::new(|| Arc::new(Mutex::new(SocketManager::new())));

pub fn get_instance() -> Arc<Mutex<SocketManager>> {
    SOCKET_MANAGER.clone()
}
