//! The core playback module.
//!
//!

pub(crate) mod note_parser;
pub(crate) mod note_player;
pub use note_parser::note2freq;
pub use note_player::NotePlayer;
