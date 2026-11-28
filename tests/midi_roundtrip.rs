use noteplay::prelude::*;

mod common;
use common::assert_midi_eq;

#[test]
fn midi_serialize_deserialize_roundtrip() {
    // Arrange
    let midi = Midi {
        tempo: Tempo::new_unchecked(123.5),
        meta: MidiMeta {
            song_name: Some(FixedString::from_str_const("Demo Song")),
            copyright: None,
            author: None,
        },
        events: vec![Event::Normal {
            delta_beat: 0.0,
            notes: vec![(Frequency::new_unchecked(440.0), 0.5, Timbre::Default, 90)],
        }],
    };

    // Act
    let bin = serialize(&midi);
    let restored = deserialize(bin).unwrap();

    // Assert
    assert_midi_eq(&midi, &restored);
}
