use axum::{
    debug_handler,
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use tokio::{fs, sync::Mutex};

// --- Global State for Caddy Process ---
lazy_static! {
    pub static ref CADDY_PID: Mutex<Option<u32>> = Mutex::new(None);
}

// --- API Router ---
pub fn api_router() -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/hosts", get(get_hosts).post(add_host))
        .route("/hosts/:name", delete(delete_host))
}

// --- Config Path ---
fn get_config_path() -> String {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    format!("{}/Caddyfile", manifest_dir)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Host {
    pub name: String, // The domain name for routing
    pub ip: String,
    pub port: u16,
    pub scheme: String,
}

#[derive(Serialize)]
pub struct HealthStatus {
    status: &'static str,
    caddy_running: bool,
}

pub async fn health_check() -> Json<HealthStatus> {
    let caddy_running = CADDY_PID.lock().await.is_some();
    Json(HealthStatus {
        status: "ok",
        caddy_running,
    })
}

pub async fn get_hosts() -> Result<Json<Vec<Host>>, AppError> {
    let config_path = get_config_path();
    let config = fs::read_to_string(&config_path).await?;
    let hosts = parse_config(&config);
    Ok(Json(hosts))
}

#[debug_handler]
pub async fn add_host(Json(host): Json<Host>) -> Result<StatusCode, AppError> {
    println!("Starting add_host for: {:?}", host);

    let config_path = get_config_path();
    println!("Config path: {}", config_path);

    let mut config = fs::read_to_string(&config_path).await.map_err(|e| {
        println!("Failed to read config file: {}", e);
        e
    })?;

    let mut hosts = parse_config(&config);
    println!("Current hosts count: {}", hosts.len());

    if hosts.iter().any(|h| h.name == host.name) {
        println!("Host with name '{}' already exists", host.name);
        return Err(AppError(
            StatusCode::CONFLICT,
            format!("Host with name '{}' already exists", host.name),
        ));
    }

    hosts.push(host.clone());
    println!("Added host, new count: {}", hosts.len());

    config = generate_config(&config, &hosts).map_err(|e| {
        println!("Failed to generate config: {:?}", e);
        e
    })?;

    fs::write(&config_path, &config).await.map_err(|e| {
        println!("Failed to write config file: {}", e);
        e
    })?;
    println!("Successfully wrote config file");

    reload_caddy().await.map_err(|e| {
        println!("Failed to reload caddy: {:?}", e);
        e
    })?;

    println!("Successfully completed add_host for: {}", host.name);
    Ok(StatusCode::CREATED)
}

#[debug_handler]
pub async fn delete_host(Path(name): Path<String>) -> Result<StatusCode, AppError> {
    let config_path = get_config_path();
    let mut config = fs::read_to_string(&config_path).await?;
    let mut hosts = parse_config(&config);
    hosts.retain(|h| h.name != name);
    config = generate_config(&config, &hosts)?;
    fs::write(&config_path, config).await?;
    reload_caddy().await?;
    Ok(StatusCode::NO_CONTENT)
}

fn parse_config(config: &str) -> Vec<Host> {
    let mut hosts = Vec::new();

    // Parse Caddy configuration format: domain.com { reverse_proxy https://target:port { ... } }
    let re_host = Regex::new(r"(?s)([a-zA-Z0-9.-]+)\s*\{[^}]*reverse_proxy\s+(https?://)?([^:\s/]+):(\d+)").unwrap();

    for cap in re_host.captures_iter(config) {
        let name = cap[1].to_string();
        let scheme_prefix = cap.get(2).map_or("", |m| m.as_str());
        let ip = cap[3].to_string();
        let port = cap[4].parse().unwrap_or(80);
        
        // Get the full block content to check for TLS
        let full_match = cap.get(0).unwrap().as_str();

        // Determine scheme based on URL prefix, port, or TLS configuration
        let scheme = if !scheme_prefix.is_empty() {
            scheme_prefix.trim_end_matches("://").to_string()
        } else if port == 443 || full_match.contains("tls") {
            "https".to_string()
        } else {
            "http".to_string()
        };

        hosts.push(Host {
            name,
            ip,
            port,
            scheme,
        });
    }
    hosts
}

fn generate_config(current_config: &str, hosts: &[Host]) -> Result<String, AppError> {
    let mut host_blocks = String::new();

    for host in hosts {
        let upstream_url = format!("{}://{}:{}", host.scheme, host.ip, host.port);
        
        let host_block = format!(
            "{} {{\n    reverse_proxy {} {{\n        header_up Host {{upstream_hostport}}\n    }}\n}}\n",
            host.name, upstream_url
        );
        host_blocks.push_str(&host_block);
    }

    let re_dynamic_block =
        Regex::new(r"(?s)# --- START DYNAMIC CONFIG ---(.*)# --- END DYNAMIC CONFIG ---").unwrap();
    let new_config = re_dynamic_block.replace(
        current_config,
        &format!(
            "# --- START DYNAMIC CONFIG ---\n{}\n# --- END DYNAMIC CONFIG ---",
            host_blocks.trim()
        ),
    );

    Ok(new_config.to_string())
}

async fn reload_caddy() -> Result<(), AppError> {
    let config_path = get_config_path();

    // For Caddy, we use the reload command which gracefully reloads the configuration
    println!(
        "Gracefully reloading Caddy configuration from: {}",
        config_path
    );

    let output = tokio::time::timeout(
        std::time::Duration::from_secs(10),
        tokio::process::Command::new("caddy")
            .arg("reload")
            .arg("--config")
            .arg(&config_path)
            .output(),
    )
    .await;

    match output {
        Ok(Ok(result)) if result.status.success() => {
            println!("Caddy configuration reloaded successfully");
            let stdout = String::from_utf8_lossy(&result.stdout);
            if !stdout.is_empty() {
                println!("Caddy reload stdout: {}", stdout);
            }
            Ok(())
        }
        Ok(Ok(result)) => {
            let error_msg = String::from_utf8_lossy(&result.stderr);
            let stdout_msg = String::from_utf8_lossy(&result.stdout);
            println!(
                "Caddy reload failed with exit code: {:?}",
                result.status.code()
            );
            println!("Caddy stderr: {}", error_msg);
            println!("Caddy stdout: {}", stdout_msg);
            Err(AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!(
                    "Failed to reload Caddy (exit code: {:?}): {}",
                    result.status.code(),
                    error_msg
                ),
            ))
        }
        Ok(Err(e)) => {
            println!("Failed to execute caddy command: {}", e);
            Err(AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to execute caddy reload command: {}", e),
            ))
        }
        Err(_timeout_err) => {
            println!("Caddy reload command timed out after 10 seconds");
            Err(AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Caddy reload command timed out after 10 seconds".to_string(),
            ))
        }
    }
}

// --- Error Handling ---
#[derive(Debug)]
pub struct AppError(StatusCode, String);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.0, self.1).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: std::error::Error,
{
    fn from(err: E) -> Self {
        AppError(StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
    }
}
