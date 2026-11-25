// xeton_core — Rust backend for the Xeton Download Manager
// This crate is the UniFFI-exposed engine replacing the Kotlin downloader/core module.
// Full implementation is built up phase by phase.

#![allow(dead_code, unused_variables, deprecated)]
#![allow(clippy::all)]

pub mod connection;
pub mod db;
pub mod destination;
pub mod privacy;
pub mod vault;
pub mod downloader;
pub mod extractors;
pub mod hls;
pub mod http;
pub mod manager;
pub mod models;
pub mod part;
pub mod protocols;
pub mod queue;
pub mod throttle;
pub mod transcoder;
pub mod uniffi_api;

pub use uniffi_api::{extract_audio, extract_media_info, merge_video_audio};

#[cfg(feature = "sync")]
pub mod sync;

uniffi::setup_scaffolding!();
