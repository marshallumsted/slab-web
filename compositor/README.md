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

## Two Modes

### Native Mode
- Smithay-based Wayland compositor written in Rust
- Renders slab's UI natively: tile grid, top menu bar, bottom taskbar, windows
- GPU-accelerated via Vulkan/OpenGL (wgpu or direct EGL)
- Wayland clients (native Linux apps) run as windows inside slab
- Multi-monitor support (Pi 5 has dual HDMI)
- Direct input handling (keyboard, mouse, touch)
- Boots straight from TTY — no login manager needed, systemd service

### Remote Mode
- The existing web server (axum + HTML/CSS/JS)
- Access the same desktop from any browser on any device
- Can run alongside native mode — use the physical display AND access remotely
- Downloadable as a separate module if you only want remote access

## Why Smithay

- Pure Rust — no C bindings, no unsafe wrappers
- Built for exactly this use case — custom Wayland compositors
- Handles the hard parts: DRM/KMS output, libinput, EGL/Vulkan, Wayland protocol
- Active project, used by other Rust compositors (cosmic-comp, etc.)
- Same language as the rest of slab — one codebase, one build

## Rendering

The compositor needs to render slab's UI. Options:

### Option A — Custom renderer
- Render the tile grid, taskbar, menu bar, and windows directly using wgpu or Skia
- Most performant, most work
- Full control over every pixel — slab's brutalist design rendered natively

### Option B — Embedded WebView
- Use wry/webview2 to embed a browser engine inside the compositor
- The compositor manages windows and input, the WebView renders slab's HTML/CSS/JS
- Reuses all existing frontend code
- Native Wayland apps coexist alongside web-rendered slab apps

### Option C — Hybrid
- Shell chrome (taskbar, tile grid, menu bar) rendered natively for performance
- App content rendered via embedded WebView per-window
- Native Wayland apps get native windows, slab web apps get WebView windows
- Best of both but most complex

## What the Compositor Owns

- **Output management** — DRM/KMS, monitor detection, resolution, multi-head
- **Input** — libinput for keyboard, mouse, touchpad, touchscreen
- **Window management** — Wayland client windows (xdg_toplevel), positioning, focus, stacking
- **Tiling engine** — the Hyprland/iPadOS hybrid tiling from the design doc
- **Shell rendering** — tile grid, taskbar, menu bar drawn on screen
- **Session** — TTY login, user switching, lock screen

## What the Compositor Shares with Web Mode

- **App manifests** — same `frontend/apps/*/manifest.json` files
- **Backend APIs** — same `/api/*` endpoints for files, sysmon, config, etc.
- **Config system** — same `~/.config/slab/config.json`
- **Workspace definitions** — same workspace JSON files
- **Design tokens** — colors, fonts, spacing (expressed as Rust constants instead of CSS variables)

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
    renderer.rs       # wgpu/skia rendering
  Cargo.toml
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
