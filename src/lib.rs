#![deny(missing_docs)]
#![deny(unknown_lints)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::invalid_html_tags)]
#![warn(clippy::all)]
//! # Noteplay: Note Player for Rust
//! **Noteplay** is a simple library for generating sine-wave musical notes.
//! Supports scientific pitch notation (C0–B8, sharps `#` / flats `b`),
//! synchronous blocking playback through your speaker.

pub mod core;
pub mod midi;
