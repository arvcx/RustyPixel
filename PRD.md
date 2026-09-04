<html><head></head><body><h1>RustyPixel — Product Requirements Document</h1><blockquote><p><strong>Status:</strong> Draft<br><strong>Project:</strong> RustyPixel<br><strong>Primary Language:</strong> Rust<br><strong>Game Framework:</strong> Bevy<br><strong>Scripting:</strong> Lua<br><strong>Platform:</strong> Desktop first, Android planned<br><strong>License:</strong> TBD</p></blockquote><hr><h1>1. Overview</h1><h2>1.1 What is RustyPixel?</h2><p>RustyPixel is an open and extensible <strong>2D multiplayer pixel sandbox platform</strong> built with Rust.</p><p>The project provides:</p><ul><li><p>A 2D pixel-art sandbox client</p></li><li><p>A dedicated multiplayer server</p></li><li><p>Lua scripting for gameplay</p></li><li><p>A modding system</p></li><li><p>Community-hosted servers</p></li><li><p>An official global beta server</p></li><li><p>Original assets and gameplay</p></li></ul><p>RustyPixel should not depend on a single centralized server. Community members should be able to host and customize their own servers.</p><hr><h1>2. Product Vision</h1><p>RustyPixel aims to provide a simple but powerful sandbox platform where:</p><blockquote><p><strong>Rust handles performance and core systems.</strong><br><strong>Lua handles gameplay and customization.</strong><br><strong>The community creates content, mods, and servers.</strong></p></blockquote><p>The project should be easy enough for beginners to modify while remaining performant and maintainable.</p><hr><h1>3. Core Principles</h1><h2>3.1 Extensibility</h2><p>Game-specific content should not be unnecessarily hardcoded into Rust.</p><p>Examples:</p><ul><li><p>Items</p></li><li><p>Blocks</p></li><li><p>Commands</p></li><li><p>Gameplay rules</p></li><li><p>Events</p></li><li><p>Abilities</p></li></ul><p>These should be customizable through Lua.</p><hr><h2>3.2 Community Ownership</h2><p>Users should be able to:</p><ul><li><p>Host their own server</p></li><li><p>Create custom worlds</p></li><li><p>Create custom items</p></li><li><p>Create custom blocks</p></li><li><p>Create mods</p></li><li><p>Change gameplay rules</p></li></ul><p>The official server is only one possible server.</p><hr><h2>3.3 Original Content</h2><p>RustyPixel must use:</p><ul><li><p>Original pixel art</p></li><li><p>Original item names</p></li><li><p>Original worlds</p></li><li><p>Original gameplay content</p></li></ul><p>RustyPixel should be inspired by the broader 2D sandbox genre without copying another game's proprietary assets or content.</p><hr><h1>4. Technology Stack</h1><table><thead><tr><th>System</th><th>Technology</th></tr></thead><tbody><tr><td>Programming Language</td><td>Rust</td></tr><tr><td>Game Framework</td><td>Bevy</td></tr><tr><td>Gameplay Scripting</td><td>Lua</td></tr><tr><td>Lua Runtime</td><td>mlua</td></tr><tr><td>Rendering</td><td>Bevy</td></tr><tr><td>Networking</td><td>Rust</td></tr><tr><td>Async Runtime</td><td>Tokio</td></tr><tr><td>Serialization</td><td>TBD</td></tr><tr><td>Assets</td><td>PNG</td></tr><tr><td>Configuration</td><td>TOML</td></tr></tbody></table><hr><h1>5. High-Level Architecture</h1><pre><code class="language-text">                    RUSTYPIXEL
                         │
        ┌────────────────┴────────────────┐
        │                                 │
      CLIENT                            SERVER
        │                                 │
        ▼                                 ▼
   Bevy Runtime                     Rust Runtime
        │                                 │
        ├── Rendering                       ├── Networking
        ├── Input                           ├── World Authority
        ├── Animation                       ├── Player Management
        ├── UI                              └── Lua Runtime
        │
        └──────────────┐
                       │
                    Lua API
                       │
              │                 │
            Game               Mods
</code></pre><hr><h1>6. Core Engine Responsibilities</h1><p>The Rust engine is responsible for fundamental systems.</p><h2>Rust Responsibilities</h2><pre><code class="language-text">Rendering
Physics
Collision
Input
Networking
World Storage
Entity Management
Asset Loading
Animation Runtime
Lua Runtime
Server Runtime
</code></pre><p>Rust should provide a stable API to Lua.</p><hr><h1>7. Lua Responsibilities</h1><p>Lua is responsible for gameplay and customization.</p><pre><code class="language-text">Items
Blocks
Commands
Gameplay Rules
Events
Abilities
Mods
NPC Behavior
Server Rules
</code></pre><p>Example:</p><pre><code class="language-lua">engine.register_item({
    id = "example:wings",
    name = "Example Wings",

    equipment = {
        slot = "back"
    }
})
</code></pre><hr><h1>8. Client</h1><p>The RustyPixel client should provide:</p><ul><li><p>Player rendering</p></li><li><p>World rendering</p></li><li><p>Input</p></li><li><p>UI</p></li><li><p>Animation</p></li><li><p>Audio</p></li><li><p>Networking</p></li><li><p>Asset loading</p></li></ul><p>Initial platforms:</p><pre><code class="language-text">Desktop
├── Linux
├── Windows
└── macOS

Future:
└── Android
</code></pre><hr><h1>9. Player System</h1><h2>9.1 Base Character</h2><p>The default player should be simple and universal.</p><p>Initial appearance:</p><ul><li><p>Bald</p></li><li><p>Simple eyes</p></li><li><p>Simple face</p></li><li><p>Basic body</p></li><li><p>Default tank top</p></li></ul><p>The base character should not be permanently gender-locked.</p><hr><h2>9.2 Player Animation</h2><p>Initial animations:</p><pre><code class="language-text">Idle
Walk
Jump
Fall
Break Block
Place Block
</code></pre><p>Future animations:</p><pre><code class="language-text">Run
Sit
Emote
Dance
Use Item
Damage
</code></pre><hr><h1>10. Equipment System</h1><p>Player customization should use a generic attachment system.</p><h2>Equipment Slots</h2><pre><code class="language-text">Hair
Hat
Face
Shirt
Pants
Shoes
Back
Hand
Aura
</code></pre><p>Example:</p><pre><code class="language-text">Player
│
├── Base Body
├── Hair
├── Shirt
├── Pants
├── Shoes
└── Back Equipment
</code></pre><hr><h1>11. Attachment Rendering</h1><p>Equipment should render as layers.</p><p>Example:</p><pre><code class="language-text">Layer 0 → Back Equipment
Layer 1 → Base Body
Layer 2 → Shirt
Layer 3 → Pants
Layer 4 → Hair
Layer 5 → Face Accessory
Layer 6 → Front Accessory
</code></pre><p>Equipment should follow the player's current animation.</p><hr><h1>12. Ability System</h1><p>Equipment may grant abilities.</p><p>Example:</p><pre><code class="language-text">Angel Wings
└── Double Jump

Rocket Pack
└── Air Movement

Speed Shoes
└── Movement Speed Modifier
</code></pre><p>The engine should not hardcode specific items such as <code inline="">Wings</code>.</p><p>Instead:</p><pre><code class="language-text">Equipment
     │
     ▼
Ability
     │
     ▼
Player
</code></pre><p>Example concept:</p><pre><code class="language-lua">engine.register_item({
    id = "example:jump_wings",

    equipment = {
        slot = "back"
    },

    abilities = {
        "double_jump"
    }
})
</code></pre><hr><h1>13. World System</h1><p>The world consists of a tile grid.</p><p>Initial features:</p><ul><li><p>Tile placement</p></li><li><p>Tile breaking</p></li><li><p>Collision</p></li><li><p>World saving</p></li><li><p>World loading</p></li></ul><p>Example:</p><pre><code class="language-text">🧱🧱🧱🧱🧱🧱
🧱          🧱
🧱    👤    🧱
🧱          🧱
🧱🧱🧱🧱🧱🧱
</code></pre><hr><h1>14. Block System</h1><p>Blocks should be data-driven.</p><p>Example properties:</p><pre><code class="language-text">ID
Name
Texture
Collision
Break Time
Drops
Behavior
</code></pre><p>Example Lua:</p><pre><code class="language-lua">engine.register_block({
    id = "example:dirt",
    name = "Dirt",

    solid = true,

    texture = "assets/blocks/dirt.png"
})
</code></pre><hr><h1>15. Inventory System</h1><p>Initial inventory features:</p><pre><code class="language-text">Item Storage
Stacking
Item Selection
Equip Item
Unequip Item
Place Block
Use Item
</code></pre><p>Future:</p><pre><code class="language-text">Crafting
Trading
Storage Containers
Item Metadata
</code></pre><hr><h1>16. Asset System</h1><p>The primary asset format should be:</p><pre><code class="language-text">PNG
</code></pre><p>Assets may include:</p><pre><code class="language-text">Sprites
Tiles
Items
Equipment
UI
Particles
Backgrounds
</code></pre><hr><h1>17. Animation System</h1><p>RustyPixel should use:</p><pre><code class="language-text">PNG Sprite Sheet
+
Animation Metadata
</code></pre><p>Example animation definitions:</p><pre><code class="language-text">Idle
Walk
Jump
Fall
Break
Place
</code></pre><p>Equipment assets should support compatible animation frames.</p><p>Example:</p><pre><code class="language-text">body.png
hair.png
shirt.png
pants.png
wings.png
</code></pre><p>When the player enters the <code inline="">Walk</code> animation, all compatible layers should render the appropriate frame.</p><hr><h1>18. Lua API</h1><p>Lua should interact with the engine through a controlled API.</p><p>Example:</p><pre><code class="language-lua">engine.on("player_join", function(player)
    player:message("Welcome!")
end)
</code></pre><p>Example events:</p><pre><code class="language-text">player_join
player_leave

player_move

tile_break
tile_place

item_use

chat_message

world_load
world_save
</code></pre><hr><h1>19. Event System</h1><p>The engine should expose events to Lua.</p><p>Example:</p><pre><code class="language-lua">engine.on("tile_break", function(event)

    print("Tile broken")

end)
</code></pre><p>Some events may support cancellation.</p><p>Example:</p><pre><code class="language-lua">engine.on("tile_place", function(event)

    if event.world.name == "spawn" then
        event:cancel()
    end

end)
</code></pre><hr><h1>20. Modding System</h1><p>Mods should allow community-created content.</p><p>Example:</p><pre><code class="language-text">mods/
└── example_mod/
    ├── mod.toml
    ├── init.lua
    │
    ├── scripts/
    │   └── items.lua
    │
    └── assets/
        └── example_item.png
</code></pre><hr><h1>21. Mod Manifest</h1><p>Example:</p><pre><code class="language-toml">id = "example_mod"
name = "Example Mod"
version = "1.0.0"

[dependencies]
</code></pre><p>Future metadata:</p><pre><code class="language-text">Author
Description
Website
Dependencies
Compatible Version
</code></pre><hr><h1>22. Lua Sandbox</h1><p>Community scripts should not automatically receive unrestricted operating system access.</p><p>Lua should only access APIs explicitly provided by Rust.</p><p>Allowed APIs may include:</p><pre><code class="language-text">World API
Player API
Item API
Entity API
Event API
Storage API
</code></pre><hr><h1>23. Multiplayer Architecture</h1><p>RustyPixel uses a client-server architecture.</p><pre><code class="language-text">CLIENT
   │
   │ Action Request
   ▼
SERVER
   │
   ├── Validate
   ├── Update Game State
   ├── Execute Gameplay Events
   │
   ▼
Broadcast Update
   │
   ▼
CLIENTS
</code></pre><hr><h1>24. Server Authority</h1><p>The server is authoritative.</p><p>Clients should request actions rather than directly modifying game state.</p><p>Example:</p><pre><code class="language-text">Client:
"Place this block."

        ↓

Server:
"Is this valid?"

        ↓

Update World

        ↓

Broadcast Result
</code></pre><p>The client should not control:</p><pre><code class="language-text">Inventory Amount
World State
Other Player State
Server Rules
</code></pre><hr><h1>25. Dedicated Server</h1><p>RustyPixel must provide a standalone dedicated server.</p><p>Example:</p><pre><code class="language-bash">rustypixel-server
</code></pre><p>Server configuration:</p><pre><code class="language-text">server.toml
</code></pre><p>Example:</p><pre><code class="language-toml">name = "My RustyPixel Server"
max_players = 50
port = 17091
</code></pre><hr><h1>26. Community Servers</h1><p>Users should be able to:</p><pre><code class="language-text">Download Server
      ↓
Configure Server
      ↓
Modify Lua Scripts
      ↓
Install Mods
      ↓
Start Server
</code></pre><p>Example:</p><pre><code class="language-bash">rustypixel-server start
</code></pre><hr><h1>27. Official Beta Server</h1><p>RustyPixel may provide an official global beta server.</p><p>Example:</p><pre><code class="language-text">OFFICIAL BETA SERVER

Status: Online

Players: 10/100
</code></pre><p>The official server uses the same server software available to the community.</p><pre><code class="language-text">                  SAME SERVER SOFTWARE

          ┌──────────────┴──────────────┐
          │                             │
          ▼                             ▼

    Official Server              Community Server

    Official Lua                 Custom Lua
    Official Mods                Community Mods
    Official Rules               Custom Rules
</code></pre><hr><h1>28. Server Connection</h1><p>The client should eventually support:</p><pre><code class="language-text">Play Official Beta
Community Servers
Direct Connect
Host Local Server
</code></pre><p>Example:</p><pre><code class="language-text">┌──────────────────────────┐
│        RUSTYPIXEL        │
│                          │
│ [ PLAY OFFICIAL BETA ]   │
│                          │
│ [ COMMUNITY SERVERS ]    │
│                          │
│ [ DIRECT CONNECT ]       │
│                          │
│ [ HOST LOCAL SERVER ]    │
│                          │
└──────────────────────────┘
</code></pre><hr><h1>29. Server Browser</h1><p>Future feature:</p><pre><code class="language-text">┌─────────────────────────────────┐
│       COMMUNITY SERVERS         │
├─────────────────────────────────┤
│ 🟢 Sandbox World       12/50    │
│ 🟢 Farming Server        5/30    │
│ 🟢 Minigames            20/50    │
│ 🔴 Test Server          Offline  │
└─────────────────────────────────┘
</code></pre><p>This feature is not required for the initial version.</p><hr><h1>30. Project Structure</h1><p>Initial Cargo workspace:</p><pre><code class="language-text">rustypixel/
│
├── crates/
│   │
│   ├── client/
│   │
│   ├── server/
│   │
│   ├── engine/
│   │
│   ├── protocol/
│   │
│   └── scripting/
│
├── assets/
│
├── game/
│   └── default/
│       ├── init.lua
│       ├── items.lua
│       ├── blocks.lua
│       └── gameplay.lua
│
├── mods/
│
├── examples/
│
├── docs/
│
├── Cargo.toml
└── README.md
</code></pre><hr><h1>31. Development Workflow</h1><p>RustyPixel should initially be code-first.</p><pre><code class="language-text">Code
 ↓
cargo run
 ↓
Game Window
 ↓
Test
</code></pre><p>No graphical editor is required initially.</p><hr><h1>32. Future CLI</h1><p>Future commands may include:</p><pre><code class="language-bash">rustypixel new my-game
</code></pre><pre><code class="language-bash">rustypixel run
</code></pre><pre><code class="language-bash">rustypixel server start
</code></pre><pre><code class="language-bash">rustypixel build
</code></pre><hr><h1>33. Future GUI Editor</h1><p>A graphical editor may be developed after the core engine is stable.</p><p>Possible features:</p><pre><code class="language-text">World Editor
Item Editor
Animation Editor
Character Preview
Server Configuration
Mod Manager
</code></pre><p>Potential stack:</p><pre><code class="language-text">Rust
+
Bevy
+
egui
</code></pre><hr><h1>34. Development Phases</h1><h2>Phase 0 — Foundation</h2><p>Goals:</p><pre><code class="language-text">Cargo Workspace
Bevy Setup
Client
Server Placeholder
Engine Crate
Shared Protocol
Logging
Configuration
</code></pre><hr><h2>Phase 1 — Basic Rendering</h2><p>Goals:</p><pre><code class="language-text">Window
Camera
Pixel Scaling
Sprite Rendering
Player Spawn
</code></pre><hr><h2>Phase 2 — Player Controller</h2><p>Goals:</p><pre><code class="language-text">Movement
Jump
Gravity
Collision
Ground Detection
Animation State
</code></pre><p>Animations:</p><pre><code class="language-text">Idle
Walk
Jump
Fall
</code></pre><hr><h2>Phase 3 — Tile World</h2><p>Goals:</p><pre><code class="language-text">Tile Grid
World Rendering
Block Collision
World Save
World Load
</code></pre><hr><h2>Phase 4 — Block Interaction</h2><p>Goals:</p><pre><code class="language-text">Target Block
Break Block
Place Block
Particles
Basic Inventory
</code></pre><hr><h2>Phase 5 — Items and Equipment</h2><p>Goals:</p><pre><code class="language-text">Item Registry
Inventory
Equipment Slots
Hair
Shirt
Pants
Back Equipment
Ability System
</code></pre><hr><h2>Phase 6 — Lua Integration</h2><p>Goals:</p><pre><code class="language-text">mlua
Lua Runtime
Lua API
Events
Item Registration
Block Registration
Commands
</code></pre><hr><h2>Phase 7 — Modding</h2><p>Goals:</p><pre><code class="language-text">Mod Manifest
Mod Loading
Dependencies
Mod Assets
Lua Sandbox
Example Mods
</code></pre><hr><h2>Phase 8 — Multiplayer</h2><p>Goals:</p><pre><code class="language-text">Dedicated Server
Client Connection
Player Synchronization
World Synchronization
Server Authority
</code></pre><hr><h2>Phase 9 — Community Servers</h2><p>Goals:</p><pre><code class="language-text">Server Configuration
Custom Lua
Custom Worlds
Custom Mods
Direct Connection
Local Hosting
</code></pre><hr><h2>Phase 10 — Community Ecosystem</h2><p>Goals:</p><pre><code class="language-text">Official Beta
Server Browser
Documentation
Examples
CLI Tools
Mod Documentation
</code></pre><hr><h1>35. Testing Requirements</h1><p>Every phase must include:</p><pre><code class="language-text">Build Verification
Basic Tests
Manual Testing
Error Handling
Documentation
</code></pre><p>Recommended workflow:</p><pre><code class="language-text">Implement
    ↓
cargo check
    ↓
cargo test
    ↓
cargo run
    ↓
Manual Test
    ↓
Git Commit
</code></pre><hr><h1>36. AI Development Rules</h1><p>When using AI coding models:</p><p>Do not request the entire project at once.</p><p>Bad:</p><pre><code class="language-text">"Build a complete Growtopia clone in Rust."
</code></pre><p>Good:</p><pre><code class="language-text">"Implement Phase 1 only.
Do not implement future phases.
Ensure the project compiles.
Do not add unnecessary dependencies."
</code></pre><p>Every AI task should include:</p><pre><code class="language-text">Scope
Requirements
Architecture
Implementation
Tests
Verification
</code></pre><hr><h1>37. Non-Goals</h1><p>RustyPixel should NOT initially attempt to become:</p><pre><code class="language-text">Unity Clone
Unreal Engine Clone
Generic 3D Engine
Complete Game Editor
Complete MMORPG Platform
</code></pre><p>The initial focus is:</p><blockquote><p><strong>A clean, extensible 2D multiplayer pixel sandbox platform.</strong></p></blockquote><hr><h1>38. Definition of MVP</h1><p>The first playable MVP should include:</p><pre><code class="language-text">✓ 2D World
✓ Player
✓ Movement
✓ Jump
✓ Collision
✓ Basic Animation
✓ Blocks
✓ Break Blocks
✓ Place Blocks
✓ Basic Inventory
✓ Save World
✓ Load World
</code></pre><p>Lua and multiplayer may be added after the basic sandbox loop is stable.</p><hr><h1>39. Long-Term Vision</h1><p>RustyPixel should evolve into a platform where users can:</p><pre><code class="language-text">Create Worlds
Create Items
Create Blocks
Create Mods
Write Lua Scripts
Host Servers
Create Communities
</code></pre><p>The project architecture should allow experimentation without requiring users to modify the Rust engine.</p><hr><h1>40. Final Product Philosophy</h1><pre><code class="language-text">             RUSTYPIXEL

                  🦀
                 RUST
             Engine Core

                  │

          ┌───────┴───────┐

          🌙              🎨
         LUA            ASSETS

      Gameplay         Community
      Mods             Content
      Rules

          │
          ▼

          🌐
      COMMUNITY

   Servers + Mods + Worlds
</code></pre><h2>Final Statement</h2><p>RustyPixel is an extensible 2D pixel sandbox platform built with Rust and Lua.</p><p>Rust provides:</p><blockquote><p>Performance, stability, networking, rendering, and core systems.</p></blockquote><p>Lua provides:</p><blockquote><p>Gameplay, customization, and modding.</p></blockquote><p>The community provides:</p><blockquote><p>Worlds, servers, content, and creativity.</p></blockquote><p><strong>The official server is part of the RustyPixel ecosystem, not the entire ecosystem.</strong></p></body></html>
