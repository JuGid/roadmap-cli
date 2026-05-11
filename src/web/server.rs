use axum::{routing::{get, post}, Router, Extension};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

use super::assets::serve_static;
use super::routes::{api_get_phase, api_list_phases, api_report, api_update_phase_status, index, phase_detail, phases_list};
use crate::auth;

/// Run the web server (local mode, no auth)
pub async fn run_server(port: u16, open_browser: bool) {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Try to connect to DB for SaaS mode
    dotenvy::dotenv().ok();
    let pool = if let Ok(url) = std::env::var("DATABASE_URL") {
        crate::db::create_pool(&url).await.ok()
    } else {
        None
    };

    let mut app = Router::new()
        // HTML routes
        .route("/", get(index))
        .route("/phases", get(phases_list))
        .route("/phases/{id}", get(phase_detail))
        // API routes
        .route("/api/phases", get(api_list_phases))
        .route("/api/phases/{id}", get(api_get_phase))
        .route("/api/phases/{id}/status", post(api_update_phase_status))
        .route("/api/report", get(api_report))
        // Static assets
        .route("/static/{*path}", get(serve_static));

    // Add auth + API v1 + WebSocket routes if DB is available
    if let Some(pool) = pool {
        let ws_hub = crate::ws::hub::WsHub::new();
        let device_store = auth::device::DeviceStore::new();

        app = app
            .route("/api/auth/register", post(auth::routes::register))
            .route("/api/auth/login", post(auth::routes::login))
            .route("/api/auth/refresh", post(auth::routes::refresh))
            // Device flow (CLI login)
            .route("/api/auth/device", post(auth::device::request_device_code))
            .route("/api/auth/device/approve", post(auth::device::approve_device))
            .route("/api/auth/device/deny", post(auth::device::deny_device))
            .route("/api/auth/device/{device_code}", get(auth::device::poll_device))
            .nest("/api/v1", crate::api::router::api_v1_router())
            .route("/ws", get(crate::ws::handler::ws_handler))
            .layer(Extension(pool))
            .layer(Extension(ws_hub))
            .layer(Extension(device_store));
    }

    let app = app.layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let url = format!("http://localhost:{}", port);

    println!("Serveur web démarré sur {}", url);
    println!();
    println!("  Kanban:  {}/", url);
    println!("  Phases:  {}/phases", url);
    println!("  API:     {}/api/phases", url);
    println!();
    println!("Appuyez sur Ctrl+C pour arrêter");

    if open_browser {
        if let Err(e) = open::that(&url) {
            eprintln!("Impossible d'ouvrir le navigateur: {}", e);
        }
    }

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
