# D&D Encounter Manager

**Lightweight offline D&D 5e Encounter Manager** built with **Rust + Tauri 2 + Svelte 5**.

Faithful to the original specification in Google Drive (Parts 1–4).

## Features (Phase 1 MVP)
- Single-DM desktop app for managing one D&D 5e (2014) encounter
- Monster library (persistent JSON DB, importable via canonical schema)
- Auto-roll initiative for monsters + 5-step tie-breaking rules
- Manual player initiative entry
- HP tracking (DM-only edits, clamp ≥0, auto-remove at 0)
- 15 standard conditions with toggle
- Undo stack (last 5 actions)
- Auto-save every 60s + on changes (single save file, auto-restore on launch)
- Full stat block viewer
- Compact initiative sidebar
- Suggested hotkeys
- Max 20 entities per encounter
- SRD monsters + custom import support

## Tech Stack
- **Backend**: Rust (Tauri 2) — pure domain logic, commands, persistence
- **Frontend**: Svelte 5 + TypeScript
- **Storage**: Local JSON files (human-readable)
- **License**: MIT

## Development Phases
See `PHASES.md` (to be added) for the full TDD roadmap with tests.

## Quick Start
```bash
npm install
npm run tauri dev
```

## Links
- Google Drive Spec: (your Drive folder)
- GitHub: https://github.com/Shiely/dnd-encounter-manager

## Status
Phase 0 complete (setup + alignment). Phase 1 (domain models + rules) in progress.

Built for Dungeon Masters who want faster, error-free monster turns.