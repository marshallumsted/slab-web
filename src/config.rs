use axum::{http::StatusCode, response::Json};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ── System config (/etc/slab/config.json) — requires root ──

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct SystemConfig {
    #[serde(default)]
    pub performance: PerformanceSettings,
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub shared_network: Vec<NetworkPlace>,
    #[serde(default)]
    pub locked: Vec<String>, // setting keys users cannot override
}

// ── User config (~/.config/slab/config.json) — per user ──

#[derive(Serialize, Deserialize, Clone)]
pub struct UserConfig {
    #[serde(default)]
    pub settings: UserSettings,
    pub places: Vec<Place>,
    pub network: Vec<NetworkPlace>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct UserSettings {
    #[serde(default)]
    pub general: GeneralSettings,
    #[serde(default)]
    pub performance: Option<PerformanceSettings>, // overrides system if not locked
    #[serde(default)]
    pub files: FilesSettings,
    #[serde(default)]
    pub terminal: TerminalSettings,
    #[serde(default)]
    pub editor: EditorSettings,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GeneralSettings {
    #[serde(default = "default_dark")]
    pub theme: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PerformanceSettings {
    #[serde(default = "default_true")]
    pub animations: bool,
    #[serde(default = "default_true")]
    pub dot_grid: bool,
    #[serde(default = "default_true")]
    pub backdrop_blur: bool,
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        Self { animations: true, dot_grid: true, backdrop_blur: true }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FilesSettings {
    #[serde(default = "default_true")]
    pub image_previews: bool,
    #[serde(default = "default_true")]
    pub video_previews: bool,
    #[serde(default)]
    pub show_hidden: bool,
    #[serde(default = "default_list")]
    pub default_view: String,
}

impl Default for FilesSettings {
    fn default() -> Self {
        Self {
            image_previews: true,
            video_previews: true,
            show_hidden: false,
            default_view: "list".into(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct TerminalSettings {
    #[serde(default = "default_14")]
    pub font_size: String,
    #[serde(default = "default_true")]
    pub bold_is_bright: bool,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct EditorSettings {
    #[serde(default = "default_14")]
    pub font_size: String,
    #[serde(default = "default_true")]
    pub word_wrap: bool,
    #[serde(default = "default_true")]
    pub line_numbers: bool,
}

fn default_true() -> bool { true }
fn default_list() -> String { "list".into() }
fn default_dark() -> String { "dark".into() }
fn default_14() -> String { "14".into() }

#[derive(Serialize, Deserialize, Clone)]
pub struct Place {
    pub name: String,
    pub path: String,
    pub builtin: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NetworkPlace {
    pub id: String,
    pub name: String,
    pub protocol: String,
    pub host: String,
    pub port: Option<u16>,
    pub path: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub pinned: bool,
}

// ── Merged config — what the frontend sees ──

#[derive(Serialize)]
pub struct MergedConfig {
    pub settings: MergedSettings,
    pub places: Vec<Place>,
    pub network: Vec<NetworkPlace>,
    pub locked: Vec<String>,
    pub is_admin: bool,
}

#[derive(Serialize)]
pub struct MergedSettings {
    pub general: GeneralSettings,
    pub performance: PerformanceSettings,
    pub files: FilesSettings,
    pub terminal: TerminalSettings,
    pub editor: EditorSettings,
}

// ── Paths ──

fn system_config_path() -> PathBuf {
    PathBuf::from("/etc/slab/config.json")
}

fn user_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".into());
    PathBuf::from(home).join(".config/slab/config.json")
}

// ── Loaders ──

fn load_system_config() -> SystemConfig {
    let path = system_config_path();
    if path.exists() {
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        SystemConfig::default()
    }
}

fn default_user_config() -> UserConfig {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home/user".into());
    UserConfig {
        settings: UserSettings::default(),
        places: vec![
            Place { name: "Home".into(), path: home.clone(), builtin: true },
            Place { name: "Desktop".into(), path: format!("{home}/Desktop"), builtin: true },
            Place { name: "Documents".into(), path: format!("{home}/Documents"), builtin: true },
            Place { name: "Downloads".into(), path: format!("{home}/Downloads"), builtin: true },
            Place { name: "Pictures".into(), path: format!("{home}/Pictures"), builtin: true },
            Place { name: "Music".into(), path: format!("{home}/Music"), builtin: true },
            Place { name: "Videos".into(), path: format!("{home}/Videos"), builtin: true },
        ],
        network: vec![],
    }
}

fn load_user_config() -> UserConfig {
    let path = user_config_path();
    if path.exists() {
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(default_user_config)
    } else {
        default_user_config()
    }
}

fn save_user_config(config: &UserConfig) -> Result<(), String> {
    let path = user_config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())
}

fn save_system_config(config: &SystemConfig) -> Result<(), String> {
    let path = system_config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())
}

fn is_admin() -> bool {
    // check if current user can write to /etc/slab/
    let test = PathBuf::from("/etc/slab/.write_test");
    let ok = std::fs::write(&test, "").is_ok();
    let _ = std::fs::remove_file(&test);
    ok
}

fn merge_configs(sys: &SystemConfig, user: &UserConfig) -> MergedConfig {
    // performance: user override unless locked
    let perf = if sys.locked.contains(&"performance".to_string()) {
        sys.performance.clone()
    } else {
        user.settings.performance.clone().unwrap_or_else(|| sys.performance.clone())
    };

    // combine network: system shared + user personal
    let mut network = sys.shared_network.clone();
    network.extend(user.network.clone());

    MergedConfig {
        settings: MergedSettings {
            general: user.settings.general.clone(),
            performance: perf,
            files: user.settings.files.clone(),
            terminal: user.settings.terminal.clone(),
            editor: user.settings.editor.clone(),
        },
        places: user.places.clone(),
        network,
        locked: sys.locked.clone(),
        is_admin: is_admin(),
    }
}

// ── API handlers ──

// GET /api/config — returns merged view
pub async fn get_config() -> Json<MergedConfig> {
    let sys = load_system_config();
    let user = load_user_config();
    Json(merge_configs(&sys, &user))
}

// POST /api/config — saves user config
// Frontend sends the full user config shape (settings, places, network)
pub async fn set_config(Json(config): Json<UserConfig>) -> StatusCode {
    match save_user_config(&config) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

// POST /api/config/system — saves system config (requires root)
pub async fn set_system_config(Json(config): Json<SystemConfig>) -> StatusCode {
    if !is_admin() {
        return StatusCode::FORBIDDEN;
    }
    match save_system_config(&config) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
