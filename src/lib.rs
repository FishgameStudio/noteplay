#![deny(
    missing_docs,
    non_snake_case,
    non_ascii_idents,
    non_camel_case_types,
    non_upper_case_globals,
    unsafe_op_in_unsafe_fn,
    unused_must_use,
    clippy::correctness
)]
#![warn(
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::float_cmp,
    clippy::if_not_else,
    clippy::needless_borrow,
    clippy::redundant_closure_for_method_calls,
    clippy::unused_self,
    clippy::useless_let_if_seq,
    clippy::match_wildcard_for_single_variants,
    clippy::same_name_method,
    clippy::self_named_module_files,
    rustdoc::broken_intra_doc_links,
    rustdoc::invalid_html_tags,
    clippy::all,
    clippy::nursery,
    rustdoc::missing_crate_level_docs,
    explicit_outlives_requirements,
    clippy::dbg_macro,
    clippy::todo,
    clippy::unimplemented,
    clippy::semicolon_if_nothing_returned,
    clippy::doc_markdown,
    clippy::must_use_candidate,
    clippy::missing_const_for_fn
)]
#![allow(clippy::redundant_pub_crate)]

//! # Noteplay: Note & Midi Player for Rust
//!
//! **Noteplay** is a Rust crate for turning
//! musical notes and simple arrangements into
//! sound. It can play individual notes through
//! the system's default audio output, schedule
//! notes together into chords and sequences, and
//! play short songs at a chosen tempo. It's
//! intended as a straightforward starting point
//! for adding musical playback to Rust projects -
//! not as a full digital audio workstation.
//!
//! View the documentation [here](https://docs.rs/noteplay/latest/noteplay).

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;

#[cfg(feature = "sf2")]
pub mod builtin_sf2;
#[cfg(feature = "songs")]
pub mod songs;

pub mod core;
pub mod midi;
pub mod prelude;
pub mod soundfont;

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
    /// Bad root octave.
    #[error("Bad root octave: {0}")]
    BadRootOctave(i32),
    /// Invalid chord format.
    #[error("Invalid chord format: {0}")]
    InvalidChord(String),
    /// Unknown chord suffix.
    #[error("Unknown chord suffix: {0}")]
    UnknownChordSuffix(String),
    /// Failed to create a synthesizer.
    #[error("Failed to create a synthesizer: {0}")]
    SynthesizerCreation(#[from] rustysynth::SynthesizerError),
    /// Failed to create a new sound font.
    #[error("Failed to create a new sound font: {0}")]
    SoundFontCreation(#[from] rustysynth::SoundFontError),
    /// Try to apply default timbre while soundfont processing.
    #[error("Try to apply default timbre while soundfont processing")]
    TryToApplyDefaultTimbre,
    /// Unknown timbre identifier number while deserializing.
    #[error("Unknown timbre identifier number while deserializing: {0}")]
    UnknownTimbreNum(i32),
    /// `FixedString` parse error
    #[error("FixedString parse error: {0}")]
    FixedStringParseError(#[from] crate::midi::fixed_string::FixedStringParseError),
    /// Failed to merge two midis.
    #[error("Failed to merge two midis: {0}")]
    MergeMidi(String),
    /// Assertion that not a normal event failed.
    #[error("Assertion that not a normal event failed")]
    NotNormalEvent,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_try_to_apply_default_timbre_display() {
        let e = Error::TryToApplyDefaultTimbre;
        assert_eq!(
            e.to_string(),
            "Try to apply default timbre while soundfont processing"
        );
    }
}
