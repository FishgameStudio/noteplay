//! Structure serializer.

use crate::midi::{
    Event,
    types::{Bytes, MAGIC, Midi, VERSION},
};

/// Serialize a [`Midi`] structure to binary.
///
/// # Example
/// ```
/// # use noteplay::midi::{serialize, Midi, MAGIC};
/// let bytes = serialize(&Midi::new());
/// assert!(bytes.starts_with(MAGIC));
/// ```
#[must_use]
pub fn serialize(midi: &Midi) -> Bytes {
    let mut buf: Bytes = Vec::new();

    /*
     * Header:
     * magic number + version + BPM + events_count + Meta
     * Meta:
     * song_name + copyright + author
     * Total 718 bytes.
     */
    let mut header: Bytes = vec![];
    header.extend(MAGIC);
    header.extend(&VERSION.to_le_bytes());
    header.extend(&midi.tempo.get_tempo().to_le_bytes());
    header.extend(&(midi.events.len() as u32).to_le_bytes());
    header.extend(&midi.meta.song_name.unwrap_or_default().to_u32_le_bytes());
    header.extend(&midi.meta.copyright.unwrap_or_default().to_u32_le_bytes());
    header.extend(&midi.meta.author.unwrap_or_default().to_u32_le_bytes());

    buf.extend(header);

    /*
     * Events:
     * type(u8) + data
     * Data for normal events:
     * delta_beat + notes_count + note_vec
     * Data for tempo change:
     * tempo: f32
     * Data for EOF:
     * (None)
     * Note_vec:
     * note_frequency + note_duration + note_timbre + note_velocity
     */
    for event in &midi.events {
        let mut event_buffer = Bytes::new();
        match event {
            Event::Normal { notes, delta_beat } => {
                event_buffer.extend(&0u8.to_le_bytes());
                event_buffer.extend(&delta_beat.to_le_bytes());
                event_buffer.extend(&(notes.len() as u32).to_le_bytes());
                for note in notes {
                    event_buffer.extend(&note.0.get_freq().to_le_bytes());
                    event_buffer.extend(&note.1.to_le_bytes());
                    event_buffer.extend(&(note.2 as i32).to_le_bytes());
                    event_buffer.extend(&note.3.to_le_bytes());
                }
            }
            Event::TempoChange { tempo } => {
                event_buffer.extend(&1u8.to_le_bytes());
                event_buffer.extend(tempo.get_tempo().to_le_bytes());
            }
        };
        buf.extend(event_buffer);
    }

    buf
}
