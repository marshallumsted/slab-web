# [S] slab

A brutalist webtop — a full desktop environment in the browser. Built for headless servers and VMs.

Sharp edges. Flat blocks. Zero rounded corners. One Rust binary. One browser tab. Your workflow, everywhere.

## Concept

**Build an app for your task.** slab workflows are custom apps — assembled from tiles, scoped folders, pinned bookmarks, and terminals — that open like any other app on the taskbar. One click and your entire working context is there: the right files, the right tabs, the right tools, already arranged.

Or don't. slab works as a regular desktop out of the box. Workflows are opt-in.

### What makes slab different

- **Tiles are not apps.** Tiles are the relevant pieces of apps, composed into a surface that fits your workflow. A file browser tile shows just your repos. Click deeper and it morphs into the full file browser. Click a document and it becomes the editor. The tile evolves to match what you're doing.
- **Workspaces are not virtual desktops.** macOS Spaces and Windows Task View just hide and show windows. slab workspaces change everything underneath — different files, different bookmarks, different terminal directories, different toolbar tools.
- **Modular apps.** Each app is a self-contained module in `frontend/apps/`. Drop a folder to add one, delete to remove. The desktop discovers apps dynamically — no hardcoded app list. Remove every app and the desktop still works.
- **Two bars.** Top bar: contextual app settings (left) + system icons and clock (right, KDE-style). Bottom bar: open apps and workflows (left) + quick-spawn tools (right). App settings appear automatically when you focus a window — apps just declare what settings they have, the desktop renders the UI.
- **The tile grid replaces the desktop.** No wallpaper. Live data tiles — CPU stats, service status, recent files — always visible, always updating.

## Features

- **Desktop shell** — window manager with drag, resize, minimize, maximize
- **Top menu bar** — contextual app settings, theme toggle, system icons, clock
- **Bottom taskbar** — open apps/workflows, quick-spawn tools from app manifests
- **Metro tile grid** — live data tiles, running app indicators, click to launch
- **Modular app system** — apps in `frontend/apps/`, self-register with `Slab.register()`, cross-app capabilities
- **File browser** — sidebar, list/grid views, editable path bar, network places (SMB, SFTP, FTP, NFS, WebDAV)
- **Terminal** — real shell via websocket PTY, tabs, split panes
- **Text editor** — Monaco-based, syntax highlighting, tabs, markdown preview, PDF viewer
- **System monitor** — live CPU/RAM/disk/network/temps, live tile data
- **Media viewer** — image/video display, folder scanning, thumbnail strip
- **Settings** — system-level: setup, theme, performance, network
- **Image & video previews** — lazy-loaded, cached ffmpeg thumbnails
- **File operations** — rename, copy, cut, paste, delete, new folder/file, download
- **Right-click menus** — context-aware actions everywhere
- **Dark & Light themes** — fully tokenized, instant switch
- **Two-tier config** — system-wide (`/etc/slab`) + per-user (`~/.config/slab`)
- **X11 bridge** — native Linux GUI apps via Xpra streaming

## App Architecture

```
frontend/apps/
  terminal/
    manifest.json       # name, tile config, spawn entries, settings, dependencies
    terminal.js         # self-registering app module
  sysmon/
    manifest.json
    sysmon.js
  files/  editor/  services/  logs/  media/  settings/
    ...
```

Each app registers with the shell via `Slab.register(id, { buildApp(), getData(), capabilities })`. The shell discovers apps via `GET /api/shell/apps`, loads dependencies, and builds the UI from manifests. Apps declare settings as data — the top menu bar renders them automatically.

## Install

### From source

```bash
git clone https://github.com/marshallumsted/slab.git
cd slab
cargo build --release
```

### Run

```bash
./target/release/slab
```

Open `http://localhost:8080` in your browser.

### Options

```bash
SLAB_PORT=3000 ./target/release/slab   # custom port
```

### Requirements

- Rust 1.70+
- ffmpeg (optional, for video thumbnails)
- Xpra (optional, for native Linux app streaming)

### Supported platforms

All major systemd-based Linux distributions:
Arch, Debian, Ubuntu, Fedora, openSUSE

## Planned

### Apps
- Notes (per-workspace plain text scratchpad)
- Service manager (real systemd interface via D-Bus)
- Log viewer (journalctl streaming)

### Interaction Model
- Workflows — workspace apps that scope files, bookmarks, terminals, and tiles to a task
- Adaptive tiles — tiles that morph through progressive disclosure (list → browser → editor)
- Quick Spawn elements — app elements (not full apps) spawned into the tile grid
- Sticky note styles — plain, legal pad (dual-column), canvas (drawing/handwriting)
- Hybrid tiling — auto-arrange like Hyprland, drag-to-resize like iPadOS, full-screen opt-in
- Customizable taskbar — layout, position, visibility, per-workflow tool sets
- Onboarding — guided workspace setup or skip to full desktop

### Platform
- Web app pinning (any URL as a window)
- Multi-user login screen
- User management (system users, sudo, profiles)
- Docker deployment

## Design

See [DESIGN.md](DESIGN.md) for interaction model, design language, and architecture.

See [BLUEPRINT.md](BLUEPRINT.md) for the phased build plan.

## Stack

Rust (axum + tokio) · HTML/CSS/JS · `/proc` · `/sys` · D-Bus · ffmpeg

## License

TBD
