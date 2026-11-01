//! Player for deserialized structure.
use crate::core::note_player::NotePlayer;
use crate::midi::types::Midi;

/// QMid player
#[derive(Debug)]
pub struct MidiPlayer;

impl MidiPlayer {
    /// Play a Midi struct synchronously.
    /// Block current thread until finish.
    pub fn play(midi: &Midi, np: &NotePlayer) {
        let sec_per_beat = 60.0 / midi.tempo;

        for event in &midi.events {
            // wait delta beats
            let wait_sec = event.delta_beat * sec_per_beat;
            NotePlayer::rest(wait_sec);

            // spawn all notes in this event (async together)
            for (freq, dur_beat) in &event.notes {
                let dur_sec = dur_beat * sec_per_beat;
                np.play_freq_async(*freq, dur_sec);
            }
        }

        // wait the longest note in last event to finish
        if let Some(last) = midi.events.last() {
            let max_dur = last.notes.iter().map(|(_, d)| *d).fold(0.0, f32::max);
            NotePlayer::rest(max_dur * sec_per_beat);
        }
    }
}
