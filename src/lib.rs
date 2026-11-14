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

/// Top-level error type for this crate.
///
/// This error type is returned by all fallible public functions in the crate.
/// Use pattern matching to inspect variants, or [`source()`](std::error::Error::source)
/// to access the underlying cause.
///
/// # Example
/// ```
/// # use noteplay::{Error};
/// # const fn do_something() -> Result<(), Error> {
/// #     Err(Error::InvalidFloat(1.0))
/// # }
/// # fn main() {
/// match do_something() {
///     Ok(_) => {}
///     Err(Error::Io(e)) => { eprintln!("IO Error: {e}") }
///     Err(Error::ParseInt(e)) => { eprintln!("Error when parsing integers: {e}") }
///     Err(e) => { eprintln!("Other error: {e}") }
/// }
/// # }
/// ```
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// Failed to do IO operations.
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    /// Failed to parse an integer.
    #[error("Error while parsing an integer: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    /// Failed to parse a floating point number.
    #[error("Error while parsing a floating point number: {0}")]
    ParseFloat(#[from] std::num::ParseFloatError),
    /// The provided floating point number didn't meet the requirements.
    #[error("The provided floating point number didn't meet the requirements: {0}")]
    InvalidFloat(f32),
    /// Invalid data provided.
    #[error("Invalid data provided: {0}")]
    InvalidData(String),
    /// Failed to open device default sink.
    #[error("Failed to open device default sink: {0}")]
    OpenDefaultSink(#[from] rodio::DeviceSinkError),
}
