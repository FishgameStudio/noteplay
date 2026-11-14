//! The core playback module.

pub(crate) mod note_parser;
pub(crate) mod note_player;
pub(crate) mod types;
pub use note_parser::{note2freq, note2semi, semi2freq};
pub use note_player::NotePlayer;
pub use types::{Frequency, Semitones, Tempo};
