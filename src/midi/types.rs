//! Main definition of the structure.

use crate::Error;
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

/// A struct that stores fields in a normal event ([`Event::Normal`]).
#[derive(Debug, PartialEq, Clone)]
pub struct NormalEvent {
    /// Notes to press in a beat.
    pub notes: Vec<Note>,
    /// How many beats have passed since the previous event.
    pub delta_beat: f32,
}
impl NormalEvent {
    /// Create a new [`NormalEvent`] object.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::NormalEvent;
    /// let event = NormalEvent::new();
    /// assert!(event.notes.is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            notes: vec![],
            delta_beat: 0.0,
        }
    }
    /// Create a new [`NormalEvent`] object with given `delta_beat`.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::NormalEvent;
    /// let event = NormalEvent::with_delta_beat(0.5);
    /// assert_eq!(event.delta_beat, 0.5);
    /// ```
    #[must_use]
    pub const fn with_delta_beat(delta_beat: f32) -> Self {
        Self {
            notes: vec![],
            delta_beat,
        }
    }
    /// Create a new [`NormalEvent`] object with given `notes`.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Frequency;
    /// # use noteplay::midi::NormalEvent;
    /// # use noteplay::soundfont::Timbre;
    /// let event = NormalEvent::with_notes(vec![
    ///     (Frequency::new_unchecked(440.0), 1.0, Timbre::Default, 90)
    /// ]);
    /// assert_eq!(event.notes.len(), 1);
    /// ```
    #[must_use]
    pub const fn with_notes(notes: Vec<Note>) -> Self {
        Self {
            notes,
            delta_beat: 0.0,
        }
    }
    /// Returns an [`Event`] that is an [`Event::Normal`] enumeration.
    /// Consumes `self`.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::NormalEvent;
    /// let event = NormalEvent::new().to_event();
    /// assert!(event.get_normal().is_ok());
    /// ```
    #[must_use]
    pub fn to_event(self) -> Event {
        Event::Normal {
            notes: self.notes,
            delta_beat: self.delta_beat,
        }
    }
}
impl Default for NormalEvent {
    fn default() -> Self {
        Self::new()
    }
}

/// A storage of an event in a beat.
#[derive(Debug, PartialEq, Clone)]
#[repr(u8)]
pub enum Event {
    /// Normal event.
    // 0
    Normal {
        /// Notes to press in a beat.
        notes: Vec<Note>,
        /// How many beats have passed since the previous event.
        delta_beat: f32,
    },
    /// Tempo change.
    // 1
    TempoChange {
        /// The new tempo.
        tempo: Tempo,
    },
}
impl Event {
    /// Create a new [`Event`] object.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::Event;
    /// assert!(Event::new().get_normal().is_ok());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self::Normal {
            notes: vec![],
            delta_beat: 0.0,
        }
    }
    /// Create a new [`Event`] object with given `delta_beat`.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::Event;
    /// let event = Event::with_delta_beat(0.5).get_normal().unwrap();
    /// assert_eq!(event.delta_beat, 0.5);
    /// ```
    #[must_use]
    pub const fn with_delta_beat(delta_beat: f32) -> Self {
        Self::Normal {
            notes: vec![],
            delta_beat,
        }
    }
    /// Create a new [`Event`] object with given `notes`.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Frequency;
    /// # use noteplay::midi::Event;
    /// # use noteplay::soundfont::Timbre;
    /// let event = Event::with_notes(vec![
    ///     (Frequency::new_unchecked(440.0), 1.0, Timbre::Default, 90)
    /// ]);
    /// assert_eq!(event.get_normal().unwrap().notes.len(), 1);
    /// ```
    #[must_use]
    pub const fn with_notes(notes: Vec<Note>) -> Self {
        Self::Normal {
            notes,
            delta_beat: 0.0,
        }
    }
    /// Returns a [`NormalEvent`] (aka the [`Event::Normal`] enumeration) if it is,
    /// or returns an [`Err`] value if not.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::Event;
    /// assert!(Event::new().get_normal().is_ok());
    /// ```
    pub fn get_normal(&self) -> Result<NormalEvent, Error> {
        if let Self::Normal { notes, delta_beat } = self {
            Ok(NormalEvent {
                notes: notes.clone(),
                delta_beat: *delta_beat,
            })
        } else {
            Err(Error::NotNormalEvent)
        }
    }
    /// Returns a [`NormalEvent`] (aka the [`Event::Normal`] enumeration) if it is.
    /// # Panics
    /// If `self` is not [`Event::Normal`].
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::Event;
    /// let normal = Event::new().get_normal_unchecked();
    /// assert!(normal.notes.is_empty());
    /// ```
    #[must_use]
    pub fn get_normal_unchecked(&self) -> NormalEvent {
        self.get_normal().unwrap()
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
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::MidiMeta;
    /// let meta = MidiMeta::new();
    /// assert!(meta.song_name.is_none());
    /// ```
    #[must_use]
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

/// A custom Midi format (`QMid`).
#[derive(Debug, PartialEq, Clone)]
pub struct Midi {
    /// Beats per minute.
    pub tempo: Tempo,
    /// Events happened in a beat.
    pub events: Vec<Event>,
    /// Metadata.
    pub meta: MidiMeta,
}
impl Midi {
    /// Create a new [`Midi`] object.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::Midi;
    /// assert!(Midi::new().events.is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            tempo: Tempo::new_unchecked(0.0),
            events: vec![],
            meta: MidiMeta::new(),
        }
    }
    /// Create a new [`Midi`] object with given `tempo`.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Tempo;
    /// # use noteplay::midi::Midi;
    /// let midi = Midi::with_tempo(Tempo::new_unchecked(120.0));
    /// assert_eq!(midi.tempo.get_tempo(), 120.0);
    /// ```
    #[must_use]
    pub const fn with_tempo(tempo: Tempo) -> Self {
        Self {
            tempo,
            events: vec![],
            meta: MidiMeta::new(),
        }
    }
    /// Create a new [`Midi`] object with given `events`.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::{Event, Midi};
    /// let midi = Midi::with_events(vec![Event::new()]);
    /// assert_eq!(midi.events.len(), 1);
    /// ```
    #[must_use]
    pub const fn with_events(events: Vec<Event>) -> Self {
        Self {
            tempo: Tempo::new_unchecked(0.0),
            events,
            meta: MidiMeta::new(),
        }
    }
    /// Create a new [`Midi`] object with given `meta`.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::{Midi, MidiMeta};
    /// let midi = Midi::with_meta(MidiMeta::new());
    /// assert!(midi.meta.song_name.is_none());
    /// ```
    #[must_use]
    pub const fn with_meta(meta: MidiMeta) -> Self {
        Self {
            tempo: Tempo::new_unchecked(0.0),
            events: vec![],
            meta,
        }
    }
    /// Transposition: take self, return a new sequence with all notes shifted by semitone offset.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Frequency;
    /// # use noteplay::midi::{Event, Midi};
    /// # use noteplay::soundfont::Timbre;
    /// let midi = Midi {
    ///     events: vec![Event::Normal {
    ///         notes: vec![(Frequency::new_unchecked(440.0), 1.0, Timbre::Default, 90)],
    ///         delta_beat: 0.0,
    ///     }],
    ///     ..Midi::new()
    /// };
    /// let raised = midi.transposed(12);
    /// let note = raised.events[0].get_normal_unchecked().notes[0].0.get_freq();
    /// assert!((note - 880.0).abs() < 0.001);
    /// ```
    #[must_use]
    pub fn transposed(mut self, offset: i32) -> Self {
        let factor = (offset as f32 / 12.0).exp2();
        for e in &mut self.events {
            if let Event::Normal {
                notes,
                delta_beat: _,
            } = e
            {
                for note in notes {
                    note.0.set_freq_unchecked(note.0.get_freq() * factor);
                }
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

/// A struct that stores fields in the [`EventRaw::Normal`].
#[derive(Debug, Clone, Copy)]
pub struct NormalEventRaw<'a> {
    /// Notes to press in a beat.
    pub notes: &'a [RawNote],
    /// How many beats have passed since the previous event.
    pub delta_beat: f32,
}
impl<'a> NormalEventRaw<'a> {
    /// Create a new [`EventRaw`] object.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::NormalEventRaw;
    /// assert!(NormalEventRaw::new().notes.is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            notes: &[],
            delta_beat: 0.0,
        }
    }
    /// Convert to a runtime type [`NormalEvent`].
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::NormalEventRaw;
    /// assert!(NormalEventRaw::new().to_runtime().notes.is_empty());
    /// ```
    #[must_use]
    pub fn to_runtime(self) -> NormalEvent {
        let notes: Vec<Note> = self
            .notes
            .iter()
            .map(|&RawNote(semi, dur, timbre, velocity)| (semi2freq(semi), dur, timbre, velocity))
            .collect();
        NormalEvent {
            notes,
            delta_beat: self.delta_beat,
        }
    }

    /// Returns an [`EventRaw`] that is an [`EventRaw::Normal`] enumeration.
    /// Consumes `self`.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::NormalEventRaw;
    /// assert!(NormalEventRaw::new().to_event().get_normal().is_ok());
    /// ```
    #[must_use]
    pub const fn to_event(self) -> EventRaw<'a> {
        EventRaw::Normal {
            notes: self.notes,
            delta_beat: self.delta_beat,
        }
    }
}
impl<'a> Default for NormalEventRaw<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Static read-only [`Event`] sequence definition, compatible with `const`.
///
/// Use [`EventRaw::to_runtime`] to convert to a [`Event`] object.
#[derive(Debug, Clone, Copy)]
pub enum EventRaw<'a> {
    /// Normal event.
    // 0
    Normal {
        /// Raw notes (semi offset + duration)
        notes: &'a [RawNote],
        /// How many beats have passed since the previous event.
        delta_beat: f32,
    },
    /// Tempo changes.
    // 1
    TempoChange {
        /// The new tempo.
        tempo: Tempo,
    },
}
impl<'a> EventRaw<'a> {
    /// Create a new [`EventRaw`] object.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::EventRaw;
    /// assert!(EventRaw::new().get_normal().is_ok());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self::Normal {
            notes: &[],
            delta_beat: 0.0,
        }
    }

    /// Returns a [`NormalEventRaw`] (aka the [`EventRaw::Normal`] enumeration) if it is,
    /// or returns an [`Err`] value if not.
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::EventRaw;
    /// assert!(EventRaw::new().get_normal().is_ok());
    /// ```
    pub const fn get_normal(&'a self) -> Result<NormalEventRaw<'a>, Error> {
        if let Self::Normal { notes, delta_beat } = self {
            Ok(NormalEventRaw {
                notes,
                delta_beat: *delta_beat,
            })
        } else {
            Err(Error::NotNormalEvent)
        }
    }
    /// Returns a [`NormalEventRaw`] (aka the [`EventRaw::Normal`] enumeration) if it is.
    /// # Panics
    /// If `self` is not [`EventRaw::Normal`].
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::EventRaw;
    /// assert!(EventRaw::new().get_normal_unchecked().notes.is_empty());
    /// ```
    #[must_use]
    pub fn get_normal_unchecked(&'a self) -> NormalEventRaw<'a> {
        self.get_normal().unwrap()
    }

    /// Convert to a runtime type [`Event`].
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::EventRaw;
    /// assert!(EventRaw::new().to_runtime().get_normal().is_ok());
    /// ```
    #[must_use]
    pub fn to_runtime(self) -> Event {
        if let Self::Normal { notes, delta_beat } = self {
            let notes: Vec<Note> = notes
                .iter()
                .map(|&RawNote(semi, dur, timbre, velocity)| {
                    (semi2freq(semi), dur, timbre, velocity)
                })
                .collect();
            Event::Normal { notes, delta_beat }
        } else if let Self::TempoChange { tempo } = self {
            Event::TempoChange { tempo }
        } else {
            unreachable!()
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
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::MidiRaw;
    /// assert!(MidiRaw::new().events.is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            tempo: 0.0,
            events: &[],
            meta: MidiMeta::new(),
        }
    }

    /// Convert to a runtime type [`Midi`].
    ///
    /// # Example
    /// ```
    /// # use noteplay::midi::MidiRaw;
    /// assert!(MidiRaw::new().to_runtime().events.is_empty());
    /// ```
    #[must_use]
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
///
/// # Example
/// ```
/// # use noteplay::prelude::semi2freq;
/// # fn main() {
/// assert_eq!(semi2freq(0).get_freq(), 440.0);
/// # }
/// ```
pub fn semi2freq(semi: i32) -> Frequency {
    Frequency::new_unchecked(440.0 * (semi as f32 / 12.0).exp2())
}

/// Bytes.
pub type Bytes = Vec<u8>;

/// Version.
pub const VERSION: u16 = 0x0004;

/// Magic number.
///
/// ASCII: `QMid`
pub const MAGIC: &[u8] = &[0x51, 0x4D, 0x69, 0x64];

/// Length of the `copyright` field in the [`MidiMeta`].
pub const COPYRIGHT_LEN: usize = 96;
/// Length of the `author` field in the [`MidiMeta`].
pub const AUTHOR_LEN: usize = 32;
/// Length of the `song_name` field in the [`MidiMeta`].
pub const SONG_NAME_LEN: usize = 48;
/// Length of the `QMid` header (Bytes).
pub const HEADER_LEN: usize = 718;

/// Default metadata for this crate.
pub const DEFAULT_META: MidiMeta = MidiMeta {
    song_name: None,
    copyright: Some(FixedString::<COPYRIGHT_LEN>::from_str_const(
        "Copyright (c) 2026 FishgameStudio",
    )),
    author: Some(FixedString::<AUTHOR_LEN>::from_str_const("Nicola Grey")),
};
