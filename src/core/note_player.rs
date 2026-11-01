use crate::core::note_parser::note2freq;
use rodio::source::{SineWave, Source};
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player};
use std::time::Duration;

/// The note player.
/// This struct has no public fields.
/// [Noteplsye](NotePlayer)
pub struct NotePlayer {
    _handle: MixerDeviceSink,
    player: Player,
}

impl std::fmt::Debug for NotePlayer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Noteplayer").finish_non_exhaustive()
    }
}

impl NotePlayer {
    /// Create a new [`NotePlayer`] object.
    /// # Errors
    /// If failed to open the default sink.
    pub fn new() -> Result<Self, rodio::DeviceSinkError> {
        let mut handle = DeviceSinkBuilder::open_default_sink()?;
        handle.log_on_drop(false);
        let player = Player::connect_new(handle.mixer());
        Ok(Self {
            _handle: handle,
            player,
        })
    }

    /// **Asynchronous** play a note of a specified frequency.
    /// # Panics
    /// If the frequency isn't greater than 0.
    /// # Example
    /// ```no_run
    /// # use noteplay::core::NotePlayer;
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let p = NotePlayer::new()?;
    /// // Play A4 for 0.5 seconds.
    /// p.play_freq_async(440.0, 0.5);
    /// std::thread::sleep(Duration::from_secs_f32(0.5));
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_freq_async(&self, freq: f32, sec: f32) {
        assert!(freq > 0.0, "Frequency must be greater than 0");
        let source = SineWave::new(freq).take_duration(Duration::from_secs_f32(sec));
        self.player.append(source);
    }

    /// Synchronous play a note of a specified frequency.
    /// # Panics
    /// If the frequency isn't greater than 0.
    /// # Example
    /// ```no_run
    /// # use noteplay::core::NotePlayer;
    /// # use std::time::Duration;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let p = NotePlayer::new()?;
    /// // Play A4 for 0.5 seconds.
    /// p.play_freq(440.0, 0.5);
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_freq(&self, freq: f32, sec: f32) {
        self.play_freq_async(freq, sec);
        std::thread::sleep(Duration::from_secs_f32(sec + 0.05)); // Pause for a moment
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
    /// // Play A4 for 0.5 seconds.
    /// p.play_note_async("A4", 0.5);
    /// std::thread::sleep(Duration::from_secs_f32(0.5));
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_note_async(&self, note: &str, sec: f32) {
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
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let p = NotePlayer::new()?;
    /// // Play A4 for 0.5 seconds.
    /// p.play_note("A4", 0.5);
    /// # Ok(())
    /// # }
    /// ```
    pub fn play_note(&self, note: &str, sec: f32) {
        self.play_freq(note2freq(note), sec);
    }

    /// Synchronous repeat notes of a specified frequency.
    ///
    /// # Panics
    /// If the frequency isn't greater than 0.
    pub fn repeat_freq(&self, freq: f32, sec: f32, repetitions: u64) {
        for _ in 0..repetitions {
            self.play_freq(freq, sec);
        }
    }
    /// Synchronous repeat notes.
    /// Click [here](https://www.dolmetsch.com/musictheory1.htm) to learn more about the note.
    ///
    /// # Panics
    /// If the frequency isn't greater than 0.
    pub fn repeat_note(&self, note: &str, sec: f32, repetitions: u64) {
        for _ in 0..repetitions {
            self.play_note(note, sec);
        }
    }
}
