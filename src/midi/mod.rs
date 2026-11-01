//! A simple parser for custom MIDI files.

pub(crate) mod deserializer;
pub(crate) mod player;
pub(crate) mod serializer;
pub(crate) mod types;
pub use deserializer::deserialize;
pub use player::MidiPlayer;
pub use serializer::serialize;
pub use types::{Event, Midi, Note};

#[cfg(test)]
mod tests {
    use super::deserializer::deserialize;
    use super::serializer::serialize;
    use super::types::Midi;

    fn midi_eq(a: &Midi, b: &Midi) -> bool {
        const EPS: f32 = 1e-5;
        if (a.tempo - b.tempo).abs() > EPS {
            return false;
        }
        if a.events.len() != b.events.len() {
            return false;
        }
        for (ea, eb) in a.events.iter().zip(b.events.iter()) {
            if (ea.delta_beat - eb.delta_beat).abs() > EPS {
                return false;
            }
            if ea.notes.len() != eb.notes.len() {
                return false;
            }
            for ((fa, da), (fb, db)) in ea.notes.iter().zip(eb.notes.iter()) {
                if (fa - fb).abs() > EPS || (da - db).abs() > EPS {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_serde() {
        assert!(midi_eq(
            &Midi::new(),
            &deserialize(serialize(Midi::new())).unwrap()
        ));
    }
}
