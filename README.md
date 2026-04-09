# slab

A brutalist webtop — a full desktop environment in the browser, built on sharp edges, flat blocks, and zero rounded corners. Server management meets virtual desktop.

## What is this

Slab is a web-based desktop environment inspired by Windows 8 Metro design and brutalist architecture. It combines cockpit-style server management with a full windowed desktop — run system tools, web apps, and native Linux GUI apps side by side, all from a browser.

One Rust binary. One browser tab. A complete desktop.

## Design principles

- **0px border-radius.** No exceptions.
- **Flat solid colors.** No gradients, no shadows.
- **Bold type.** Clean sans-serif or monospace. High contrast.
- **Dense and information-heavy.** No wasted whitespace.
- **Sharp, blocky, unapologetic.**
- **iPhone settings philosophy.** All app and system settings centralized in one Settings app — no scattered menus.

## Architecture

Slab runs three tiers of applications in a unified window manager. All tiers tile, snap, drag, and resize the same way.

### Tier 1 — Native slab apps

Built-in apps written in web tech, served by the Rust backend. Fast, lightweight, zero streaming overhead.

- Terminal (real shell via websocket)
- File manager (sidebar, grid/list views, image/video previews, network places)
- System monitor (CPU, RAM, disk, network)
- Service manager (systemd)
- Log viewer (journalctl)
- Text editor
- Settings (centralized — all app + system config in one place)

### Tier 2 — Web apps as windows

Any URL pinned as a standalone app. No address bar, no tabs — it's a window, not a browser page.

- Pin any URL as an app (YouTube, Grafana, GitHub, etc.)
- Each gets its own window and taskbar entry
- Own start screen tile
- Iframe-based, sandboxed

### Tier 3 — X11 bridge

Real native Linux GUI apps streamed into slab windows via Xpra. For when you need the actual application.

- Browser (Brave, Firefox, etc.)
- Video player (VLC, mpv)
- Image editor (GIMP)
- Any installed GUI application
- Keyboard, mouse, clipboard forwarding

## Shell

- **Window manager** — drag, resize, minimize, maximize, snap to edges and halves
- **Taskbar** — floating, centered, red, auto-sizes based on open apps, [S] logo
- **Start screen** — Metro-style tile grid with live data tiles and app launchers
- **Right-click context menus** — system right-click suppressed, custom menus everywhere
- **Drag and drop** — files, windows, between panes (planned)

## Milestones

### Done
- Desktop shell (window manager, taskbar, start screen)
- File browser (sidebar, list/grid views, breadcrumb + editable path bar)
- Image/video previews (lazy-loaded, ffmpeg thumbnails, cached)
- File operations (rename, copy, cut, paste, delete, new folder/file, download)
- Selection system (click, ctrl+click, shift+click)
- Right-click context menus
- Network places (SMB, SFTP, FTP, NFS, WebDAV config — browsing not yet)
- Editable sidebar places
- Settings app (centralized, sidebar nav, per-app sections)
- Performance settings (animations, dot grid, backdrop blur)
- Config persistence (~/.config/slab/config.json)

### Next
- Drag and drop (files between panes, to/from desktop, reorder sidebar)
- Split-screen file browser (two panes side by side)
- User management (system users, passwords, sudo permissions, profile pictures)
- Login screen (multi-user, per-user desktops)
- Terminal app (real shell via websocket)
- System monitor (live stats from /proc)
- Service manager (systemd via D-Bus)
- Log viewer (journalctl stream)
- Text editor (syntax highlighting)
- Tier 2: web app pinning
- Tier 3: X11 bridge via Xpra
- Docker deployment
- System language settings

## Stack

- **Backend:** Rust (axum + tokio)
- **Frontend:** HTML/CSS/JS
- **System interfaces:** `/proc`, `/sys`, D-Bus (systemd)
- **Thumbnails:** ffmpeg (video frame extraction, cached)
- **X11 bridge:** Xpra (optional, for Tier 3 apps)
- **Deployment:** single binary, or Docker

## Target platforms

All major systemd-based Linux distributions:
- Arch
- Debian
- Ubuntu
- Fedora (RHEL/CentOS)
- openSUSE

## License

TBD
