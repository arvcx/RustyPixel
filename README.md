# RustyPixel

An open and extensible **2D multiplayer pixel sandbox platform** built with
Rust and Lua.

- **Rust** — performance, core systems, server, rendering.
- **Lua** — gameplay, customization, modding (Phase 6).
- **Community** — worlds, mods, servers.

## Workspace

```
crates/
  client/      Bevy client (game window)
  server/      Dedicated server (placeholder)
  engine/      Core engine + config
  protocol/    Shared network types
  scripting/   Lua runtime (Phase 6)
assets/textures/   Game art
game/default/      Default Lua scripts (Phase 6)
```

## Getting started

Requires Rust (edition 2021).

```bash
cargo check --workspace
cargo test --workspace
cargo run -p rustypixel-client   # open the game window
cargo run -p rustypixel-server   # dedicated server (placeholder)
```

## Development phases

Phased roadmap in `PRD.md §34`. Do not implement future phases ahead of
schedule.