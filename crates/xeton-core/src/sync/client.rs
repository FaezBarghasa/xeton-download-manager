use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, warn};

#[derive(Clone)]
pub struct PeerDevice {
    pub id: String,
    pub name: String,
    pub ip: String,
    pub port: u16,
}

pub struct SyncService {
    device_name: String,
    devices: Arc<Mutex<Vec<PeerDevice>>>,
}

impl SyncService {
    pub fn new(device_name: String) -> Arc<Self> {
        Arc::new(Self {
            device_name,
            devices: Arc::new(Mutex::new(Vec::new())),
        })
    }

    pub async fn start_discovery(&self) {
        info!("Starting network discovery for device: {}", self.device_name);
    }

    pub async fn stop_discovery(&self) {
        info!("Stopping network discovery");
    }

    pub async fn discovered_devices(&self) -> Vec<PeerDevice> {
        let lock = self.devices.lock().await;
        lock.clone()
    }

    pub async fn push_link(&self, peer_id: String, url: String) {
        let target_peer = {
            let lock = self.devices.lock().await;
            lock.iter().find(|d| d.id == peer_id).cloned()
        };

        if let Some(peer) = target_peer {
            let client = reqwest::Client::new();
            let res = client
                .post(format!("http://{}:{}/api/push-link", peer.ip, peer.port))
                .json(&serde_json::json!({
                    "url": url,
                    "peer_id": self.device_name
                }))
                .send()
                .await;
            if let Err(e) = res {
                warn!("Failed to push link to {}: {}", peer.name, e);
            }
        }
    }

    pub async fn pair(&self, peer_id: String, pin: String) {
        info!("Pairing with peer {} using verification code", peer_id);
    }
}
