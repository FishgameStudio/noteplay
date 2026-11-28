//! Main type definitions.

use crate::Error;

/// A structure that stores a legal [`f32`] number of a frequency.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Frequency {
    freq: f32,
}
impl Frequency {
    /// Create a new [`Frequency`] object.
    ///
    /// # Errors
    /// This function returns an [`Err`] value if the `freq`
    /// isn't finite or is less than 0.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Frequency;
    /// let frequency = Frequency::new(440.0).unwrap();
    /// assert_eq!(frequency.get_freq(), 440.0);
    /// ```
    pub const fn new(freq: f32) -> Result<Self, Error> {
        if freq.is_finite() && freq >= 0.0 {
            Ok(Self { freq })
        } else {
            Err(Error::InvalidFloat(freq))
        }
    }
    /// Create a new [`Frequency`] object **without validation**.
    ///
    /// ## ⚠️ Caution
    /// You must call this function when you really ensure that `freq` is
    /// **finite and greater than 0**.
    ///
    /// # Panics
    /// If the freq is invalid.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Frequency;
    /// let frequency = Frequency::new_unchecked(440.0);
    /// assert_eq!(frequency.get_freq(), 440.0);
    /// ```
    #[must_use]
    pub const fn new_unchecked(freq: f32) -> Self {
        assert!(freq.is_finite() && freq >= 0.0);
        Self { freq }
    }

    /// Get the frequency.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Frequency;
    /// let frequency = Frequency::new_unchecked(440.0);
    /// assert_eq!(frequency.get_freq(), 440.0);
    /// ```
    #[must_use]
    pub const fn get_freq(&self) -> f32 {
        self.freq
    }
    /// Set the frequency.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Frequency;
    /// let mut frequency = Frequency::new_unchecked(440.0);
    /// frequency.set_freq(880.0).unwrap();
    /// assert_eq!(frequency.get_freq(), 880.0);
    /// ```
    pub const fn set_freq(&mut self, freq: f32) -> Result<(), Error> {
        if freq.is_finite() && freq >= 0.0 {
            self.freq = freq;
            Ok(())
        } else {
            Err(Error::InvalidFloat(freq))
        }
    }
    /// Set the frequency **without validation**.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Frequency;
    /// let mut frequency = Frequency::new_unchecked(440.0);
    /// frequency.set_freq_unchecked(880.0);
    /// assert_eq!(frequency.get_freq(), 880.0);
    /// ```
    pub const fn set_freq_unchecked(&mut self, freq: f32) {
        assert!(freq.is_finite() && freq >= 0.0);
        self.freq = freq;
    }
}

/// BPM, beats per seconds.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tempo {
    tempo: f32,
}
impl Tempo {
    /// Create a new [`Tempo`] object.
    ///
    /// # Errors
    /// This function returns an [`Err`] value if the `tempo`
    /// isn't finite or is less than 0.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Tempo;
    /// let tempo = Tempo::new(120.0).unwrap();
    /// assert_eq!(tempo.get_tempo(), 120.0);
    /// ```
    pub const fn new(tempo: f32) -> Result<Self, Error> {
        if tempo.is_finite() && tempo >= 0.0 {
            Ok(Self { tempo })
        } else {
            Err(Error::InvalidFloat(tempo))
        }
    }
    /// Create a new [`Tempo`] object **without validation**.
    ///
    /// ## ⚠️ Caution
    /// You must call this function when you really ensure that `tempo` is
    /// **finite and greater than 0**.
    ///
    /// # Panics
    /// If the freq is invalid.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Tempo;
    /// let tempo = Tempo::new_unchecked(120.0);
    /// assert_eq!(tempo.get_tempo(), 120.0);
    /// ```
    #[must_use]
    pub const fn new_unchecked(tempo: f32) -> Self {
        assert!(tempo.is_finite() && tempo >= 0.0);
        Self { tempo }
    }

    /// Get the tempo.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Tempo;
    /// let tempo = Tempo::new_unchecked(120.0);
    /// assert_eq!(tempo.get_tempo(), 120.0);
    /// ```
    #[must_use]
    pub const fn get_tempo(&self) -> f32 {
        self.tempo
    }
    /// Set the tempo.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Tempo;
    /// let mut tempo = Tempo::new_unchecked(120.0);
    /// tempo.set_tempo(90.0).unwrap();
    /// assert_eq!(tempo.get_tempo(), 90.0);
    /// ```
    pub const fn set_tempo(&mut self, tempo: f32) -> Result<(), Error> {
        if tempo.is_finite() && tempo >= 0.0 {
            self.tempo = tempo;
            Ok(())
        } else {
            Err(Error::InvalidFloat(tempo))
        }
    }
    /// Set the tempo **without validation**.
    ///
    /// # Example
    /// ```
    /// # use noteplay::core::Tempo;
    /// let mut tempo = Tempo::new_unchecked(120.0);
    /// tempo.set_tempo_unchecked(90.0);
    /// assert_eq!(tempo.get_tempo(), 90.0);
    /// ```
    pub const fn set_tempo_unchecked(&mut self, tempo: f32) {
        assert!(tempo.is_finite() && tempo >= 0.0);
        self.tempo = tempo;
    }
}

/// Semitones
pub type Semitones = i32;
