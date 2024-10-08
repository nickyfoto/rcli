use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Router,
};
use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::{info, warn};

#[derive(Debug)]
struct HttpServeState {
    dir: PathBuf,
}

pub async fn process_http_serve(dir: PathBuf, port: u16) -> Result<()> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    info!("Serving files from {:?} on http://{}", dir, addr);

    let state = HttpServeState { dir: dir.clone() };
    // axum router
    let router = Router::new()
        .nest_service("/tower", ServeDir::new(dir))
        .route("/*path", get(file_handler))
        .with_state(Arc::new(state));

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = std::path::Path::new(&state.dir).join(path);
    info!("Reading file {:?}", p);
    if p.exists() {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    } else {
        (
            StatusCode::NOT_FOUND,
            format!("File {} not found", p.display()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_handler() {
        let state = Arc::new(HttpServeState {
            dir: PathBuf::from("."),
        });
        let (status, content) = file_handler(State(state), Path("Cargo.toml".to_string())).await;
        assert_eq!(status, StatusCode::OK);
        assert!(content.trim().starts_with("[package]"));
    }
}
