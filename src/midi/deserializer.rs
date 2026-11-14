//! Binary deserailizer.

use crate::Error;
use crate::core::Frequency;
use crate::core::types::Tempo;
use crate::midi::Note;
use crate::midi::types::{Bytes, Event, MAGIC, Midi, VERSION};

use std::io::{Cursor, Read};

/// Deserialize binary which serialized with function [`serialize()`](crate::midi::serialize).
/// # Errors
/// If the format is invalid.
pub fn deserialize(bin: Bytes) -> Result<Midi, Error> {
    #[inline]
    fn error(msg: &str) -> Result<Midi, Error> {
        Err(Error::InvalidData(msg.to_string()))
    }

    let mut r = Cursor::new(&bin);
    if bin.len() < 14 {
        error("Binary too short")?;
    }
    // Magic number
    let mut magic = [0u8; 4];
    r.read_exact(&mut magic)?;
    if magic != MAGIC {
        error("not a QMid file")?;
    }
    // Version
    let mut version_buf = [0u8; 2];
    r.read_exact(&mut version_buf)?;
    let version = u16::from_le_bytes(version_buf);
    if version != VERSION {
        error("Version mismatch")?;
    }
    // Tempo BPM
    let mut tempo_buf = [0u8; 4];
    r.read_exact(&mut tempo_buf)?;
    let tempo = f32::from_le_bytes(tempo_buf);
    if tempo < 0.0 {
        error(&format!("Invalid tempo: {tempo}"))?;
    }
    // Event count
    let mut event_cnt_buf = [0u8; 4];
    r.read_exact(&mut event_cnt_buf)?;
    let event_count = u32::from_le_bytes(event_cnt_buf);

    let mut events = Vec::with_capacity(event_count as usize);
    for _ in 0..event_count {
        let mut delta_buf = [0u8; 4];
        r.read_exact(&mut delta_buf)?;
        let delta_beat = f32::from_le_bytes(delta_buf);

        let mut note_cnt_buf = [0u8; 4];
        r.read_exact(&mut note_cnt_buf)?;
        let note_count = u32::from_le_bytes(note_cnt_buf);

        let mut notes: Vec<Note> = Vec::with_capacity(note_count as usize);
        for _ in 0..note_count {
            let mut freq_buf = [0u8; 4];
            r.read_exact(&mut freq_buf)?;
            let freq = f32::from_le_bytes(freq_buf);

            let mut dur_buf = [0u8; 4];
            r.read_exact(&mut dur_buf)?;
            let dur = f32::from_le_bytes(dur_buf);

            notes.push((Frequency::new_unchecked(freq), dur));
        }
        events.push(Event { notes, delta_beat });
    }

    Ok(Midi {
        tempo: Tempo::new_unchecked(tempo),
        events,
    })
}
