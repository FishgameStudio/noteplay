#![deny(missing_docs)]
#![deny(non_snake_case)]
#![deny(non_ascii_idents)]
#![deny(non_camel_case_types)]
#![deny(non_upper_case_globals)]
#![deny(unsafe_op_in_unsafe_fn)]
#![deny(unused_must_use)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::invalid_html_tags)]
#![warn(clippy::all)]
#![warn(clippy::nursery)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(explicit_outlives_requirements)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::semicolon_if_nothing_returned)]

//! # Noteplay: Note Player for Rust
//! **Noteplay** is a simple library for generating sine-wave musical notes.
//! Supports scientific pitch notation (C0–B8, sharps `#` / flats `b`),
//! synchronous blocking playback through your speaker.

pub mod core;
pub mod midi;
#[cfg(feature = "songs")]
pub mod songs;
