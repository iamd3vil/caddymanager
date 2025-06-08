use crate::caddy::CADDY_PID;
use axum::Router;
use tokio::process::Command;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod caddy;

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");

    println!("Shutting down...");
    println!("Stopping Caddy process...");
    let _ = Command::new("pkill").arg("caddy").output().await;
    // Clean up any temporary files
    let _ = std::fs::remove_file("/tmp/caddy.pid");
    println!("Caddy process stopped.");
}

#[tokio::main]
async fn main() {
    // Set an environment variable to work around a macOS fork safety issue
    std::env::set_var("OBJC_DISABLE_INITIALIZE_FORK_SAFETY", "YES");

    // Ensure no old instances are running and clean up files
    let _ = Command::new("pkill").arg("caddy").output().await;
    let _ = std::fs::remove_file("/tmp/caddy.pid");
    // Clear the log file on startup
    let _ = std::fs::File::create("/tmp/caddy.log");

    // Start Caddy as a child process
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    let config_path = format!("{}/Caddyfile", manifest_dir);

    let child = Command::new("caddy")
        .arg("run")
        .arg("--config")
        .arg(&config_path)
        .spawn()
        .expect("Failed to start Caddy");

    // Store the child process handle in our global static
    if let Some(pid) = child.id() {
        *CADDY_PID.lock().await = Some(pid);
    }

    // Determine static files directory
    let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
        format!("{}/static", manifest_dir)
    });

    let app = Router::new()
        .nest("/api", caddy::api_router())
        .nest_service("/", ServeDir::new(&static_dir))
        .layer(TraceLayer::new_for_http())
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
