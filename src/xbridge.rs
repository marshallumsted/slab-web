use axum::{http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// track running xpra sessions
lazy_static! {
    static ref SESSIONS: Arc<Mutex<HashMap<String, XpraSession>>> =
        Arc::new(Mutex::new(HashMap::new()));
}

// simple lazy_static without the crate
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
}

// check if xpra is available
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
    // bind to port 0 to get a free port from the OS
    std::net::TcpListener::bind("127.0.0.1:0")
        .ok()
        .map(|l| l.local_addr().unwrap().port())
        .unwrap_or(10000)
}

// GET /api/xbridge/status
pub async fn status() -> Json<BridgeStatus> {
    let version = xpra_version().unwrap_or_default();
    let sessions = SESSIONS.lock().unwrap().values().cloned().collect();
    Json(BridgeStatus {
        available: !version.is_empty(),
        version,
        sessions,
    })
}

// POST /api/xbridge/launch
pub async fn launch(Json(params): Json<LaunchParams>) -> Result<Json<LaunchResult>, StatusCode> {
    if xpra_version().is_none() {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    let port = find_free_port();
    let id = format!("xb-{}", port);

    // strip field codes from exec
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

    // start xpra with websocket binding
    let output = std::process::Command::new("xpra")
        .args([
            "start",
            "--no-daemon",
            "--no-notifications",
            "--no-mdns",
            "--no-pulseaudio",
            "--no-speaker",
            "--no-microphone",
            &format!("--bind-ws=0.0.0.0:{port}"),
            "--html=on",
            &format!("--start={cmd}"),
        ])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();

    match output {
        Ok(child) => {
            let pid = child.id();

            // get the display from xpra
            // xpra assigns one automatically, we'll find it
            let display = format!(":{}", port % 1000 + 100);

            let session = XpraSession {
                id: id.clone(),
                app_name,
                exec: cmd,
                port,
                display,
                pid,
            };

            SESSIONS.lock().unwrap().insert(id.clone(), session);

            Ok(Json(LaunchResult { id, port }))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// POST /api/xbridge/stop
#[derive(Deserialize)]
pub struct StopParams {
    pub id: String,
}

pub async fn stop(Json(params): Json<StopParams>) -> StatusCode {
    let session = SESSIONS.lock().unwrap().remove(&params.id);

    if let Some(session) = session {
        // kill the xpra session
        let _ = std::process::Command::new("kill")
            .arg(session.pid.to_string())
            .status();

        // also try xpra stop
        let _ = std::process::Command::new("xpra")
            .args(["stop", &session.display])
            .status();

        StatusCode::OK
    } else {
        StatusCode::NOT_FOUND
    }
}
