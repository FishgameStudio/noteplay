//! Structure serializer.

use crate::midi::types::{Bytes, MAGIC, Midi, VERSION};

/// Serialize a [`Midi`] structure to binary.
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
     * delta_beat + notes_count + note_vec
     * Note_vec:
     * note_frequency + note_duration + note_timbre + note_velocity
     */
    for event in &midi.events {
        let mut event_buffer = Bytes::new();
        event_buffer.extend(&event.delta_beat.to_le_bytes());
        event_buffer.extend(&(event.notes.len() as u32).to_le_bytes());
        for note in &event.notes {
            event_buffer.extend(&note.0.get_freq().to_le_bytes());
            event_buffer.extend(&note.1.to_le_bytes());
            event_buffer.extend(&(note.2 as i32).to_le_bytes());
            event_buffer.extend(&note.3.to_le_bytes());
        }
        buf.extend(event_buffer);
    }

    buf
}
