//! Main definition of the structure.

/// The one note.
/// This is an alias of two [`f32`] types.
/// The first number is the frequency,
/// and the second one is the duration (beat).
pub type Note = (f32, f32);

/// A storage of an event in a tempo.
#[derive(Debug, PartialEq, Clone)]
pub struct Event {
    /// Notes to press in a tempo.
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

/// A custom Midi format.
#[derive(Debug, PartialEq, Clone)]
pub struct Midi {
    /// Beats per minute.
    pub tempo: f32,
    /// Events happend in a beat.
    pub events: Vec<Event>,
}
impl Midi {
    /// Create a new [`Midi`] object.
    pub const fn new() -> Self {
        Self {
            tempo: 0.0,
            events: vec![],
        }
    }
    /// Create a new [`Midi`] object with given `tempo`.
    pub const fn with_tempo(tempo: f32) -> Self {
        Self {
            tempo,
            events: vec![],
        }
    }
    /// Create a new [`Midi`] object with given `events`.
    pub const fn with_events(events: Vec<Event>) -> Self {
        Self { tempo: 0.0, events }
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
    /// Notes to press in a tempo.
    pub notes: &'a [Note],
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
    pub fn to_runtime(self) -> Event {
        Event {
            notes: self.notes.to_vec(),
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
    /// Events happend in a beat.
    pub events: &'a [EventRaw<'a>],
}
impl<'a> MidiRaw<'a> {
    /// Create a new [`MidiRaw`] object.
    pub const fn new() -> Self {
        Self {
            tempo: 0.0,
            events: &[],
        }
    }
    /// Convert to a runtime type [`Midi`].
    pub fn to_runtime(self) -> Midi {
        Midi {
            tempo: self.tempo,
            events: {
                let mut res: Vec<Event> = vec![];
                for event in self.events {
                    res.push(event.to_runtime());
                }
                res
            },
        }
    }
}
impl<'a> Default for MidiRaw<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Bytes.
pub type Bytes = Vec<u8>;

/// Version.
pub const VERSION: u16 = 0x0001;

/// Magic number.
///
/// ASCII: QMid
pub const MAGIC: &[u8] = &[0x51, 0x4D, 0x69, 0x64];
