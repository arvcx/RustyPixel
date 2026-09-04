# AGENTS.md

RustyPixel is a Cargo workspace (edition 2021, resolver 2) with crates mounted
under `crates/*`:

- `crates/client` — Bevy 0.19 client binary, opens the game window.
- `crates/server` — dedicated server binary (placeholder until Phase 8).
- `crates/engine` — core engine lib: `world`, `player`, `config` (stubs);
  `config::ServerConfig` loads `server.toml` (serde + toml).
- `crates/protocol` — shared network `Message` types (stubs, serde).
- `crates/scripting` — Lua runtime crate, intentionally empty until Phase 6.

Conventions:

- Internal crates are named `rustypixel-*` and referenced as `rustypixel_*` in
  code; shared deps live under `[workspace.dependencies]`.
- Logging uses `tracing` + `tracing_subscriber` (initialized in each binary's
  `main`).
- Phases are defined in `PRD.md §34`. Do not implement future phases ahead of
  schedule; keep stubs compiling with `#![allow(dead_code)]` where needed.