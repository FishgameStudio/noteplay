//! Player for deserialized structure.
use crate::core::note_player::NotePlayer;
use crate::midi::types::{Event, Midi};

const NOTE_DURATION_FACTOR: f32 = 0.95;

fn articulated_duration_beats(duration_beats: f32) -> f32 {
    duration_beats * NOTE_DURATION_FACTOR
}

fn remaining_playback_beats(events: &[Event]) -> f32 {
    let mut elapsed_beats = 0.0_f32;
    let mut latest_note_end = 0.0_f32;

    for event in events {
        elapsed_beats += event.delta_beat;
        for (_, duration) in &event.notes {
            latest_note_end =
                latest_note_end.max(elapsed_beats + articulated_duration_beats(*duration));
        }
    }

    (latest_note_end - elapsed_beats).max(0.0)
}

/// QMid player
#[derive(Debug)]
pub struct MidiPlayer;

impl MidiPlayer {
    /// Play a Midi struct synchronously.
    /// Block current thread until finish.
    /// Notes are played for 95% of their specified duration to add a short articulation gap
    /// without shifting event start times.
    ///
    /// # Panics
    /// If the Midi is invalid.
    pub fn play(midi: &Midi, np: &NotePlayer) {
        // Validate
        assert!(midi.tempo > 20.0, "Tempo too low");

        let sec_per_beat = 60.0 / midi.tempo;

        for event in &midi.events {
            // wait delta beats
            let wait_sec = event.delta_beat * sec_per_beat;
            NotePlayer::rest_panic(wait_sec);

            // spawn all notes in this event (async together)
            for (freq, dur_beat) in &event.notes {
                let dur_sec = articulated_duration_beats(*dur_beat) * sec_per_beat;
                np.play_freq_async(*freq, dur_sec);
            }
        }

        NotePlayer::rest_panic(remaining_playback_beats(&midi.events) * sec_per_beat);
    }
}

#[cfg(test)]
mod tests {
    use super::{articulated_duration_beats, remaining_playback_beats};
    use crate::midi::types::Event;

    #[test]
    fn note_articulation_shortens_duration_without_changing_beats() {
        assert_eq!(articulated_duration_beats(1.0), 0.95);
        assert_eq!(articulated_duration_beats(0.5), 0.475);
    }

    #[test]
    fn playback_wait_includes_notes_that_outlast_later_events() {
        let events = [
            Event {
                notes: vec![(440.0, 4.0)],
                delta_beat: 0.0,
            },
            Event {
                notes: vec![(880.0, 0.5)],
                delta_beat: 1.0,
            },
        ];

        assert!((remaining_playback_beats(&events) - 2.8).abs() < 1e-5);
    }
}
