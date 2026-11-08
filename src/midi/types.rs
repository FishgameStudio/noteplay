//! Main definition of the structure.

/// Runtime note: (frequency Hz, duration beat).
/// This is an alias of two [`f32`] types.
pub type Note = (f32, f32);

/// Const-static raw note: (semi offset, duration beat).
/// A4 = 0. Convert to [`Note`] at runtime with semi2freq.
#[derive(Debug, Clone, Copy)]
pub struct RawNote(pub i32, pub f32);

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
    /// Transposition: take self, return a new sequence with all notes shifted by semitone offset.
    pub fn transposed(mut self, offset: i32) -> Self {
        let factor = (offset as f32 / 12.0).exp2();
        for e in &mut self.events {
            for note in &mut e.notes {
                note.0 *= factor;
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
            .map(|&RawNote(semi, dur)| (semi2freq(semi), dur))
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
            events: self.events.iter().map(|er| er.to_runtime()).collect(),
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
pub fn semi2freq(semi: i32) -> f32 {
    440.0 * (semi as f32 / 12.0).exp2()
}

/// Bytes.
pub type Bytes = Vec<u8>;

/// Version.
pub const VERSION: u16 = 0x0001;

/// Magic number.
///
/// ASCII: QMid
pub const MAGIC: &[u8] = &[0x51, 0x4D, 0x69, 0x64];
