//! Shared network message types between client and server.
//!
//! Phase 0 defines only stub message types; the real protocol (handshake,
//! world sync, player state) is designed in Phase 8 (Multiplayer).

#![allow(dead_code)] // Stub enums below are not constructed anywhere yet.

use serde::{Deserialize, Serialize};

/// A single message sent between client and server.
///
/// Phase 0 contains three marker variants so the type is usable in later
/// phases. Real payloads are added with the networking work in Phase 8.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Message {
    /// Client → server: authentication / hello.
    Hello { protocol_version: u32 },
    /// Server → client: acknowledgement of a `Hello`.
    Welcome { server_name: String },
    /// Placeholder for all future message kinds.
    Ping { payload: u32 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_round_trips_through_toml_style_serialization() {
        let msg = Message::Hello {
            protocol_version: 0,
        };
        let encoded = toml::to_string(&msg).expect("serialize");
        let decoded: Message = toml::from_str(&encoded).expect("deserialize");
        assert_eq!(msg, decoded);
    }
}