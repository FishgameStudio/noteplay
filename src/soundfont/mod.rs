#![doc = include_str!("./README.md")]

pub(crate) mod loader;
pub(crate) mod types;
//pub(crate) use loader::load_soundfont_from_bytes;
pub use types::Timbre;
