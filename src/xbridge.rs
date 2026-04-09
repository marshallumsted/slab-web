use axum::{http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

lazy_static! {
    static ref SESSIONS: Arc<Mutex<HashMap<String, XpraSession>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

mod lazy_static {
    macro_rules! lazy_static {
        (static ref $name:ident : $ty:ty = $init:expr ;) => {
            static $name: std::sync::LazyLock<$ty> = std::sync::LazyLock::new(|| $init);
        };
    }
    pub(crate) use lazy_static;
}
use lazy_static::lazy_static;

#[derive(Serialize, Clone)]
pub struct XpraSession {
    pub id: String,
    pub app_name: String,
    pub exec: String,
    pub port: u16,
    pub display: String,
    pub pid: u32,
}

#[derive(Serialize)]
pub struct BridgeStatus {
    pub available: bool,
    pub version: String,
    pub sessions: Vec<XpraSession>,
}

#[derive(Deserialize)]
pub struct LaunchParams {
    pub exec: String,
    pub name: Option<String>,
}

#[derive(Serialize)]
pub struct LaunchResult {
    pub id: String,
    pub port: u16,
    pub url: String,
}

fn xpra_version() -> Option<String> {
    std::process::Command::new("xpra")
        .arg("--version")
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
}

fn find_free_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0")
        .ok()
        .map(|l| l.local_addr().unwrap().port())
        .unwrap_or(10000)
}

fn find_free_display() -> u32 {
    // find a display number not in use
    for d in 100..200 {
        let lock = format!("/tmp/.X{d}-lock");
        if !std::path::Path::new(&lock).exists() {
            return d;
        }
    }
    100
}

fn xpra_html5_available() -> bool {
    // check common paths for the HTML5 client
    std::path::Path::new("/usr/share/xpra/www/index.html").exists()
        || std::path::Path::new("/usr/share/xpra-html5/index.html").exists()
}

pub async fn status() -> Json<BridgeStatus> {
    let version = xpra_version().unwrap_or_default();
    let sessions = SESSIONS.lock().unwrap().values().cloned().collect();
    Json(BridgeStatus {
        available: !version.is_empty() && xpra_html5_available(),
        version,
        sessions,
    })
}

pub async fn launch(Json(params): Json<LaunchParams>) -> Result<Json<LaunchResult>, StatusCode> {
    if xpra_version().is_none() {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    let port = find_free_port();
    let display_num = find_free_display();
    let display = format!(":{display_num}");
    let id = format!("xb-{port}");

    let cmd = params
        .exec
        .split_whitespace()
        .filter(|s| !s.starts_with('%'))
        .collect::<Vec<_>>()
        .join(" ");

    let app_name = params.name.unwrap_or_else(|| {
        cmd.split_whitespace()
            .next()
            .unwrap_or("app")
            .rsplit('/')
            .next()
            .unwrap_or("app")
            .to_string()
    });

    // start xpra as a daemon (it backgrounds itself)
    let result = std::process::Command::new("xpra")
        .args([
            "start",
            &display,
            &format!("--bind-ws=0.0.0.0:{port}"),
            "--html=on",
            "--no-notifications",
            "--no-mdns",
            "--no-pulseaudio",
            "--no-speaker",
            "--no-microphone",
            "--sharing=yes",
            &format!("--start={cmd}"),
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::piped())
        .status();

    match result {
        Ok(status) if status.success() => {
            // get the xpra server pid
            let pid = std::process::Command::new("xpra")
                .args(["info", &display])
                .output()
                .ok()
                .and_then(|o| {
                    let text = String::from_utf8_lossy(&o.stdout);
                    text.lines()
                        .find(|l| l.starts_with("server.pid="))
                        .and_then(|l| l.split('=').nth(1))
                        .and_then(|v| v.trim().parse::<u32>().ok())
                })
                .unwrap_or(0);

            let session = XpraSession {
                id: id.clone(),
                app_name,
                exec: cmd,
                port,
                display: display.clone(),
                pid,
            };

            SESSIONS.lock().unwrap().insert(id.clone(), session);

            let url = format!("/api/xbridge/proxy/{port}/index.html");

            Ok(Json(LaunchResult { id, port, url }))
        }
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// proxy requests to xpra's HTML5 client to avoid cross-origin issues
pub async fn proxy(
    axum::extract::Path((port, path)): axum::extract::Path<(u16, String)>,
) -> impl axum::response::IntoResponse {
    let url = format!("http://127.0.0.1:{port}/{path}");
    match reqwest_blocking(&url) {
        Some((content_type, body)) => {
            let mut headers = axum::http::HeaderMap::new();
            headers.insert(
                axum::http::header::CONTENT_TYPE,
                content_type.parse().unwrap_or_else(|_| "text/html".parse().unwrap()),
            );
            (StatusCode::OK, headers, body)
        }
        None => (
            StatusCode::BAD_GATEWAY,
            axum::http::HeaderMap::new(),
            Vec::new(),
        ),
    }
}

fn reqwest_blocking(url: &str) -> Option<(String, Vec<u8>)> {
    // simple blocking HTTP client using std
    use std::io::Read;
    let url_parsed: std::collections::HashMap<&str, &str> = {
        let without_proto = url.strip_prefix("http://")?;
        let (host_port, path) = without_proto.split_once('/')?;
        let mut m = std::collections::HashMap::new();
        m.insert("host", host_port);
        m.insert("path", path);
        m
    };

    let host = url_parsed.get("host")?;
    let path = url_parsed.get("path")?;

    let mut stream = std::net::TcpStream::connect(host).ok()?;
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .ok()?;

    use std::io::Write;
    write!(
        stream,
        "GET /{path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n\r\n"
    )
    .ok()?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response).ok()?;

    let response_str = String::from_utf8_lossy(&response);
    let header_end = response_str.find("\r\n\r\n")? + 4;
    let headers = &response_str[..header_end];
    let body = &response[header_end..];

    let content_type = headers
        .lines()
        .find(|l| l.to_lowercase().starts_with("content-type:"))
        .map(|l| l.split_once(':').unwrap().1.trim().to_string())
        .unwrap_or_else(|| "application/octet-stream".to_string());

    Some((content_type, body.to_vec()))
}

#[derive(Deserialize)]
pub struct StopParams {
    pub id: String,
}

pub async fn stop(Json(params): Json<StopParams>) -> StatusCode {
    let session = SESSIONS.lock().unwrap().remove(&params.id);

    if let Some(session) = session {
        // stop the xpra session cleanly
        let _ = std::process::Command::new("xpra")
            .args(["stop", &session.display])
            .status();

        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
