//! Load sf2 files.

use crate::Error;
use crate::soundfont::types::Timbre;
use rustysynth::{SoundFont, Synthesizer, SynthesizerSettings};
use std::{io::Cursor, sync::Arc};

/// Load soundfont from given bytes (which read from *.sf2 files)
pub(crate) fn load_soundfont_from_bytes(data: &[u8]) -> Result<SoundFont, Error> {
    let mut cursor = Cursor::new(data);
    let sf = SoundFont::new(&mut cursor)?;
    Ok(sf)
}

/// Take out preset from the soundfont via the given timbre.
pub(crate) fn make_synth(
    sf: &Arc<SoundFont>,
    timbre: Timbre,
) -> Result<(Synthesizer, SynthesizerSettings), Error> {
    const CHANNEL: i32 = 0;

    let settings = SynthesizerSettings::new(44100);
    let mut synth = Synthesizer::new(sf, &settings)?;

    if let Some(prog) = timbre.to_program() {
        synth.process_midi_message(CHANNEL, 0xC0, prog as i32, 0);
    }

    Ok((synth, settings))
}

#[cfg(test)]
mod tests {}
