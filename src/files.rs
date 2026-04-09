use axum::{
    extract::Query,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct ListParams {
    path: Option<String>,
}

#[derive(Serialize)]
pub struct DirEntry {
    name: String,
    is_dir: bool,
    size: u64,
    modified: u64,
}

#[derive(Serialize)]
pub struct DirListing {
    path: String,
    parent: Option<String>,
    entries: Vec<DirEntry>,
}

pub async fn list_dir(Query(params): Query<ListParams>) -> Json<DirListing> {
    let path = params
        .path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"));

    // resolve to canonical, fall back to /
    let path = path.canonicalize().unwrap_or_else(|_| PathBuf::from("/"));

    let parent = path.parent().map(|p| p.to_string_lossy().into_owned());

    let mut entries = Vec::new();

    if let Ok(read_dir) = std::fs::read_dir(&path) {
        for entry in read_dir.flatten() {
            let meta = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };

            let modified = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);

            entries.push(DirEntry {
                name: entry.file_name().to_string_lossy().into_owned(),
                is_dir: meta.is_dir(),
                size: meta.len(),
                modified,
            });
        }
    }

    // dirs first, then alphabetical
    entries.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then(a.name.to_lowercase().cmp(&b.name.to_lowercase())));

    Json(DirListing {
        path: path.to_string_lossy().into_owned(),
        parent,
        entries,
    })
}

#[derive(Deserialize)]
pub struct RawParams {
    path: String,
}

pub async fn serve_raw(Query(params): Query<RawParams>) -> impl IntoResponse {
    let path = PathBuf::from(&params.path);
    let path = match path.canonicalize() {
        Ok(p) => p,
        Err(_) => return (StatusCode::NOT_FOUND, HeaderMap::new(), Vec::new()),
    };

    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(_) => return (StatusCode::NOT_FOUND, HeaderMap::new(), Vec::new()),
    };

    let mime = match path.extension().and_then(|e| e.to_str()) {
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("bmp") => "image/bmp",
        Some("ico") => "image/x-icon",
        Some("avif") => "image/avif",
        _ => "application/octet-stream",
    };

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, mime.parse().unwrap());
    headers.insert(header::CACHE_CONTROL, "public, max-age=60".parse().unwrap());

    (StatusCode::OK, headers, bytes)
}
