//! Main definition of the structure.

use crate::midi::fixed_string::FixedString;
use crate::{
    core::types::{Frequency, Semitones, Tempo},
    soundfont::types::Timbre,
};

/// Runtime note: (frequency Hz, duration beat, Timbre, velocity).
pub type Note = (Frequency, f32, Timbre, i32);

/// Const-static raw note: (semi offset, duration beat, Timbre, velocity).
/// A4 = 0. Convert to [`Note`] at runtime with semi2freq.
#[derive(Debug, Clone, Copy)]
pub struct RawNote(pub Semitones, pub f32, pub Timbre, pub i32);

/// A storage of an event in a beat.
#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    /// Notes to press in a beat.
    pub notes: Vec<Note>,
    /// How many beats have passed since the previous event.
    pub delta_beat: f32,
}
impl Event {
    /// Create a new [`Event`] object.
    pub const fn new() -> Self {
        Self {
            notes: vec![],
            delta_beat: 0.0,
        }
    }
    /// Create a new [`Event`] object with given `delta_beat`.
    pub const fn with_delta_beat(delta_beat: f32) -> Self {
        Self {
            notes: vec![],
            delta_beat,
        }
    }
    /// Create a new [`Event`] object with given `notes`.
    pub const fn with_notes(notes: Vec<Note>) -> Self {
        Self {
            notes,
            delta_beat: 0.0,
        }
    }
}
impl Default for Event {
    fn default() -> Self {
        Self::new()
    }
}

/// Metadata of a [`Midi`] or [`MidiRaw`] structure.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MidiMeta {
    /// Name of the song.
    pub song_name: Option<FixedString<SONG_NAME_LEN>>,
    /// Copyright
    pub copyright: Option<FixedString<COPYRIGHT_LEN>>,
    /// Author
    pub author: Option<FixedString<AUTHOR_LEN>>,
}
impl MidiMeta {
    /// Create a new [`MidiMeta`] object.
    pub const fn new() -> Self {
        Self {
            song_name: None,
            copyright: None,
            author: None,
        }
    }
}
impl Default for MidiMeta {
    fn default() -> Self {
        Self::new()
    }
}

/// A custom Midi format (QMid).
#[derive(Debug, PartialEq, Clone)]
pub struct Midi {
    /// Beats per minute.
    pub tempo: Tempo,
    /// Events happend in a beat.
    pub events: Vec<Event>,
    /// Metadata.
    pub meta: MidiMeta,
}
impl Midi {
    /// Create a new [`Midi`] object.
    pub fn new() -> Self {
        Self {
            tempo: Tempo::new_unchecked(0.0),
            events: vec![],
            meta: MidiMeta::new(),
        }
    }
    /// Create a new [`Midi`] object with given `tempo`.
    pub const fn with_tempo(tempo: Tempo) -> Self {
        Self {
            tempo,
            events: vec![],
            meta: MidiMeta::new(),
        }
    }
    /// Create a new [`Midi`] object with given `events`.
    pub fn with_events(events: Vec<Event>) -> Self {
        Self {
            tempo: Tempo::new_unchecked(0.0),
            events,
            meta: MidiMeta::new(),
        }
    }
    /// Create a new [`Midi`] object with given `meta`.
    pub fn with_meta(meta: MidiMeta) -> Self {
        Self {
            tempo: Tempo::new_unchecked(0.0),
            events: vec![],
            meta,
        }
    }
    /// Transposition: take self, return a new sequence with all notes shifted by semitone offset.
    pub fn transposed(mut self, offset: i32) -> Self {
        let factor = (offset as f32 / 12.0).exp2();
        for e in &mut self.events {
            for note in &mut e.notes {
                note.0.set_freq_unchecked(note.0.get_freq() * factor);
            }
        }
        self
    }
}
impl Default for Midi {
    fn default() -> Self {
        Self::new()
    }
}

/// Static read-only [`Event`] sequence definition, compatible with `const`.
///
/// Use [`EventRaw::to_runtime`] to convert to a [`Event`] object.
#[derive(Debug, Clone, Copy)]
pub struct EventRaw<'a> {
    /// Raw notes (semi offset + duration)
    pub notes: &'a [RawNote],
    /// How many beats have passed since the previous event.
    pub delta_beat: f32,
}
impl<'a> EventRaw<'a> {
    /// Create a new [`EventRaw`] object.
    pub const fn new() -> Self {
        Self {
            notes: &[],
            delta_beat: 0.0,
        }
    }

    /// Convert to a runtime type [`Event`].
    /// Calculates frequency from semi offset using standard exp2.
    pub fn to_runtime(self) -> Event {
        let notes: Vec<Note> = self
            .notes
            .iter()
            .map(|&RawNote(semi, dur, timbre, velocity)| (semi2freq(semi), dur, timbre, velocity))
            .collect();
        Event {
            notes,
            delta_beat: self.delta_beat,
        }
    }
}
impl<'a> Default for EventRaw<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Static read-only [`Midi`] sequence definition, compatible with `const`.
///
/// Use [`MidiRaw::to_runtime`] to convert to a [`Midi`] object.
#[derive(Debug, Clone, Copy)]
pub struct MidiRaw<'a> {
    /// Beats per minute.
    pub tempo: f32,
    /// Raw events
    pub events: &'a [EventRaw<'a>],
    /// Metadata.
    pub meta: MidiMeta,
}
impl<'a> MidiRaw<'a> {
    /// Create a new [`MidiRaw`] object.
    pub const fn new() -> Self {
        Self {
            tempo: 0.0,
            events: &[],
            meta: MidiMeta::new(),
        }
    }

    /// Convert to a runtime type [`Midi`].
    pub fn to_runtime(self) -> Midi {
        Midi {
            tempo: Tempo::new_unchecked(self.tempo),
            events: self.events.iter().map(|er| er.to_runtime()).collect(),
            meta: self.meta,
        }
    }
}
impl<'a> Default for MidiRaw<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Convert semi offset (A4 = 0) to frequency Hz.
/// Runtime only, precise, uses standard library exp2.
pub fn semi2freq(semi: i32) -> Frequency {
    Frequency::new_unchecked(440.0 * (semi as f32 / 12.0).exp2())
}

/// Bytes.
pub type Bytes = Vec<u8>;

/// Version.
pub const VERSION: u16 = 0x0003;

/// Magic number.
///
/// ASCII: QMid
pub const MAGIC: &[u8] = &[0x51, 0x4D, 0x69, 0x64];

/// Length of the `copyright` field in the [`MidiMeta`].
pub const COPYRIGHT_LEN: usize = 96;
/// Length of the `author` field in the [`MidiMeta`].
pub const AUTHOR_LEN: usize = 32;
/// Length of the `song_name` field in the [`MidiMeta`].
pub const SONG_NAME_LEN: usize = 48;
/// Length of the QMid header (Bytes).
pub const HEADER_LEN: usize = 718;

/// Default metadata for this crate.
#[cfg(feature = "songs")]
pub const DEFAULT_META: MidiMeta = MidiMeta {
    song_name: None,
    copyright: Some(FixedString::<COPYRIGHT_LEN>::from_str_const(
        "Copyright (c) 2026 FishgameStudio",
    )),
    author: Some(FixedString::<AUTHOR_LEN>::from_str_const("Nicola Grey")),
};
