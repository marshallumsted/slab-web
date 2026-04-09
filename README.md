# slab

A brutalist webtop — a full desktop environment in the browser, built on sharp edges, flat blocks, and zero rounded corners.

## What is this

Slab is a web-based desktop environment inspired by Windows 8 Metro design and brutalist architecture. It provides a complete windowed shell with a taskbar, start screen, and built-in system management apps — all served from a single Rust binary.

## Design principles

- **0px border-radius.** No exceptions.
- **Flat solid colors.** No gradients, no shadows.
- **Bold type.** Clean sans-serif or monospace. High contrast.
- **Dense and information-heavy.** No wasted whitespace.
- **Sharp, blocky, unapologetic.**

## Planned features

### Shell
- Window manager (drag, resize, minimize, maximize, snap)
- Taskbar with open apps, clock, system tray
- Metro-style start screen with live tiles

### Built-in apps
- Terminal (real shell via websocket)
- File manager
- System monitor (CPU, RAM, disk, network)
- Service manager (systemd)
- Log viewer (journalctl)
- Text editor
- Settings

## Stack

- **Backend:** Rust (axum + tokio)
- **Frontend:** HTML/CSS/JS
- **System interfaces:** `/proc`, `/sys`, D-Bus (systemd)
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
