//! WebSocket connection handler

use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Extension, Query},
    response::IntoResponse,
};
use serde::Deserialize;
use uuid::Uuid;

use super::hub::WsHub;
use crate::auth::jwt::{validate_token, JwtConfig};

#[derive(Deserialize)]
pub struct WsParams {
    pub token: String,
    pub project_id: String,
}

/// GET /ws?token=xxx&project_id=xxx
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(hub): Extension<WsHub>,
    Query(params): Query<WsParams>,
) -> impl IntoResponse {
    // Validate JWT
    let config = JwtConfig::default();
    let claims = match validate_token(&params.token, &config) {
        Ok(c) => c,
        Err(_) => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };

    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };

    let project_id = match Uuid::parse_str(&params.project_id) {
        Ok(id) => id,
        Err(_) => return axum::http::StatusCode::BAD_REQUEST.into_response(),
    };

    let user_name = claims.email.clone();

    ws.on_upgrade(move |socket| handle_socket(socket, hub, project_id, user_id, user_name))
        .into_response()
}

async fn handle_socket(
    mut socket: WebSocket,
    hub: WsHub,
    project_id: Uuid,
    user_id: Uuid,
    user_name: String,
) {
    // Subscribe to project channel
    let mut rx = hub.subscribe(project_id).await;

    // Register presence
    hub.join(project_id, user_id, user_name).await;

    // Send current presence to new connection
    let presence = hub.get_presence(project_id).await;
    let presence_msg = serde_json::json!({
        "event": "presence",
        "project_id": project_id.to_string(),
        "data": { "users": presence }
    });
    let _ = socket.send(Message::Text(serde_json::to_string(&presence_msg).unwrap().into())).await;

    // Forward broadcast messages to this socket
    loop {
        tokio::select! {
            // Receive from broadcast channel → send to client
            Ok(msg) = rx.recv() => {
                let json = serde_json::to_string(&msg).unwrap();
                if socket.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
            // Receive from client (ping/pong, or future: client events)
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Close(_))) | None => break,
                    Some(Ok(Message::Ping(data))) => {
                        let _ = socket.send(Message::Pong(data)).await;
                    }
                    _ => {}
                }
            }
        }
    }

    // Cleanup
    hub.leave(project_id, user_id).await;
}
