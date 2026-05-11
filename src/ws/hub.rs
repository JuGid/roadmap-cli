//! WebSocket hub - manages channels per project and broadcasts

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WsMessage {
    pub event: String,           // "task_updated", "phase_updated", "user_joined", etc.
    pub project_id: String,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub data: serde_json::Value,
}

#[derive(Clone)]
pub struct WsHub {
    channels: Arc<RwLock<HashMap<Uuid, broadcast::Sender<WsMessage>>>>,
    presence: Arc<RwLock<HashMap<Uuid, Vec<PresenceEntry>>>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PresenceEntry {
    pub user_id: Uuid,
    pub user_name: String,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

impl WsHub {
    pub fn new() -> Self {
        WsHub {
            channels: Arc::new(RwLock::new(HashMap::new())),
            presence: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Get or create a broadcast channel for a project
    pub async fn subscribe(&self, project_id: Uuid) -> broadcast::Receiver<WsMessage> {
        let mut channels = self.channels.write().await;
        let sender = channels.entry(project_id).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        sender.subscribe()
    }

    /// Broadcast a message to all subscribers of a project
    pub async fn broadcast(&self, project_id: Uuid, message: WsMessage) {
        let channels = self.channels.read().await;
        if let Some(sender) = channels.get(&project_id) {
            let _ = sender.send(message);
        }
    }

    /// Add user presence
    pub async fn join(&self, project_id: Uuid, user_id: Uuid, user_name: String) {
        let mut presence = self.presence.write().await;
        let entries = presence.entry(project_id).or_default();

        // Remove existing entry for this user (reconnect)
        entries.retain(|e| e.user_id != user_id);

        entries.push(PresenceEntry {
            user_id,
            user_name: user_name.clone(),
            connected_at: chrono::Utc::now(),
        });

        // Broadcast join event
        drop(presence);
        self.broadcast(project_id, WsMessage {
            event: "user_joined".to_string(),
            project_id: project_id.to_string(),
            user_id: Some(user_id.to_string()),
            user_name: Some(user_name),
            data: serde_json::json!({}),
        }).await;
    }

    /// Remove user presence
    pub async fn leave(&self, project_id: Uuid, user_id: Uuid) {
        let mut presence = self.presence.write().await;
        if let Some(entries) = presence.get_mut(&project_id) {
            let user_name = entries.iter().find(|e| e.user_id == user_id).map(|e| e.user_name.clone());
            entries.retain(|e| e.user_id != user_id);

            drop(presence);
            if let Some(name) = user_name {
                self.broadcast(project_id, WsMessage {
                    event: "user_left".to_string(),
                    project_id: project_id.to_string(),
                    user_id: Some(user_id.to_string()),
                    user_name: Some(name),
                    data: serde_json::json!({}),
                }).await;
            }
        }
    }

    /// Get online users for a project
    pub async fn get_presence(&self, project_id: Uuid) -> Vec<PresenceEntry> {
        let presence = self.presence.read().await;
        presence.get(&project_id).cloned().unwrap_or_default()
    }
}
