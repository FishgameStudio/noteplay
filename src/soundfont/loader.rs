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

pub(crate) const fn check_timbre_valid_for_synth(timbre: Timbre) -> Result<(), Error> {
    if timbre.to_program().is_none() {
        return Err(Error::TryToApplyDefaultTimbre);
    }
    Ok(())
}

/// Take out preset from the soundfont via the given timbre.
pub(crate) fn make_synth(
    sf: &Arc<SoundFont>,
    timbre: Timbre,
) -> Result<(Synthesizer, SynthesizerSettings), Error> {
    check_timbre_valid_for_synth(timbre)?;
    let settings = SynthesizerSettings::new(44100);
    let mut synth = Synthesizer::new(sf, &settings)?;

    const CHANNEL: i32 = 0;
    synth.process_midi_message(
        CHANNEL,
        0xC0,
        timbre.to_program().ok_or(Error::TryToApplyDefaultTimbre)? as i32,
        0,
    );

    Ok((synth, settings))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;
    use crate::soundfont::types::Timbre;

    #[test]
    fn check_timbre_valid_for_synth_rejects_default() {
        let res = check_timbre_valid_for_synth(Timbre::Default);
        assert!(matches!(res, Err(Error::TryToApplyDefaultTimbre)));
    }

    #[test]
    fn check_timbre_valid_for_synth_accepts_valid_timbre() {
        let res = check_timbre_valid_for_synth(Timbre::AcousticGrandPiano);
        assert!(res.is_ok());
    }
}
