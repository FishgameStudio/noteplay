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
        const __CHECK: &[&str] = &[ $( $name ),* ];
        let _ = __CHECK;

        $crate::midi::Event {
            delta_beat: $delta,
            notes: vec![$(
                ($crate::core::note2freq($name),
                $dur,$crate::soundfont::Timbre::BrightAcousticPiano, 100)
            ),*],
        }
    }};
    ($delta:expr,[$(($name:literal, $dur:expr, $timbre:ident)),*$(,)?]) => {{
        const __CHECK: &[&str] = &[ $( $name ),* ];
        let _ = __CHECK;

        $crate::midi::Event {
            delta_beat: $delta,
            notes: vec![$(
                ($crate::core::note2freq($name),
                $dur,$crate::soundfont::Timbre::$timbre, 100)
            ),*],
        }
    }};
    ($delta:expr,[$(($name:literal, $dur:expr, $timbre:ident, $velocity:expr)),*$(,)?]) => {{
        const __CHECK: &[&str] = &[ $( $name ),* ];
        let _ = __CHECK;

        $crate::midi::Event {
            delta_beat: $delta,
            notes: vec![$(
                ($crate::core::note2freq($name),
                $dur,$crate::soundfont::Timbre::$timbre, $velocity)
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
/// let with_events = midi!("My Song", 60.0, event!(), event!());
/// }
/// ```
/// See also: macro [`event`]
#[macro_export]
macro_rules! midi {
    () => {{ $crate::midi::Midi::new() }};
    ($tempo:expr) => {{
        $crate::midi::Midi::with_tempo($crate::core::Tempo::new_unchecked($tempo))
    }};
    ($song_name:literal, $tempo:expr,$($event:expr),*$(,)?) => {{
        const _: &str = $song_name;

        $crate::midi::Midi {
            tempo: $crate::core::Tempo::new_unchecked($tempo),
            events: vec![$(
                $event
            ),*],
            meta: $crate::midi::MidiMeta {
                song_name: Some($crate::midi::FixedString::<{$crate::midi::SONG_NAME_LEN}>::from_str_const($song_name)),
                ..$crate::midi::DEFAULT_META
            }
        }
    }};
    ($song_name:literal, $copyright:literal, $author:literal, $tempo:expr,$($event:expr),*$(,)?) => {
        const _a: &str = $song_name;
        const _b: &str = $copyright;
        const _c: &str = $author;

        $crate::midi::Midi {
            tempo: $crate::core::Tempo::new_unchecked($tempo),
            events: vec![$(
                $event
            ),*],
            meta: $crate::midi::MidiMeta {
                song_name: Some($crate::midi::FixedString::<{$crate::midi::SONG_NAME_LEN}>::from_str_const($song_name)),
                author: Some($crate::midi::FixedString::<{$crate::midi::AUTHOR_LEN}>::from_str_const($author))
                copyright: Some($crate::midi::FixedString::<{$crate::midi::COPYRIGHT_LEN}>::from_str_const($copyright))
            }
        }
    }
}

/// Create a new [`EventRaw`](crate::midi::EventRaw) object with the [`event_const`] macro.
///
/// You can **certainly call** this macro inside a const block.
#[macro_export]
macro_rules! event_const {
    () => {{ $crate::midi::EventRaw::new() }};
    ($delta:expr,[$(($name:literal, $dur:expr)),*$(,)?]) => {{
        const __CHECK: &[&str] = &[ $( $name ),* ];
        let _ = __CHECK;

        $crate::midi::EventRaw {
            delta_beat: $delta,
            notes: &[$(
                $crate::midi::types::RawNote($crate::core::note2semi($name), $dur, $crate::soundfont::Timbre::BrightAcousticPiano, 100)
            ),*],
        }
    }};
    ($delta:expr,[$(($name:literal, $dur:expr, $timbre:ident)),*$(,)?]) => {{
        const __CHECK: &[&str] = &[ $( $name ),* ];
        let _ = __CHECK;

        $crate::midi::EventRaw {
            delta_beat: $delta,
            notes: &[$(
                $crate::midi::types::RawNote($crate::core::note2semi($name), $dur, $crate::soundfont::Timbre::$timbre, 100)
            ),*],
        }
    }};
    ($delta:expr,[$(($name:literal, $dur:expr, $timbre:ident, $velocity:expr)),*$(,)?]) => {{
        const __CHECK: &[&str] = &[ $( $name ),* ];
        let _ = __CHECK;

        $crate::midi::EventRaw {
            delta_beat: $delta,
            notes: &[$(
                $crate::midi::types::RawNote($crate::core::note2semi($name), $dur, $crate::soundfont::Timbre::$timbre, $velocity)
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
    ($song_name:literal, $tempo:expr,$($event:expr),*$(,)?) => {{
        const _: &str = $song_name;

        $crate::midi::MidiRaw {
            tempo: $tempo,
            events: &[$(
                $event
            ),*],
            meta: $crate::midi::MidiMeta {
                song_name: Some($crate::midi::FixedString::<{$crate::midi::SONG_NAME_LEN}>::from_str_const($song_name)),
                ..$crate::midi::DEFAULT_META
            }
        }
    }};
    ($song_name:literal, $copyright:literal, $author:literal, $tempo:expr,$($event:expr),*$(,)?) => {{
        const _: &str = $song_name;

        $crate::midi::MidiRaw {
            tempo: $tempo,
            events: &[$(
                $event
            ),*],
            meta: $crate::midi::MidiMeta {
                song_name: Some($crate::midi::FixedString::<{$crate::midi::SONG_NAME_LEN}>::from_str_const($song_name)),
                author: Some($crate::midi::FixedString::<{$crate::midi::AUTHOR_LEN}>::from_str_const($author))
                copyright: Some($crate::midi::FixedString::<{$crate::midi::COPYRIGHT_LEN}>::from_str_const($copyright))
            }
        }
    }};
}

/// Repeat an [`Event`](crate::midi::Event) object or an [`EventRaw`](crate::midi::EventRaw) object.
#[macro_export]
macro_rules! repeat {
    ($event:expr,$repetitions:expr) => {
        vec![$event; $repetitions]
    };
}

pub use {event, event_const, midi, midi_const, repeat};
