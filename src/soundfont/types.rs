//! Main type definitions.

use num_enum::TryFromPrimitive;
use rodio::Source;
use rustysynth::{Synthesizer, SynthesizerSettings};
use std::collections::VecDeque;
use std::num::NonZero;
use std::sync::{Arc, Mutex};
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
    ///
    /// # Example
    /// ```
    /// # use noteplay::soundfont::Timbre;
    /// assert_eq!(Timbre::Default.to_program(), None);
    /// assert_eq!(Timbre::Cello.to_program(), Some(42));
    /// ```
    #[must_use]
    pub const fn to_program(self) -> Option<u32> {
        match self {
            Self::Default => None,
            other => Some(other as u32),
        }
    }
}

#[inline]
/// Convert to a Midi note number.
pub(crate) fn freq_to_midi(freq: f32) -> i32 {
    let midi = 12.0f32.mul_add((freq / 440.0).log2(), 69.0);
    midi.clamp(0.0, 127.0).round() as i32
}

struct NoteEvent {
    midi_note: i32,
    velocity: i32,
    program: u32,
    duration_samples: u64,
}

struct SynthEngine {
    synth: Option<Synthesizer>,
    pending_notes: VecDeque<NoteEvent>,
    generation: u64,
}

#[derive(Clone)]
pub(crate) struct SynthHandle {
    engine: Arc<Mutex<SynthEngine>>,
    sample_rate: NonZero<u32>,
}

impl SynthHandle {
    pub(crate) fn new(settings: &SynthesizerSettings) -> Self {
        Self {
            engine: Arc::new(Mutex::new(SynthEngine {
                synth: None,
                pending_notes: VecDeque::new(),
                generation: 0,
            })),
            sample_rate: NonZero::new(settings.sample_rate as u32).unwrap(),
        }
    }

    pub(crate) fn install(&self, synth: Synthesizer) {
        let mut engine = self.engine.lock().unwrap();
        engine.pending_notes.clear();
        engine.synth = Some(synth);
        engine.generation = engine.generation.wrapping_add(1);
    }

    pub(crate) fn is_ready(&self) -> bool {
        self.engine.lock().unwrap().synth.is_some()
    }

    pub(crate) fn queue_note(
        &self,
        midi_note: i32,
        velocity: i32,
        timbre: Timbre,
        duration: Duration,
    ) {
        let program = timbre
            .to_program()
            .expect("SoundFont playback requires a non-default timbre");
        let duration_samples = (duration.as_secs_f64() * f64::from(self.sample_rate.get()))
            .min(u64::MAX as f64) as u64;
        self.engine
            .lock()
            .unwrap()
            .pending_notes
            .push_back(NoteEvent {
                midi_note,
                velocity,
                program,
                duration_samples,
            });
    }

    pub(crate) fn source(&self) -> SynthSource {
        SynthSource {
            engine: self.engine.clone(),
            sample_rate: self.sample_rate,
            generation: 0,
            active_notes: Vec::new(),
            buffer: Vec::with_capacity(256 * 2),
            left: vec![0.0; 256],
            right: vec![0.0; 256],
            pos: 0,
        }
    }
}

struct ActiveNote {
    channel: i32,
    midi_note: i32,
    remaining_samples: u64,
}

/// A single rodio source that advances the `SoundFont` synthesizer's shared timeline.
pub(crate) struct SynthSource {
    engine: Arc<Mutex<SynthEngine>>,
    sample_rate: NonZero<u32>,
    generation: u64,
    active_notes: Vec<ActiveNote>,
    buffer: Vec<f32>,
    left: Vec<f32>,
    right: Vec<f32>,
    pos: usize,
}

impl SynthSource {
    #[allow(clippy::significant_drop_tightening)]
    // SAFETY: synth borrows from engine guard;
    // cannot drop engine early even if engine name
    // is no longer used
    pub(crate) fn refill_buffer(&mut self) {
        const CHANNEL: i32 = 0;
        const RENDER_FRAMES: usize = 256;

        self.buffer.clear();
        self.pos = 0;

        self.left.fill(0.0);
        self.right.fill(0.0);
        let mut engine = self.engine.lock().unwrap();
        if self.generation != engine.generation {
            self.active_notes.clear();
            self.generation = engine.generation;
        }
        let pending_notes = engine.pending_notes.drain(..).collect::<Vec<_>>();
        let Some(synth) = engine.synth.as_mut() else {
            self.buffer.resize(RENDER_FRAMES * 2, 0.0);
            return;
        };

        for event in pending_notes {
            let channel = (0..16)
                .find(|channel| {
                    !self
                        .active_notes
                        .iter()
                        .any(|note| note.channel == *channel && note.midi_note == event.midi_note)
                })
                .unwrap_or(CHANNEL);
            synth.process_midi_message(channel, 0xC0, event.program as i32, 0);
            synth.note_on(channel, event.midi_note, event.velocity);
            if event.duration_samples == 0 {
                synth.note_off(channel, event.midi_note);
            } else {
                self.active_notes.push(ActiveNote {
                    channel,
                    midi_note: event.midi_note,
                    remaining_samples: event.duration_samples,
                });
            }
        }

        let mut rendered = 0;
        while rendered < RENDER_FRAMES {
            let mut active = 0;
            while active < self.active_notes.len() {
                if self.active_notes[active].remaining_samples == 0 {
                    let note = self.active_notes.swap_remove(active);
                    synth.note_off(note.channel, note.midi_note);
                } else {
                    active += 1;
                }
            }

            let frames = self
                .active_notes
                .iter()
                .map(|note| note.remaining_samples)
                .min()
                .unwrap_or((RENDER_FRAMES - rendered) as u64)
                .min((RENDER_FRAMES - rendered) as u64) as usize;
            synth.render(
                &mut self.left[rendered..rendered + frames],
                &mut self.right[rendered..rendered + frames],
            );
            for note in &mut self.active_notes {
                note.remaining_samples -= frames as u64;
            }
            rendered += frames;
        }

        for (&l, &r) in self.left.iter().zip(&self.right) {
            self.buffer.push(l);
            self.buffer.push(r);
        }
    }
}

impl Iterator for SynthSource {
    type Item = f32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.buffer.len() {
            self.refill_buffer();
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

    #[test]
    fn queued_notes_advance_and_release_on_the_sample_timeline() {
        let soundfont = crate::soundfont::loader::load_soundfont_from_bytes(include_bytes!(
            "../../assets/TimGM6mb.sf2"
        ))
        .unwrap();
        let soundfont = Arc::new(soundfont);
        let (synth, settings) =
            crate::soundfont::loader::make_synth(&soundfont, Timbre::AcousticGrandPiano).unwrap();
        let handle = SynthHandle::new(&settings);
        handle.install(synth);
        handle.queue_note(
            69,
            100,
            Timbre::AcousticGrandPiano,
            Duration::from_millis(500),
        );

        let mut source = handle.source();
        source.refill_buffer();

        assert_eq!(source.active_notes.len(), 1);
        assert_eq!(
            source.active_notes[0].remaining_samples,
            500 * 44_100 / 1000 - 256
        );
        assert_eq!(source.buffer.len(), 256 * 2);

        handle.queue_note(
            69,
            100,
            Timbre::AcousticGrandPiano,
            Duration::from_millis(500),
        );
        source.refill_buffer();

        assert_eq!(source.active_notes.len(), 2);
        assert_eq!(source.active_notes[0].channel, 0);
        assert_eq!(source.active_notes[1].channel, 1);
        assert_eq!(
            source.active_notes[0].remaining_samples,
            500 * 44_100 / 1000 - 512
        );

        handle.queue_note(
            69,
            100,
            Timbre::AcousticGrandPiano,
            Duration::from_millis(2),
        );
        source.refill_buffer();

        assert_eq!(source.active_notes.len(), 2);
        assert_eq!(
            source.active_notes[0].remaining_samples,
            500 * 44_100 / 1000 - 768
        );
        assert_eq!(
            source.active_notes[1].remaining_samples,
            500 * 44_100 / 1000 - 512
        );
    }
}
