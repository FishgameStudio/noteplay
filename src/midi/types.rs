//! Main definition of the structure.

/// The one note.
/// This is an alias of two [`f32`] types.
/// The first number is the frequency,
/// and the second one is the duration (beat).
pub type Note = (f32, f32);

/// A storage of an event in a tempo.
#[derive(Debug, PartialEq)]
pub struct Event {
    /// Notes to press in a tempo
    pub notes: Vec<Note>,
    /// How many beats have passed since the previous event
    pub delta_beat: f32,
}
impl Event {
    /// Create a new [`Event`] object.
    pub fn new() -> Self {
        Self {
            notes: vec![],
            delta_beat: 0.0,
        }
    }
}
impl Default for Event {
    fn default() -> Self {
        Self::new()
    }
}

/// A custom midi format.
#[derive(Debug)]
pub struct Midi {
    /// Beats per minute.
    pub tempo: f32,
    /// Events happened in a tempo.
    pub events: Vec<Event>,
}
impl Midi {
    /// Create a new [`Midi`] object.
    pub fn new() -> Self {
        Self {
            tempo: 0.0,
            events: vec![],
        }
    }
}
impl Default for Midi {
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
