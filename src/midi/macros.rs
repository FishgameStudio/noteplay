//! Macro definitions.

/// Create a new [`Event`](crate::midi::Event) object with the [`event`] macro.
///
/// You *cannot* call this macro inside a const block.
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
        let _ = __CHECK; // dismiss unused warn

        $crate::midi::Event::Normal {
            delta_beat: $delta,
            notes: vec![$(
                ($crate::core::note2freq($name),
                $dur,$crate::soundfont::Timbre::BrightAcousticPiano, 100)
            ),*],
        }
    }};
    ($delta:expr,[$(($name:literal, $dur:expr, $timbre:ident)),*$(,)?]) => {{
        const __CHECK: &[&str] = &[ $( $name ),* ];
        let _ = __CHECK; // dismiss unused warn

        $crate::midi::Event::Normal {
            delta_beat: $delta,
            notes: vec![$(
                ($crate::core::note2freq($name),
                $dur,$crate::soundfont::Timbre::$timbre, 100)
            ),*],
        }
    }};
    ($delta:expr,[$(($name:literal, $dur:expr, $timbre:ident, $velocity:expr)),*$(,)?]) => {{
        const __CHECK: &[&str] = &[ $( $name ),* ];
        let _ = __CHECK; // dismiss unused warn

        $crate::midi::Event::Normal {
            delta_beat: $delta,
            notes: vec![$(
                ($crate::core::note2freq($name),
                $dur,$crate::soundfont::Timbre::$timbre, $velocity)
            ),*],
        }
    }};
}
/// Update the tempo of a song with the [`tempo_change`] macro.
///
/// # Example
/// ```
/// # use noteplay::midi::{tempo_change, midi};
/// # fn main() {
/// let midi = midi!(
///     "Foo Song", 140.0,
///     tempo_change!(150.0), // Faster
///     tempo_change!(80.0),  // Slower
/// );
/// # }
/// ```
#[macro_export]
macro_rules! tempo_change {
    ($tempo:expr) => {{
        const _: f32 = $tempo;
        $crate::midi::Event::TempoChange {
            tempo: $crate::core::Tempo::new_unchecked($tempo),
        }
    }};
}

/// Update the tempo of a song with the [`tempo_change_const`] macro.
///
/// # Example
/// ```
/// # use noteplay::midi::{tempo_change_const, midi_const};
/// # fn main() {
/// let midi = midi_const!(
///     "Foo Song", 140.0,
///     tempo_change_const!(150.0), // Faster
///     tempo_change_const!(80.0),  // Slower
/// );
/// # }
/// ```
#[macro_export]
macro_rules! tempo_change_const {
    ($tempo:expr) => {{
        const _: f32 = $tempo;
        $crate::midi::EventRaw::TempoChange {
            tempo: $crate::core::Tempo::new_unchecked($tempo),
        }
    }};
}

/// Create a new [`Midi`](crate::midi::Midi) object with the [`midi`] macro.
///
/// You *cannot* call this macro inside a const block.
/// # Example
/// ```
/// # use noteplay::midi::{midi, event};
/// # fn main() {
/// let empty_midi = midi!();
/// let with_tempo = midi!(60.0); // 60 beats per minute
/// let with_events = midi!("My Song", 60.0, event!(), event!());
/// # }
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
    (Name = $song_name:literal, Copyright = $copyright:expr, Author = $author:expr, $tempo:expr,$($event:expr),*$(,)?) => {{
        const _: &str = $song_name;
        const _: &str = $copyright;
        const _: &str = $author;

        $crate::midi::Midi {
            tempo: $crate::core::Tempo::new_unchecked($tempo),
            events: vec![$(
                $event
            ),*],
            meta: $crate::midi::MidiMeta {
                song_name: Some($crate::midi::FixedString::<{$crate::midi::SONG_NAME_LEN}>::from_str_const($song_name)),
                author: Some($crate::midi::FixedString::<{$crate::midi::AUTHOR_LEN}>::from_str_const($author)),
                copyright: Some($crate::midi::FixedString::<{$crate::midi::COPYRIGHT_LEN}>::from_str_const($copyright)),
            }
        }
    }}
}

/// Create a new [`EventRaw`](crate::midi::EventRaw) object with the [`event_const`] macro.
///
/// You can **certainly call** this macro inside a const block.
///
/// # Example
/// ```
/// # use noteplay::midi::event_const;
/// # fn main() {
/// let empty_event = event_const!();
/// let with_notes = event_const!(1.0, [
///     ("C4", 1.0), ("E4", 1.0), ("G4", 1.0) // Do Mi So
/// ]);
/// # }
/// ```
#[macro_export]
macro_rules! event_const {
    () => {{ $crate::midi::EventRaw::new() }};
    ($delta:expr,[$(($name:literal, $dur:expr)),*$(,)?]) => {{
        const __CHECK: &[&str] = &[ $( $name ),* ];
        let _ = __CHECK; // dismiss unused warn
        const __NOTES: &[$crate::midi::RawNote] = &[$(
            $crate::midi::RawNote($crate::core::note2semi($name), $dur, $crate::soundfont::Timbre::BrightAcousticPiano, 100)
        ),*];

        $crate::midi::EventRaw::Normal {
            delta_beat: $delta,
            notes: __NOTES,
        }
    }};
    ($delta:expr,[$(($name:literal, $dur:expr, $timbre:ident)),*$(,)?]) => {{
        const __CHECK: &[&str] = &[ $( $name ),* ];
        let _ = __CHECK; // dismiss unused warn
        const __NOTES: &[$crate::midi::RawNote] = &[$(
            $crate::midi::RawNote($crate::core::note2semi($name), $dur, $crate::soundfont::Timbre::$timbre, 100)
        ),*];

        $crate::midi::EventRaw::Normal {
            delta_beat: $delta,
            notes: __NOTES,
        }
    }};
    ($delta:expr,[$(($name:literal, $dur:expr, $timbre:ident, $velocity:expr)),*$(,)?]) => {{
        const __CHECK: &[&str] = &[ $( $name ),* ];
        let _ = __CHECK; // dismiss unused warn
        const __NOTES: &[$crate::midi::RawNote] = &[$(
            $crate::midi::RawNote($crate::core::note2semi($name), $dur, $crate::soundfont::Timbre::$timbre, $velocity)
        ),*];

        $crate::midi::EventRaw::Normal {
            delta_beat: $delta,
            notes: __NOTES,
        }
    }};
}

/// Create a new [`MidiRaw`](crate::midi::MidiRaw) object with the [`midi_const`] macro.
///
/// You can **certainly call** this macro inside a const block.
///
/// # Example
/// ```
/// # use noteplay::midi::{midi_const, event_const};
/// # fn main() {
/// let empty_midi = midi_const!();
/// let with_events = midi_const!("My Song", 60.0, event_const!(), event_const!());
/// # }
/// ```
#[macro_export]
macro_rules! midi_const {
    () => {{ $crate::midi::MidiRaw::new() }};
    ($song_name:literal, $tempo:literal, $($event:expr),*$(,)?) => {{
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
    (Name = $song_name:literal, Copyright = $copyright:literal, Author = $author:literal, $tempo:literal, $($event:expr),*$(,)?) => {{
        const _: &str = $song_name;
        const _1: &str = $copyright;
        const _2: &str = $author;

        $crate::midi::MidiRaw {
            tempo: $tempo,
            events: &[$(
                $event
            ),*],
            meta: $crate::midi::MidiMeta {
                song_name: Some($crate::midi::FixedString::<{$crate::midi::SONG_NAME_LEN}>::from_str_const($song_name)),
                author: Some($crate::midi::FixedString::<{$crate::midi::AUTHOR_LEN}>::from_str_const($author)),
                copyright: Some($crate::midi::FixedString::<{$crate::midi::COPYRIGHT_LEN}>::from_str_const($copyright)),
            }
        }
    }};
}

/// Repeat an [`Event`](crate::midi::Event) object or an [`EventRaw`](crate::midi::EventRaw) object.
/// # Example
/// ```
/// # use noteplay::midi::{event, repeat};
/// # fn main() {
/// assert_eq!(repeat!(event!(), 3), vec![event!(), event!(), event!()]);
/// # }
/// ```
#[macro_export]
macro_rules! repeat {
    ($event:expr,$repetitions:expr) => {{
        const _: $crate::midi::Event = $event;
        vec![$event; $repetitions]
    }};
}

pub use {event, event_const, midi, midi_const, repeat, tempo_change, tempo_change_const};
