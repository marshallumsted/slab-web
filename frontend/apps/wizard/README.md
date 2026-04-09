# Setup Wizard

First-time setup wizard for slab. Launches automatically on first run (no config file found) or manually from Settings > Setup.

## Purpose

Guide new users through a complete slab setup in under two minutes. No terminal commands, no manual config editing. Click through, done.

## Flow

### Step 1 — Welcome
- Detect environment: bare metal, Docker, or VM
- Show hostname, OS, architecture
- If Docker: show mount status immediately (what's accessible, what's missing)

### Step 2 — Dependencies
- Scan for required and optional packages (ffmpeg, xpra, git, etc.)
- Show what's installed, what's missing
- **Bare metal:** One-click install buttons per package, or "Install All" 
- **Docker:** Show which dependencies are in the image vs need host-side setup. Provide the recommended `docker run` command with all necessary flags and mounts. Copy button.

### Step 3 — Permissions
- Check filesystem access: can slab read/write the home directory?
- Check `/proc` and `/sys` access for system monitor
- Check D-Bus access for service manager
- **Bare metal:** Suggest adding user to required groups, show commands
- **Docker:** Show which `--mount`, `--pid`, `--privileged` flags are needed. Generate the fix command.

### Step 4 — User Config
- Set theme preference (dark/light)
- Set default shell (if terminal app is installed)
- Configure home directory path (auto-detected, editable)
- Add initial sidebar places for file browser

### Step 5 — Workspace (Optional)
- "Would you like to set up a workflow?"
- If yes: pick a template (coding, school, server admin, media) or start blank
- Scope a folder, pin some bookmarks, pick tile layout
- If no: skip to done

### Step 6 — Done
- Summary of what was configured
- "Open Desktop" button
- Link to Settings for further customization
- Wizard won't show again (writes a `setup_complete` flag to config)

## Architecture

- App folder: `frontend/apps/wizard/`
- Manifest: standard slab app, no tile (doesn't appear on tile grid after first run)
- The shell checks for `setup_complete` in user config on init — if missing, launches the wizard automatically
- Wizard uses existing backend APIs: `/api/setup`, `/api/config`, `/api/shell/environment`, `/api/user`
- Each step is a panel that slides in — no page reloads, no routing
- Progress bar at the top shows which step you're on
- Back button on every step, skip button on optional steps

## Manifest Notes

```json
{
  "id": "wizard",
  "name": "Setup Wizard",
  "order": 0,
  "tile": null,
  "window": { "width": 600, "height": 500 },
  "scripts": ["wizard.js"],
  "dependencies": [],
  "hidden": true
}
```

- `"tile": null` — no tile on the grid, wizard is not a regular app
- `"hidden": true` — shell skips this app when building tiles and spawn buttons
- `"order": 0` — loads first so it can intercept init if needed
- Can still be launched manually via `launchApp('wizard')` from settings
