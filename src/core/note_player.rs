use crate::Error;
use crate::core::Frequency;
use crate::core::note_parser::note2freq;
use rodio::mixer::Mixer;
use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, MixerDeviceSink};
use std::time::Duration;

/// Addition for a [`Duration`] and a [`f32`].
#[inline]
fn dur_add_f32(d: Duration, sec: f32) -> Duration {
    d + Duration::from_secs_f32(sec)
}

/// The note player.
pub struct NotePlayer {
    _handle: MixerDeviceSink,
    mixer: Mixer,
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
    pub fn new() -> Result<Self, Error> {
        let mut handle = DeviceSinkBuilder::open_default_sink()?;
        handle.log_on_drop(false);
        let mixer = handle.mixer().clone();
        Ok(Self {
            _handle: handle,
            mixer,
        })
    }

    /// **Asynchronous** play a note of a specified frequency.
    /// # Panics
    /// If the frequency isn't greater than 0.
    /// # Example
    /// ```no_run
    /// # use noteplay::core::{NotePlayer, Frequency};
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let p = NotePlayer::new()?;
    /// let secs = Duration::from_secs_f32(0.5);
    /// // Play A4 for 0.5 seconds.
    /// p.play_freq_async(Frequency::new_unchecked(440.0), secs);
    /// std::thread::sleep(secs);
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_freq_async(&self, freq: Frequency, sec: Duration) {
        assert!(freq.get_freq() >= 0.0, "Frequency must be greater than 0");
        let source = SineWave::new(freq.get_freq()).take_duration(sec);
        self.mixer.add(source);
    }

    /// Synchronous play a note of a specified frequency.
    /// # Panics
    /// If the frequency isn't greater than 0.
    /// # Example
    /// ```no_run
    /// # use noteplay::core::{NotePlayer, Frequency};
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let p = NotePlayer::new()?;
    /// // Play A4 for 0.5 seconds.
    /// p.play_freq(Frequency::new_unchecked(440.0), Duration::from_secs_f32(0.5));
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_freq(&self, freq: Frequency, sec: Duration) {
        self.play_freq_async(freq, sec);
        std::thread::sleep(dur_add_f32(sec, 0.05)); // Pause for a moment
    }

    /// **Asynchronous** play a specified note within given seconds.
    /// Click [here](https://www.dolmetsch.com/musictheory1.htm) to learn more about the note.
    ///
    /// # Panics
    ///
    /// If the format of the note is invalid.
    /// The note must be greater than C0.
    ///
    /// # Example
    /// ```no_run
    /// # use noteplay::core::NotePlayer;
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let p = NotePlayer::new()?;
    /// let secs = Duration::from_secs_f32(0.5);
    /// // Play A4 for 0.5 seconds.
    /// p.play_note_async("A4", secs);
    /// std::thread::sleep(secs);
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_note_async(&self, note: &str, sec: Duration) {
        self.play_freq_async(note2freq(note), sec);
    }

    /// Synchronous play a specified note within given seconds.
    /// Click [here](https://www.dolmetsch.com/musictheory1.htm) to learn more about the note.
    ///
    /// # Panics
    ///
    /// If the format of the note is invalid.
    /// The note must be greater than C0.
    ///
    /// # Example
    /// ```no_run
    /// # use noteplay::core::NotePlayer;
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let p = NotePlayer::new()?;
    /// // Play A4 for 0.5 seconds.
    /// p.play_note("A4", Duration::from_secs_f32(0.5));
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_note(&self, note: &str, sec: Duration) {
        self.play_freq(note2freq(note), sec);
    }

    /// Synchronous repeat notes of a specified frequency.
    ///
    /// # Panics
    /// If the frequency isn't greater than 0.
    pub fn repeat_freq(&self, freq: Frequency, sec: Duration, repetitions: u64) {
        for _ in 0..repetitions {
            self.play_freq(freq, sec);
        }
    }
    /// Synchronous repeat notes.
    /// Click [here](https://www.dolmetsch.com/musictheory1.htm) to learn more about the note.
    ///
    /// # Panics
    /// If the frequency isn't greater than 0.
    pub fn repeat_note(&self, note: &str, sec: Duration, repetitions: u64) {
        for _ in 0..repetitions {
            self.play_note(note, sec);
        }
    }
    /// Wait for seconds.
    pub fn rest(sec: Duration) {
        std::thread::sleep(sec);
    }
}
