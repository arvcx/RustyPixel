//! RustyPixel core engine.
//!
//! This crate holds the engine-side systems shared by the client and the
//! dedicated server: world storage, player management, configuration, and so
//! on. In Phase 0 these modules are intentionally minimal stubs.

#![allow(dead_code)] // Stub modules: real systems land in later phases.

pub mod config;
pub mod player;
pub mod world;