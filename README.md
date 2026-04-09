# [S] slab

A brutalist webtop — a full desktop environment in the browser. Built for headless servers and VMs.

Sharp edges. Flat blocks. Zero rounded corners. One Rust binary. One browser tab. Your workflow, everywhere.

## Concept

**Build an app for your task.** slab workflows are custom apps — assembled from tiles, scoped folders, pinned bookmarks, and terminals — that open like any other app on the taskbar. One click and your entire working context is there: the right files, the right tabs, the right tools, already arranged.

Or don't. slab works as a regular desktop out of the box. Workflows are opt-in.

### What makes slab different

- **Tiles are not apps.** Tiles are the relevant pieces of apps, composed into a surface that fits your workflow. A file browser tile shows just your repos. Click deeper and it morphs into the full file browser. Click a document and it becomes the editor. The tile evolves to match what you're doing.
- **Workspaces are not virtual desktops.** macOS Spaces and Windows Task View just hide and show windows. slab workspaces change everything underneath — different files, different bookmarks, different terminal directories, different toolbar tools.
- **The taskbar is a toolbar.** Left side: open apps and workflows. Right side: quick-spawn tools (sticky notes, terminal, file browser, timer, media controls), settings, clock. Customizable per-workflow.
- **The tile grid replaces the desktop.** No wallpaper. Live data tiles — CPU stats, service status, recent files — always visible, always updating.

## Features

- **Desktop shell** — window manager with drag, resize, minimize, maximize
- **Taskbar** — toolbar with open apps, quick-spawn tools, settings popup, clock
- **Metro tile grid** — live data tiles, fluid masonry layout, resizable
- **File browser** — sidebar, list/grid views, editable path bar, network places (SMB, SFTP, FTP, NFS, WebDAV)
- **Image & video previews** — lazy-loaded, cached ffmpeg thumbnails
- **File operations** — rename, copy, cut, paste, delete, new folder/file, download
- **Right-click menus** — context-aware actions everywhere
- **Settings** — centralized, per-app sections, performance toggles
- **Dark & Light themes** — fully tokenized, instant switch
- **Two-tier config** — system-wide (`/etc/slab`) + per-user (`~/.config/slab`)

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

### Supported platforms

All major systemd-based Linux distributions:
Arch, Debian, Ubuntu, Fedora, openSUSE

## Planned

### Apps
- Terminal (real shell via websocket PTY)
- System monitor (live CPU/RAM/disk/network)
- Service manager (systemd start/stop/restart via D-Bus)
- Log viewer (journalctl streaming)
- Text editor with syntax highlighting
- Notes (per-workspace plain text scratchpad)

### Interaction Model
- Workflows — workspace apps that scope files, bookmarks, terminals, and tiles to a task
- Adaptive tiles — tiles that morph through progressive disclosure (list → browser → editor)
- Quick Spawn — taskbar tools to drop app elements (sticky notes, terminals, bookmarks) into any context
- Sticky note styles — plain, legal pad (dual-column), canvas (drawing/handwriting)
- Hybrid tiling — auto-arrange like Hyprland, drag-to-resize like iPadOS, full-screen opt-in
- Customizable taskbar — layout, position, visibility, per-workflow tool sets
- Onboarding — guided workspace setup or skip to full desktop

### Platform
- Web app pinning (any URL as a window)
- X11 app streaming via Xpra
- Multi-user login screen
- User management (system users, sudo, profiles)
- Docker deployment

## Design

See [DESIGN.md](DESIGN.md) for architecture, design language, and detailed plans.

## Stack

Rust (axum + tokio) · HTML/CSS/JS · `/proc` · `/sys` · D-Bus · ffmpeg

## License

TBD
