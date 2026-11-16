#![doc = include_str!("./README.md")]
pub use crate::Error;
pub use crate::core::{
    Frequency, NotePlayer, Semitones, Tempo, is_valid_note, note2freq, note2semi, semi2freq,
};
pub use crate::midi::{
    AUTHOR_LEN, COPYRIGHT_LEN, DEFAULT_META, Event, EventRaw, FixedString, FixedStringParseError,
    MAGIC, Midi, MidiMeta, MidiPlayer, MidiRaw, Note, SONG_NAME_LEN, VERSION, deserialize, event,
    event_const, get_metadata, midi, midi_const, play_qmid_file, repeat, serialize,
};
pub use crate::soundfont::Timbre;

#[cfg(feature = "songs")]
pub use crate::songs::*;

#[cfg(feature = "sf2")]
pub use crate::builtin_sf2::TIMGM6MB;
