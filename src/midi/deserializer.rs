//! Binary deserializer.

use crate::Error;
use crate::core::Frequency;
use crate::core::types::Tempo;
use crate::midi::types::{Bytes, Event, HEADER_LEN, MAGIC, Midi, VERSION};
use crate::midi::{AUTHOR_LEN, COPYRIGHT_LEN, FixedString, MidiMeta, Note, SONG_NAME_LEN};
use crate::soundfont::Timbre;

use std::io::{Cursor, Read};

/// Deserialize binary which serialized with function [`serialize`](crate::midi::serialize).
///
/// # Errors
/// If the format is invalid.
pub fn deserialize(bin: Bytes) -> Result<Midi, Error> {
    #[inline]
    fn error(msg: &str) -> Result<Midi, Error> {
        Err(Error::InvalidData(msg.to_string()))
    }

    let mut r = Cursor::new(&bin);
    if bin.len() < HEADER_LEN {
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
    // Song name
    let mut song_name_buf = [0u8; SONG_NAME_LEN * 4];
    r.read_exact(&mut song_name_buf)?;
    let song_name = FixedString::<SONG_NAME_LEN>::from_le_bytes(&song_name_buf)?;
    // Copyright
    let mut copyright_buf = [0u8; COPYRIGHT_LEN * 4];
    r.read_exact(&mut copyright_buf)?;
    let copyright = FixedString::<COPYRIGHT_LEN>::from_le_bytes(&copyright_buf)?;
    // Author
    let mut author_buf = [0u8; AUTHOR_LEN * 4];
    r.read_exact(&mut author_buf)?;
    let author = FixedString::<AUTHOR_LEN>::from_le_bytes(&author_buf)?;

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

            let mut timbre_buf = [0u8; 4];
            r.read_exact(&mut timbre_buf)?;
            let t = i32::from_le_bytes(timbre_buf);
            let timbre = Timbre::try_from(t).map_err(|_| Error::UnknownTimbreNum(t))?;

            let mut velocity_buf = [0u8; 4];
            r.read_exact(&mut velocity_buf)?;
            let velocity = i32::from_le_bytes(velocity_buf);

            notes.push((Frequency::new(freq)?, dur, timbre, velocity));
        }
        events.push(Event { notes, delta_beat });
    }

    Ok(Midi {
        tempo: Tempo::new(tempo)?,
        events,
        meta: MidiMeta {
            song_name: Some(song_name).filter(|s| !s.is_all_null()),
            author: Some(author).filter(|s| !s.is_all_null()),
            copyright: Some(copyright).filter(|s| !s.is_all_null()),
        },
    })
}
