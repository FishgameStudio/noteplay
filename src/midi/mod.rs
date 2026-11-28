#![doc = include_str!("./README.md")]

pub(crate) mod deserializer;
pub(crate) mod fixed_string;
pub(crate) mod macros;
pub(crate) mod player;
pub(crate) mod serializer;
pub(crate) mod types;
pub(crate) mod utils;

pub use deserializer::deserialize;
pub use fixed_string::{FixedString, FixedStringParseError};
pub use macros::{event, event_const, midi, midi_const, repeat, tempo_change, tempo_change_const};
pub use player::MidiPlayer;
pub use serializer::serialize;
pub use types::{
    AUTHOR_LEN, COPYRIGHT_LEN, DEFAULT_META, Event, EventRaw, MAGIC, Midi, MidiMeta, MidiRaw,
    NormalEvent, NormalEventRaw, Note, RawNote, SONG_NAME_LEN, VERSION,
};
pub use utils::*;

#[cfg(test)]
mod tests {
    use crate::core::types::{Frequency, Tempo};
    use crate::soundfont::Timbre;

    use super::deserializer::deserialize;
    use super::serializer::serialize;
    use super::types::{Event, MAGIC, Midi, MidiMeta, VERSION};

    fn assert_midi_eq(expected: &Midi, actual: &Midi) {
        assert_eq!(
            expected.tempo.get_tempo().to_bits(),
            actual.tempo.get_tempo().to_bits()
        );
        assert_eq!(expected.events.len(), actual.events.len());
        for (expected_event, actual_event) in expected.events.iter().zip(&actual.events) {
            let Event::Normal {
                notes: e_notes,
                delta_beat: e_delta_beat,
            } = expected_event
            else {
                panic!()
            };
            let Event::Normal {
                notes: a_notes,
                delta_beat: a_delta_beat,
            } = actual_event
            else {
                panic!()
            };
            assert_eq!(e_delta_beat.to_bits(), a_delta_beat.to_bits());
            assert_eq!(e_notes.len(), a_notes.len());
            for (expected_note, actual_note) in e_notes.iter().zip(a_notes) {
                assert_eq!(
                    expected_note.0.get_freq().to_bits(),
                    actual_note.0.get_freq().to_bits()
                );
                assert_eq!(expected_note.1.to_bits(), actual_note.1.to_bits());
            }
        }
    }

    #[test]
    fn empty_midi_round_trips() {
        let expected = Midi::new();
        let actual = deserialize(serialize(&Midi::new())).unwrap();

        assert_midi_eq(&expected, &actual);
    }

    #[test]
    fn midi_with_mixed_events_round_trips_and_has_expected_binary_layout() {
        let midi = Midi {
            tempo: Tempo::new_unchecked(123.5),
            meta: MidiMeta {
                song_name: None,
                copyright: None,
                author: None,
            },
            events: vec![
                Event::Normal {
                    notes: vec![],
                    delta_beat: 0.0,
                },
                Event::Normal {
                    notes: vec![
                        (Frequency::new_unchecked(440.0), 0.25, Timbre::Default, 90),
                        (Frequency::new_unchecked(261.625), 1.5, Timbre::Default, 90),
                    ],
                    delta_beat: 0.5,
                },
                Event::Normal {
                    notes: vec![(Frequency::new_unchecked(880.0), 0.125, Timbre::Default, 90)],
                    delta_beat: 2.0,
                },
            ],
        };

        let bin = serialize(&midi);
        let mut expected_bin = Vec::new();

        // Header
        expected_bin.extend_from_slice(MAGIC);
        expected_bin.extend_from_slice(&VERSION.to_le_bytes());
        expected_bin.extend_from_slice(&123.5_f32.to_le_bytes());
        expected_bin.extend_from_slice(&3_u32.to_le_bytes());

        // FixedString<48> song_name, FixedString<96> copyright, FixedString<32> author, all zero
        expected_bin.extend_from_slice(&[0u8; 48 * 4]);
        expected_bin.extend_from_slice(&[0u8; 96 * 4]);
        expected_bin.extend_from_slice(&[0u8; 32 * 4]);

        // Event 0
        expected_bin.push(0); // Normal event type
        expected_bin.extend_from_slice(&0.0_f32.to_le_bytes());
        expected_bin.extend_from_slice(&0_u32.to_le_bytes());

        // Event 1
        expected_bin.push(0); // Normal event type
        expected_bin.extend_from_slice(&0.5_f32.to_le_bytes());
        expected_bin.extend_from_slice(&2_u32.to_le_bytes());
        // Note 1: freq(f32), dur(f32), timbre(i32), velocity(i32)
        expected_bin.extend_from_slice(&440.0_f32.to_le_bytes());
        expected_bin.extend_from_slice(&0.25_f32.to_le_bytes());
        expected_bin.extend_from_slice(&(Timbre::Default as i32).to_le_bytes());
        expected_bin.extend_from_slice(&90i32.to_le_bytes());

        // Note 2
        expected_bin.extend_from_slice(&261.625_f32.to_le_bytes());
        expected_bin.extend_from_slice(&1.5_f32.to_le_bytes());
        expected_bin.extend_from_slice(&(Timbre::Default as i32).to_le_bytes());
        expected_bin.extend_from_slice(&90i32.to_le_bytes());

        // Event 2
        expected_bin.push(0); // Normal event type
        expected_bin.extend_from_slice(&2.0_f32.to_le_bytes());
        expected_bin.extend_from_slice(&1_u32.to_le_bytes());
        // Note
        expected_bin.extend_from_slice(&880.0_f32.to_le_bytes());
        expected_bin.extend_from_slice(&0.125_f32.to_le_bytes());
        expected_bin.extend_from_slice(&(Timbre::Default as i32).to_le_bytes());
        expected_bin.extend_from_slice(&90i32.to_le_bytes());

        assert_eq!(bin, expected_bin);

        let actual = deserialize(bin).unwrap();
        assert_midi_eq(
            &Midi {
                tempo: Tempo::new_unchecked(123.5),
                meta: MidiMeta {
                    song_name: None,
                    copyright: None,
                    author: None,
                },
                events: vec![
                    Event::Normal {
                        notes: vec![],
                        delta_beat: 0.0,
                    },
                    Event::Normal {
                        notes: vec![
                            (Frequency::new_unchecked(440.0), 0.25, Timbre::Default, 90),
                            (Frequency::new_unchecked(261.625), 1.5, Timbre::Default, 90),
                        ],
                        delta_beat: 0.5,
                    },
                    Event::Normal {
                        notes: vec![(Frequency::new_unchecked(880.0), 0.125, Timbre::Default, 90)],
                        delta_beat: 2.0,
                    },
                ],
            },
            &actual,
        );
    }

    #[test]
    fn deserializer_rejects_invalid_or_truncated_data() {
        assert!(deserialize(vec![0; 13]).is_err());

        let mut invalid_magic = Vec::new();
        invalid_magic.extend_from_slice(b"Nope");
        invalid_magic.extend_from_slice(&VERSION.to_le_bytes());
        invalid_magic.extend_from_slice(&120.0_f32.to_le_bytes());
        invalid_magic.extend_from_slice(&0_u32.to_le_bytes());
        assert!(deserialize(invalid_magic).is_err());

        let mut invalid_version = Vec::new();
        invalid_version.extend_from_slice(MAGIC);
        invalid_version.extend_from_slice(&(VERSION + 1).to_le_bytes());
        invalid_version.extend_from_slice(&120.0_f32.to_le_bytes());
        invalid_version.extend_from_slice(&0_u32.to_le_bytes());
        assert!(deserialize(invalid_version).is_err());

        let mut negative_tempo = Vec::new();
        negative_tempo.extend_from_slice(MAGIC);
        negative_tempo.extend_from_slice(&VERSION.to_le_bytes());
        negative_tempo.extend_from_slice(&(-1.0_f32).to_le_bytes());
        negative_tempo.extend_from_slice(&0_u32.to_le_bytes());
        assert!(deserialize(negative_tempo).is_err());

        let mut truncated_event = Vec::new();
        truncated_event.extend_from_slice(MAGIC);
        truncated_event.extend_from_slice(&VERSION.to_le_bytes());
        truncated_event.extend_from_slice(&120.0_f32.to_le_bytes());
        truncated_event.extend_from_slice(&1_u32.to_le_bytes());
        assert!(deserialize(truncated_event).is_err());

        let mut truncated_note = Vec::new();
        truncated_note.extend_from_slice(MAGIC);
        truncated_note.extend_from_slice(&VERSION.to_le_bytes());
        truncated_note.extend_from_slice(&120.0_f32.to_le_bytes());
        truncated_note.extend_from_slice(&1_u32.to_le_bytes());
        truncated_note.extend_from_slice(&0.0_f32.to_le_bytes());
        truncated_note.extend_from_slice(&1_u32.to_le_bytes());
        truncated_note.extend_from_slice(&440.0_f32.to_le_bytes());
        assert!(deserialize(truncated_note).is_err());
    }
}
