use crate::Error;
use crate::core::note_parser::note2freq;
use crate::core::types::Frequency;
use crate::soundfont::loader::make_synth;
use crate::soundfont::types::{SynthHandle, Timbre, freq_to_midi};
use rodio::mixer::Mixer;
use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, MixerDeviceSink};
use rustysynth::SoundFont;
use rustysynth::SynthesizerSettings;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// Addition for a [`Duration`] and a [`f32`].
///
/// # Example
/// ```no_run
/// # use std::time::Duration;
/// # fn main() {
/// let d = Duration::from_secs_f32(1.0) + Duration::from_secs_f32(1.0);
/// assert_eq!(d.as_secs_f32(), 2.0);
/// # }
/// ```
#[inline]
fn dur_add_f32(d: Duration, sec: f32) -> Duration {
    d + Duration::from_secs_f32(sec)
}

/// The note player.
pub struct NotePlayer {
    // Keep lifetime
    _handle: MixerDeviceSink,
    mixer: Mixer,
    timbre: Timbre,
    sf: Option<Arc<SoundFont>>,
    pub(crate) synth: SynthHandle,
}

impl std::fmt::Debug for NotePlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NotePlayer").finish_non_exhaustive()
    }
}

impl NotePlayer {
    /// Create a new [`NotePlayer`] object.
    /// # Errors
    /// If failed to open the default sink.
    ///
    /// # Example
    /// ```no_run
    /// # use noteplay::core::NotePlayer;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let player = NotePlayer::new()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new() -> Result<Self, Error> {
        let mut handle = DeviceSinkBuilder::open_default_sink()?;
        handle.log_on_drop(false);
        let mixer = handle.mixer().clone();
        let synth_settings = SynthesizerSettings::new(44100);
        let synth = SynthHandle::new(&synth_settings);
        mixer.add(synth.source());
        Ok(Self {
            _handle: handle,
            mixer,
            timbre: Timbre::Default,
            sf: None,
            synth,
        })
    }

    /// Get the timbre.
    #[must_use]
    ///
    /// # Example
    /// ```no_run
    /// # use noteplay::core::NotePlayer;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let player = NotePlayer::new()?;
    /// let timbre = player.get_timbre();
    /// # Ok(())
    /// # }
    /// ```
    pub const fn get_timbre(&self) -> Timbre {
        self.timbre
    }

    /// Set the timbre.
    ///
    /// # Example
    /// ```no_run
    /// # use noteplay::core::NotePlayer;
    /// # use noteplay::soundfont::Timbre;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut player = NotePlayer::new()?;
    /// player.set_timbre(Timbre::Default);
    /// # Ok(())
    /// # }
    /// ```
    pub const fn set_timbre(&mut self, timbre: Timbre) {
        self.timbre = timbre;
    }

    /// Set the soundfont from bytes from a **sf2** file.
    /// # Errors
    /// If the file isn't a valid sf2 file.
    ///
    /// # Example
    /// ```no_run
    /// # use noteplay::core::NotePlayer;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut player = NotePlayer::new()?;
    /// player.set_soundfont_from_bytes(&[])?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_soundfont_from_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        use crate::soundfont::loader::load_soundfont_from_bytes;
        let sf = Arc::new(load_soundfont_from_bytes(bytes)?);
        let t = make_synth(&sf, self.timbre)?;
        self.synth.install(t.0);
        self.sf = Some(sf);
        Ok(())
    }

    /// Set the soundfont from a **sf2** file.
    /// # Errors
    /// If failed to read the file or the file isn't a valid sf2 file.
    pub fn set_soundfont<P>(&mut self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let bytes = std::fs::read(path)?;
        self.set_soundfont_from_bytes(&bytes)?;

        Ok(())
    }

    pub(crate) fn set_sf(&mut self, sf: SoundFont) -> Result<(), Error> {
        let sf = Arc::new(sf);
        self.synth.install(make_synth(&sf, self.timbre)?.0);
        self.sf = Some(sf);
        Ok(())
    }

    /// **Asynchronously** play a note of a specified frequency.
    /// # Panics
    /// - If the frequency is not greater than 0.
    /// - If the timbre is not [`Timbre::Default`] but no soundfont has been loaded.
    /// # Example
    /// ```no_run
    /// # use noteplay::core::{NotePlayer, Frequency};
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let p = NotePlayer::new()?;
    /// let secs = Duration::from_secs_f32(0.5);
    /// // Play A4 for 0.5 seconds.
    /// p.play_freq_async(Frequency::new_unchecked(440.0), secs, 90);
    /// std::thread::sleep(secs);
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_freq_async(&self, freq: Frequency, sec: Duration, velocity: i32) {
        if self.timbre == Timbre::Default {
            let source = SineWave::new(freq.get_freq()).take_duration(sec);
            self.mixer.add(source);
            return;
        }

        let midi = freq_to_midi(freq.get_freq());
        assert!(
            self.synth.is_ready(),
            "Timbre is not Default, but no soundfont loaded. Call set_soundfont first."
        );
        self.synth.queue_note(midi, velocity, self.timbre, sec);
    }

    /// Synchronously play a note of a specified frequency.
    /// # Panics
    /// - If the frequency is not greater than 0.
    /// - If the timbre is not [`Timbre::Default`] but no soundfont has been loaded.
    /// # Example
    /// ```no_run
    /// # use noteplay::core::{NotePlayer, Frequency};
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let p = NotePlayer::new()?;
    /// // Play A4 for 0.5 seconds.
    /// p.play_freq(Frequency::new_unchecked(440.0), Duration::from_secs_f32(0.5), 90);
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_freq(&self, freq: Frequency, sec: Duration, velocity: i32) {
        self.play_freq_async(freq, sec, velocity);
        std::thread::sleep(dur_add_f32(sec, 0.05)); // Pause for a moment
    }

    /// **Asynchronously** play a specified note within given seconds.
    ///
    /// # Panics
    /// - If the note format is invalid.
    /// - If the note is lower than C0.
    /// - If the timbre is not [`Timbre::Default`] but no soundfont has been loaded.
    ///
    /// # Example
    /// ```no_run
    /// # use noteplay::core::NotePlayer;
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let p = NotePlayer::new()?;
    /// let secs = Duration::from_secs_f32(0.5);
    /// // Play A4 for 0.5 seconds.
    /// p.play_note_async("A4", secs, 90);
    /// std::thread::sleep(secs);
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_note_async(&self, note: &str, sec: Duration, velocity: i32) {
        self.play_freq_async(note2freq(note), sec, velocity);
    }

    /// Synchronously play a specified note within given seconds.
    ///
    /// # Panics
    /// - If the note format is invalid.
    /// - If the note is lower than C0.
    /// - If the timbre is not [`Timbre::Default`] but no soundfont has been loaded.
    ///
    /// # Example
    /// ```no_run
    /// # use noteplay::core::NotePlayer;
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let p = NotePlayer::new()?;
    /// // Play A4 for 0.5 seconds.
    /// p.play_note("A4", Duration::from_secs_f32(0.5), 90);
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_note(&self, note: &str, sec: Duration, velocity: i32) {
        self.play_freq(note2freq(note), sec, velocity);
    }

    /// Synchronously repeat notes of a specified frequency.
    ///
    /// # Panics
    /// - If the frequency is not greater than 0.
    /// - If the timbre is not [`Timbre::Default`] but no soundfont has been loaded.
    ///
    /// # Example
    /// ```no_run
    /// # use noteplay::core::{Frequency, NotePlayer};
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let player = NotePlayer::new()?;
    /// player.repeat_freq(Frequency::new_unchecked(440.0), Duration::from_millis(100), 2, 90);
    /// # Ok(())
    /// # }
    /// ```
    pub fn repeat_freq(&self, freq: Frequency, sec: Duration, repetitions: u64, velocity: i32) {
        for _ in 0..repetitions {
            self.play_freq(freq, sec, velocity);
        }
    }

    /// Synchronously repeat notes.
    ///
    /// # Panics
    /// - If the frequency is not greater than 0.
    /// - If the timbre is not [`Timbre::Default`] but no soundfont has been loaded.
    ///
    /// # Example
    /// ```no_run
    /// # use noteplay::core::NotePlayer;
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let player = NotePlayer::new()?;
    /// player.repeat_note("A4", Duration::from_millis(100), 2, 90);
    /// # Ok(())
    /// # }
    /// ```
    pub fn repeat_note(&self, note: &str, sec: Duration, repetitions: u64, velocity: i32) {
        for _ in 0..repetitions {
            self.play_note(note, sec, velocity);
        }
    }

    /// Wait for seconds.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::NotePlayer;
    /// # use std::time::Duration;
    /// NotePlayer::rest(Duration::ZERO);
    /// ```
    pub fn rest(sec: Duration) {
        std::thread::sleep(sec);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn dur_add_f32_correct() {
        let base = Duration::from_secs(2);
        let out = dur_add_f32(base, 0.05);
        let expected = Duration::from_millis(2050);
        assert!(
            out >= expected.saturating_sub(Duration::from_millis(1))
                && out <= expected.saturating_add(Duration::from_millis(1)),
            "{out:?} should be near {expected:?}"
        );
    }
}
