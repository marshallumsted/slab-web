# slab compositor

Native Wayland compositor for slab. Renders the desktop directly on hardware — no browser, no X11.

## Architecture

```
┌─────────────────────────────────────────────┐
│                  slab                        │
│                                              │
│  ┌──────────────┐    ┌───────────────────┐  │
│  │  Compositor   │    │   Web Server      │  │
│  │  (Smithay)    │    │   (axum)          │  │
│  │               │    │                   │  │
│  │  Native GPU   │    │  Remote browser   │  │
│  │  rendering    │    │  access           │  │
│  │               │    │                   │  │
│  │  Wayland      │    │  Same apps,       │  │
│  │  clients      │    │  same manifests   │  │
│  └──────┬───────┘    └──────┬────────────┘  │
│         │                   │                │
│  ┌──────┴───────────────────┴────────────┐  │
│  │          Shared App System             │  │
│  │   frontend/apps/*/manifest.json        │  │
│  │   App registration, capabilities,      │  │
│  │   data layer, settings declarations    │  │
│  └────────────────────────────────────────┘  │
│                                              │
│  ┌────────────────────────────────────────┐  │
│  │          Shared Backend                │  │
│  │   /api/files, /api/sysmon, /api/config │  │
│  │   /api/terminal, /api/shell/apps       │  │
│  └────────────────────────────────────────┘  │
└─────────────────────────────────────────────┘
```

## Two Separate Products

### slab (Native DE)
- Pure Rust desktop environment — all apps, all UI, all rendering in Rust
- Smithay-based Wayland compositor
- GPU-accelerated via wgpu
- Native Wayland apps (Firefox, VS Code, etc.) run as first-class windows
- Slab apps (terminal, files, editor, notes, sysmon) rebuilt as native Rust apps
- Multi-monitor support (Pi 5 has dual HDMI)
- Direct input handling (keyboard, mouse, touch)
- Boots straight from TTY — no browser, no WebView, no HTML
- This is the real desktop. Not a wrapper around a web page.

### slab-web (Remote Access Module)
- Separate downloadable binary/module
- The existing web server (axum + HTML/CSS/JS)
- Access the desktop from any browser on any device
- Runs alongside the native DE for remote access — like phone convergence
- Or runs standalone on headless servers/VMs (the current use case)
- Not required — the native DE works without it

## Why Smithay

- Pure Rust — no C bindings, no unsafe wrappers
- Built for exactly this use case — custom Wayland compositors
- Handles the hard parts: DRM/KMS output, libinput, EGL/Vulkan, Wayland protocol
- Active project, used by other Rust compositors (cosmic-comp, etc.)
- Same language as the rest of slab — one codebase, one build

## Rendering

Pure native. No WebView, no browser engine, no HTML.

- Shell chrome (tile grid, taskbar, menu bar) rendered directly via wgpu
- Slab apps (terminal, files, editor, etc.) are Rust crates rendered natively
- Native Wayland apps (Firefox, Steam, etc.) render through the compositor as normal
- UI toolkit: Iced (pure Rust, wgpu-based) or custom renderer using wgpu + text shaping
- Design tokens (colors, fonts, spacing) expressed as Rust constants — same values as the CSS variables but compiled in

## What the Compositor Owns

- **Output management** — DRM/KMS, monitor detection, resolution, multi-head
- **Input** — libinput for keyboard, mouse, touchpad, touchscreen
- **Window management** — Wayland client windows (xdg_toplevel), positioning, focus, stacking
- **Tiling engine** — the Hyprland/iPadOS hybrid tiling from the design doc
- **Shell rendering** — tile grid, taskbar, menu bar drawn on screen
- **Session** — TTY login, user switching, lock screen

## What's Shared with slab-web

- **Config system** — same `~/.config/slab/config.json`
- **Workspace definitions** — same workspace JSON files
- **Backend logic** — file operations, sysmon, config, terminal PTY (shared Rust crates)
- **Design language** — same colors, fonts, spacing, brutalist principles
- **Interaction model** — same tile grid, workspaces, adaptive tiles, quick spawn concepts

What's NOT shared: the rendering, the UI code, the app implementations. Native apps are Rust. Web apps are HTML/CSS/JS. Two separate codebases implementing the same design.

## Dependencies

```toml
[dependencies]
smithay = { version = "0.3", features = ["backend_drm", "backend_libinput", "wayland_frontend"] }
wgpu = "24"          # GPU rendering
calloop = "0.14"     # event loop (smithay uses this)
```

## Boot Sequence

1. systemd starts `slab-compositor.service` on TTY
2. Compositor initializes DRM/KMS output (detects monitors)
3. Starts libinput for keyboard/mouse
4. Renders slab shell (tile grid, taskbar, menu bar)
5. Starts the web server in the background for remote access
6. Loads app manifests, builds tile grid
7. Ready — user sees the slab desktop on their monitor(s)

## Project Structure

```
compositor/
  src/
    main.rs           # entry point, event loop
    backend.rs        # DRM/KMS output, monitor management
    input.rs          # libinput handling
    shell.rs          # tile grid, taskbar, menu bar rendering
    window.rs         # Wayland client window management
    tiling.rs         # auto-tiling engine
    renderer.rs       # wgpu rendering
  apps/
    terminal/         # native Rust terminal (PTY + custom renderer)
    files/            # native Rust file browser
    editor/           # native Rust text editor
    sysmon/           # native Rust system monitor
    notes/            # native Rust notes (sticky, legal pad, sketch)
    settings/         # native Rust settings panel
  Cargo.toml
```

Each app in `compositor/apps/` is a Rust crate that implements a common trait:

```rust
pub trait SlabApp {
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn build(&self, ctx: &mut RenderContext) -> AppView;
    fn get_data(&self) -> Option<TileData>;     // for live tiles
    fn on_input(&mut self, event: InputEvent);
}
```

## Milestone Path

1. **Bare compositor** — blank screen, handles input, can spawn a terminal (alacritty/foot)
2. **Window management** — position, resize, focus, close Wayland windows
3. **Shell chrome** — render the taskbar and tile grid natively
4. **App integration** — load manifests, show tiles, launch apps
5. **Tiling engine** — auto-arrange, split ratios, full-screen toggle
6. **Multi-monitor** — dual HDMI on Pi 5
7. **Web server integration** — run alongside for remote access
8. **Session management** — boot from TTY, lock screen
