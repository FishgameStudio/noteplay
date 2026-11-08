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

//! # Noteplay: Note & Midi Player for Rust
//! **Noteplay** is a Rust crate for turning
//! musical notes and simple arrangements into
//! sound. It can play individual notes through
//! the system's default audio output, schedule
//! notes together into chords and sequences, and
//! play short songs at a chosen tempo. It's
//! intended as a straightforward starting point
//! for adding musical playback to Rust projects -
//! not as a full digital audio workstation.

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

pub mod core;
pub mod midi;
#[cfg(feature = "songs")]
pub mod songs;
