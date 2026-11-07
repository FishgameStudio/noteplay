//! Macro definitions.

/// Create a new [`Event`](crate::midi::Event) object with the [`event`] macro.
///
/// You cannot call this macro inside a const block.
/// # Example
/// ```
/// # use noteplay::midi::event;
/// # fn main() {
/// let empty_event = event!();
/// let with_delta_beats = event!(1.0); // one beat
/// let with_notes = event!(1.0, [
///     ("C4", 1.0), ("E4", 1.0), ("G4", 1.0) // Do Mi So
/// ]);
/// # }
/// ```
#[macro_export]
macro_rules! event {
    () => {{ $crate::midi::Event::new() }};
    ($delta:expr) => {{ $crate::midi::Event::with_delta_beat($delta) }};
    ($delta:expr,[$(($name:literal, $dur:expr)),*$(,)?]) => {{
        $crate::midi::Event {
            delta_beat: $delta,
            notes: vec![$(
                ($crate::core::note2freq($name),
                $dur)
            ),*],
        }
    }};
}

/// Create a new [`Midi`](crate::midi::Midi) object with the [`midi`] macro.
///
/// You cannot call this macro inside a const block.
/// # Example
/// ```
/// use noteplay::midi::{midi, event};
/// fn main() {
/// let empty_midi = midi!();
/// let with_tempo = midi!(60.0); // 60 beats per minute
/// let with_events = midi!(60.0, event!(), event!());
/// }
/// ```
/// See also: macro [`event`]
#[macro_export]
macro_rules! midi {
    () => {{ $crate::midi::Midi::new() }};
    ($tempo:expr) => {{
        $crate::midi::Midi::with_tempo($tempo)
    }};
    ($tempo:expr,$($event:expr),*$(,)?) => {{
        $crate::midi::Midi {
            tempo: $tempo,
            events: vec![$(
                $event
            ),*]
        }
    }};
}

/// Create a new [`EventRaw`](crate::midi::EventRaw) object with the [`event_const`] macro.
///
/// You can **certainly call** this macro inside a const block.
#[macro_export]
macro_rules! event_const {
    () => {{ $crate::midi::EventRaw::new() }};
    ($delta:expr,[$(($name:literal, $dur:expr)),*$(,)?]) => {{
        $crate::midi::EventRaw {
            delta_beat: $delta,
            notes: &[$(
                ($crate::core::note2freq_const($name),
                $dur)
            ),*],
        }
    }};
}

/// Create a new [`MidiRaw`](crate::midi::MidiRaw) object with the [`midi_const`] macro.
///
/// You can **certainly call** this macro inside a const block.
#[macro_export]
macro_rules! midi_const {
    () => {{ $crate::midi::MidiRaw::new() }};
    ($tempo:expr,$($event:expr),*$(,)?) => {{
        $crate::midi::MidiRaw {
            tempo: $tempo,
            events: &[$(
                $event
            ),*]
        }
    }};
}

pub use {event, event_const, midi, midi_const};
