//! Main type definitions.

use num_enum::TryFromPrimitive;
use rodio::Source;
use rustysynth::{Synthesizer, SynthesizerSettings};
use std::num::NonZero;
use std::time::Duration;

/// Enumeration of musical instruments.
///
/// The values of the enumerations follow GM Level 1 standards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(i32)]
#[allow(missing_docs)] // These enumerations don't even need to be documented, right?
pub enum Timbre {
    /// Default sine wave
    Default = -1,

    AcousticGrandPiano = 0,
    BrightAcousticPiano = 1,
    ElectricGrandPiano = 2,
    HonkytonkPiano = 3,
    RhodesElectricPiano = 4,
    ChorusedElectricPiano = 5,
    Harpsichord = 6,
    Clavinet = 7,

    Celesta = 8,
    Glockenspiel = 9,
    MusicBox = 10,
    Vibraphone = 11,
    Marimba = 12,
    Xylophone = 13,
    TubularBells = 14,
    Dulcimer = 15,

    DrawbarOrgan = 16,
    PercussiveOrgan = 17,
    RockOrgan = 18,
    ChurchOrgan = 19,
    ReedOrgan = 20,
    Accordion = 21,
    Harmonica = 22,
    TangoAccordion = 23,

    NylonStringGuitar = 24,
    SteelStringGuitar = 25,
    JazzElectricGuitar = 26,
    CleanElectricGuitar = 27,
    MutedElectricGuitar = 28,
    OverdrivenGuitar = 29,
    DistortionGuitar = 30,
    GuitarHarmonics = 31,

    AcousticBass = 32,
    FingerElectricBass = 33,
    PickElectricBass = 34,
    FretlessBass = 35,
    SlapBass1 = 36,
    SlapBass2 = 37,
    SynthBass1 = 38,
    SynthBass2 = 39,

    Violin = 40,
    Viola = 41,
    Cello = 42,
    Contrabass = 43,
    TremoloStrings = 44,
    PizzicatoStrings = 45,
    OrchestralHarp = 46,
    Timpani = 47,

    StringEnsemble1 = 48,
    StringEnsemble2 = 49,
    SynthStrings1 = 50,
    SynthStrings2 = 51,
    ChoirAahs = 52,
    VoiceOohs = 53,
    SynthChoir = 54,
    OrchestraHit = 55,

    Trumpet = 56,
    Trombone = 57,
    Tuba = 58,
    MutedTrumpet = 59,
    FrenchHorn = 60,
    BrassSection = 61,
    SynthBrass1 = 62,
    SynthBrass2 = 63,

    SopranoSax = 64,
    AltoSax = 65,
    TenorSax = 66,
    BaritoneSax = 67,
    Oboe = 68,
    EnglishHorn = 69,
    Bassoon = 70,
    Clarinet = 71,

    Piccolo = 72,
    Flute = 73,
    Recorder = 74,
    PanFlute = 75,
    BlownBottle = 76,
    Shakuhachi = 77,
    Whistle = 78,
    Ocarina = 79,

    LeadSquare = 80,
    LeadSawtooth = 81,
    LeadCalliope = 82,
    LeadChiff = 83,
    LeadCharang = 84,
    LeadVoice = 85,
    LeadFifths = 86,
    LeadBassLead = 87,

    PadNewAge = 88,
    PadWarm = 89,
    PadPolySynth = 90,
    PadChoir = 91,
    PadBowed = 92,
    PadMetallic = 93,
    PadHalo = 94,
    PadSweep = 95,

    FxRain = 96,
    FxSoundtrack = 97,
    FxCrystal = 98,
    FxAtmosphere = 99,
    FxBrightness = 100,
    FxGoblins = 101,
    FxEchoes = 102,
    FxSciFi = 103,

    Sitar = 104,
    Banjo = 105,
    Shamisen = 106,
    Koto = 107,
    Kalimba = 108,
    Bagpipe = 109,
    Fiddle = 110,
    Shanai = 111,

    TinkleBell = 112,
    Agogo = 113,
    SteelDrums = 114,
    Woodblock = 115,
    TaikoDrum = 116,
    MelodicTom = 117,
    SynthDrum = 118,
    ReverseCymbal = 119,

    GuitarFretNoise = 120,
    BreathNoise = 121,
    Seashore = 122,
    BirdTweet = 123,
    TelephoneRing = 124,
    Helicopter = 125,
    Applause = 126,
    Gunshot = 127,
}
impl Timbre {
    /// Convert to program number, [`None`] if it is [`Timbre::Default`](Self::Default).
    pub const fn to_program(self) -> Option<u32> {
        match self {
            Self::Default => None,
            other => Some(other as i32 as u32),
        }
    }
}

#[inline]
pub(crate) fn freq_to_midi(freq: f32) -> i32 {
    let midi = 12.0f32.mul_add((freq / 440.0).log2(), 69.0);
    midi.clamp(0.0, 127.0).round() as i32
}

use std::sync::{Arc, Mutex};

/// Wrapper: wraps the rustysynth Synthesizer into a rodio Source
/// Stereo f32 audio stream
pub(crate) struct SynthSource {
    synth: Arc<Mutex<Synthesizer>>,
    sample_rate: NonZero<u32>,
    remaining_samples: u64,
    buffer: Vec<f32>,
    pos: usize,
    midi_note: i32,
    channel: i32,
}

impl SynthSource {
    pub(crate) fn new(
        synth: Arc<Mutex<Synthesizer>>,
        settings: &SynthesizerSettings,
        midi_note: i32,
        velocity: i32,
        sec: Duration,
    ) -> Self {
        const CHANNEL: i32 = 0;
        synth.lock().unwrap().note_on(CHANNEL, midi_note, velocity);

        let sample_rate = NonZero::new(settings.sample_rate as u32).unwrap();
        let total_samples = (sec.as_secs_f32() * sample_rate.get() as f32) as u64;
        let buffer = Vec::with_capacity(1024 * 2);
        Self {
            synth,
            sample_rate,
            remaining_samples: total_samples,
            buffer,
            pos: 0,
            midi_note,
            channel: CHANNEL,
        }
    }

    pub(crate) fn refill_buffer(&mut self) {
        self.buffer.clear();
        self.pos = 0;
        if self.remaining_samples == 0 {
            return;
        }
        let render = 1024u64.min(self.remaining_samples);
        let mut left = vec![0f32; render as usize];
        let mut right = vec![0f32; render as usize];
        self.synth.lock().unwrap().render(&mut left, &mut right);
        for (l, r) in left.into_iter().zip(right) {
            self.buffer.push(l);
            self.buffer.push(r);
        }
        self.remaining_samples -= render;

        if self.remaining_samples == 0 {
            self.synth
                .lock()
                .unwrap()
                .note_off(self.channel, self.midi_note);
        }
    }
}

impl Iterator for SynthSource {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.buffer.len() {
            if self.remaining_samples == 0 {
                return None;
            }
            self.refill_buffer();
            if self.buffer.is_empty() {
                return None;
            }
        }
        let v = self.buffer[self.pos];
        self.pos += 1;
        Some(v)
    }
}

impl Source for SynthSource {
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> NonZero<u16> {
        NonZero::new(2).unwrap()
    }
    fn sample_rate(&self) -> NonZero<u32> {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timbre_default_to_program_none() {
        assert!(Timbre::Default.to_program().is_none());
    }

    #[test]
    fn timbre_valid_to_program_some() {
        assert_eq!(Timbre::AcousticGrandPiano.to_program(), Some(0));
        assert_eq!(Timbre::SteelStringGuitar.to_program(), Some(25));
    }

    #[test]
    fn freq_to_midi_a4_is_69() {
        assert_eq!(freq_to_midi(440.0), 69);
    }

    #[test]
    fn freq_to_midi_clamp_low() {
        assert_eq!(freq_to_midi(0.01), 0);
    }

    #[test]
    fn freq_to_midi_clamp_high() {
        assert_eq!(freq_to_midi(50000.0), 127);
    }

    #[test]
    fn freq_to_midi_round_correct() {
        // A4 ±一点，四舍五入到69
        assert_eq!(freq_to_midi(438.0), 69);
        assert_eq!(freq_to_midi(442.0), 69);
    }
}
