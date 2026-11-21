#![doc = include_str!("./README.md")]

use crate::midi::{MidiRaw, event_const as e, midi_const as m};

/// This is a test to check if the detection function is normal.
pub const TEST: MidiRaw = m!(
    "Test",
    80.0,
    e!(0.0, [("C4", 1.0, BrightAcousticPiano), ("C5", 1.0, Violin)]),
    e!(1.0, [("D4", 1.0, BrightAcousticPiano), ("D5", 1.0, Violin)]),
    e!(1.0, [("E4", 1.0, BrightAcousticPiano), ("E5", 1.0, Violin)]),
    e!(1.0, [("F4", 1.0, BrightAcousticPiano), ("F5", 1.0, Violin)]),
    e!(1.0, [("G4", 1.0, BrightAcousticPiano), ("G5", 1.0, Violin)]),
    e!(1.0, [("A4", 1.0, BrightAcousticPiano), ("A5", 1.0, Violin)]),
    e!(1.0, [("B4", 1.0, BrightAcousticPiano), ("B5", 1.0, Violin)]),
    e!(1.0, [("C5", 1.0, BrightAcousticPiano), ("C6", 1.0, Violin)]),
    e!(1.0, [("C4", 0.5, MusicBox)]),
    e!(0.5, [("D4", 0.5, MusicBox)]),
    e!(0.5, [("E4", 0.5, MusicBox)]),
    e!(0.5, [("F4", 0.5, MusicBox)]),
    e!(0.5, [("G4", 0.5, MusicBox)]),
    e!(0.5, [("A4", 0.5, MusicBox)]),
    e!(0.5, [("B4", 0.5, MusicBox)]),
    e!(0.5, [("C5", 0.5, MusicBox)]),
);

/// Song: *Twinkle Twinkle Little Star*
pub const LITTLE_STAR: MidiRaw = m!(
    "Twinkle Twinkle Little Star",
    80.0,
    e!(0.0, [("C3", 1.0), ("C5", 1.0)]),
    e!(1.0, [("C4", 1.0), ("C5", 1.0)]),
    e!(1.0, [("E4", 1.0), ("G5", 1.0)]),
    e!(1.0, [("C4", 1.0), ("G5", 1.0)]),
    e!(1.0, [("F4", 1.0), ("A5", 1.0)]),
    e!(1.0, [("C4", 1.0), ("A5", 1.0)]),
    e!(1.0, [("E4", 1.0), ("G5", 2.0)]),
    e!(1.0, [("C4", 1.0)]),
    e!(1.0, [("D4", 1.0), ("F5", 1.0)]),
    e!(1.0, [("B3", 1.0), ("F5", 1.0)]),
    e!(1.0, [("C4", 1.0), ("E5", 1.0)]),
    e!(1.0, [("A3", 1.0), ("E5", 1.0)]),
    e!(1.0, [("F3", 1.0), ("D5", 1.0)]),
    e!(1.0, [("G3", 1.0), ("D5", 1.0)]),
    e!(1.0, [("C3", 2.0), ("C5", 2.0)]),
    e!(2.0, [("E4", 1.0), ("G5", 1.0)]),
    e!(1.0, [("G3", 1.0), ("G5", 1.0)]),
    e!(1.0, [("D4", 1.0), ("F5", 1.0)]),
    e!(1.0, [("G3", 1.0), ("F5", 1.0)]),
    e!(1.0, [("C4", 1.0), ("E5", 1.0)]),
    e!(1.0, [("G3", 1.0), ("E5", 1.0)]),
    e!(1.0, [("B3", 1.0), ("D5", 2.0)]),
    e!(1.0, [("G3", 1.0)]),
    e!(1.0, [("E4", 1.0), ("G5", 1.0)]),
    e!(1.0, [("G3", 1.0), ("G5", 1.0)]),
    e!(1.0, [("D4", 1.0), ("F5", 1.0)]),
    e!(1.0, [("G3", 1.0), ("F5", 1.0)]),
    e!(1.0, [("C4", 1.0), ("E5", 1.0)]),
    e!(1.0, [("G3", 1.0), ("E5", 1.0)]),
    e!(1.0, [("B3", 1.0), ("D5", 2.0)]),
    e!(1.0, [("G3", 1.0)]),
    e!(1.0, [("C3", 1.0), ("C5", 1.0)]),
    e!(1.0, [("C4", 1.0), ("C5", 1.0)]),
    e!(1.0, [("E4", 1.0), ("G5", 1.0)]),
    e!(1.0, [("C4", 1.0), ("G5", 1.0)]),
    e!(1.0, [("F4", 1.0), ("A5", 1.0)]),
    e!(1.0, [("C4", 1.0), ("A5", 1.0)]),
    e!(1.0, [("E4", 1.0), ("G5", 2.0)]),
    e!(1.0, [("C4", 1.0)]),
    e!(1.0, [("D4", 1.0), ("F5", 1.0)]),
    e!(1.0, [("B3", 1.0), ("F5", 1.0)]),
    e!(1.0, [("C4", 1.0), ("E5", 1.0)]),
    e!(1.0, [("A3", 1.0), ("E5", 1.0)]),
    e!(1.0, [("F3", 1.0), ("D5", 1.0)]),
    e!(1.0, [("G3", 1.0), ("D5", 1.0)]),
    e!(1.0, [("C3", 2.0), ("C5", 2.0)]),
);

/// Song: *Birthday Song*
///
/// Play this on your friend's birthday!
pub const BIRTHDAY_SONG: MidiRaw = m!(
    "Birthday Song",
    80.0,
    e!(0.0, [("G5", 0.7)]),
    e!(0.7, [("G5", 0.3)]),
    e!(0.3, [("A5", 1.0), ("C5", 1.0)]),
    e!(1.0, [("G5", 1.0), ("E5", 1.0), ("G5", 1.0)]),
    e!(1.0, [("C6", 1.0), ("E5", 1.0), ("G5", 1.0)]),
    e!(1.0, [("B5", 2.0), ("D5", 1.0)]),
    e!(1.0, [("F5", 2.0), ("G5", 1.0)]),
    e!(1.0, [("G5", 0.7), ("F5", 2.0), ("G5", 1.0)]),
    e!(0.7, [("G5", 0.3)]),
    e!(0.3, [("A5", 1.0), ("B4", 1.0)]),
    e!(1.0, [("G5", 1.0), ("F5", 2.0), ("G5", 1.0)]),
    e!(1.0, [("D6", 1.0), ("F5", 2.0), ("G5", 1.0)]),
    e!(1.0, [("C6", 2.0), ("C5", 1.0)]),
    e!(1.0, [("E5", 1.0), ("G5", 1.0)]),
    e!(1.0, [("G5", 0.7), ("E5", 1.0), ("G5", 1.0)]),
    e!(0.7, [("G5", 0.3)]),
    e!(0.3, [("G6", 1.0), ("E5", 1.0)]),
    e!(1.0, [("E6", 1.0), ("G5", 1.0), ("C6", 1.0)]),
    e!(1.0, [("C6", 0.7), ("G5", 1.0), ("C6", 1.0)]),
    e!(0.7, [("C6", 0.3)]),
    e!(0.3, [("B5", 1.0), ("D5", 1.0)]),
    e!(1.0, [("A5", 1.0), ("F5", 1.0)]),
    e!(1.0, [("F6", 0.7), ("A5", 1.0), ("D6", 1.0)]),
    e!(0.7, [("F6", 0.3)]),
    e!(0.3, [("E6", 1.0), ("E5", 2.0), ("G5", 2.0), ("C6", 2.0)]),
    e!(1.0, [("C6", 1.0)]),
    e!(1.0, [("D6", 1.0), ("B4", 1.0), ("F5", 1.0), ("G5", 1.0)]),
    e!(1.0, [("C6", 5.0), ("C5", 5.0), ("E5", 5.0), ("G5", 5.0)]),
);

/// Song: *Ode to Joy*
pub const ODE_TO_JOY: MidiRaw = m!(
    "Ode to Joy",
    114.0,
    e!(0.0, [("E5", 1.0, Violin), ("C4", 4.0, Violin)]),
    e!(1.0, [("E5", 1.0, Violin)]),
    e!(1.0, [("F5", 1.0, Violin)]),
    e!(1.0, [("G5", 1.0, Violin)]),
    e!(1.0, [("G5", 1.0, Violin), ("G4", 4.0, Violin)]),
    e!(1.0, [("F5", 1.0, Violin)]),
    e!(1.0, [("E5", 1.0, Violin)]),
    e!(1.0, [("D5", 1.0, Violin)]),
    e!(1.0, [("C5", 1.0, Violin), ("C4", 4.0, Violin)]),
    e!(1.0, [("C5", 1.0, Violin)]),
    e!(1.0, [("D5", 1.0, Violin)]),
    e!(1.0, [("E5", 1.0, Violin)]),
    e!(1.0, [("E5", 1.5, Violin), ("G4", 4.0, Violin)]),
    e!(1.5, [("D5", 0.5, Violin)]),
    e!(0.5, [("D5", 2.0, Violin)]),
    e!(2.0, [("E5", 1.0, Violin), ("C4", 4.0, Violin)]),
    e!(1.0, [("E5", 1.0, Violin)]),
    e!(1.0, [("F5", 1.0, Violin)]),
    e!(1.0, [("G5", 1.0, Violin)]),
    e!(1.0, [("G5", 1.0, Violin), ("G4", 4.0, Violin)]),
    e!(1.0, [("F5", 1.0, Violin)]),
    e!(1.0, [("E5", 1.0, Violin)]),
    e!(1.0, [("D5", 1.0, Violin)]),
    e!(1.0, [("C5", 1.0, Violin), ("C4", 4.0, Violin)]),
    e!(1.0, [("C5", 1.0, Violin)]),
    e!(1.0, [("D5", 1.0, Violin)]),
    e!(1.0, [("E5", 1.0, Violin)]),
    e!(1.0, [("D5", 1.5, Violin), ("C4", 4.0, Violin)]),
    e!(1.5, [("C5", 0.5, Violin)]),
    e!(0.5, [("C5", 2.0, Violin)]),
    e!(2.0, [("D5", 1.0, Violin), ("G4", 4.0, Violin)]),
    e!(1.0, [("D5", 1.0, Violin)]),
    e!(1.0, [("E5", 1.0, Violin)]),
    e!(1.0, [("C5", 1.0, Violin)]),
    e!(1.0, [("D5", 1.0, Violin), ("G4", 4.0, Violin)]),
    e!(1.0, [("E5", 0.5, Violin)]),
    e!(0.5, [("F5", 0.5, Violin)]),
    e!(0.5, [("E5", 1.0, Violin)]),
    e!(1.0, [("C5", 1.0, Violin)]),
    e!(1.0, [("D5", 1.0, Violin), ("G4", 4.0, Violin)]),
    e!(1.0, [("E5", 0.5, Violin)]),
    e!(0.5, [("F5", 0.5, Violin)]),
    e!(0.5, [("E5", 1.0, Violin)]),
    e!(1.0, [("D5", 1.0, Violin)]),
    e!(1.0, [("C5", 1.0, Violin), ("C4", 4.0, Violin)]),
    e!(1.0, [("D5", 1.0, Violin)]),
    e!(1.0, [("G4", 2.0, Violin)]),
    e!(2.0, [("E5", 1.0, Violin), ("C4", 4.0, Violin)]),
    e!(1.0, [("E5", 1.0, Violin)]),
    e!(1.0, [("F5", 1.0, Violin)]),
    e!(1.0, [("G5", 1.0, Violin)]),
    e!(1.0, [("G5", 1.0, Violin), ("G4", 4.0, Violin)]),
    e!(1.0, [("F5", 1.0, Violin)]),
    e!(1.0, [("E5", 1.0, Violin)]),
    e!(1.0, [("D5", 1.0, Violin)]),
    e!(1.0, [("C5", 1.0, Violin), ("C4", 4.0, Violin)]),
    e!(1.0, [("C5", 1.0, Violin)]),
    e!(1.0, [("D5", 1.0, Violin)]),
    e!(1.0, [("E5", 1.0, Violin)]),
    e!(1.0, [("D5", 1.5, Violin), ("C4", 4.0, Violin)]),
    e!(1.5, [("C5", 0.5, Violin)]),
    e!(0.5, [("C5", 2.0, Violin)]),
);

/// Song: *Jingle Bells*
pub const JINGLE_BELLS: MidiRaw = m!(
    "Jingle Bells",
    120.0,
    e!(0.0, [("E5", 1.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 2.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 2.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("G5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("C5", 1.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("D5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 4.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("C4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("F5", 1.0, MusicBox), ("D4", 1.0, MusicBox)]),
    e!(1.0, [("F5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("F5", 1.0, MusicBox), ("D4", 1.0, MusicBox)]),
    e!(1.0, [("F5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("F5", 1.0, MusicBox), ("D4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 2.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("D5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("D5", 1.0, MusicBox), ("E4", 1.0, MusicBox)]),
    e!(1.0, [("C5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("D5", 2.0, MusicBox), ("F4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("G5", 2.0, MusicBox), ("D4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 2.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 2.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("G5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("C5", 1.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("D5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 4.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("C4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("F5", 1.0, MusicBox), ("D4", 1.0, MusicBox)]),
    e!(1.0, [("F5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("F5", 1.0, MusicBox), ("D4", 1.0, MusicBox)]),
    e!(1.0, [("F5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("F5", 1.0, MusicBox), ("D4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("E5", 2.0, MusicBox), ("C4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("G5", 1.0, MusicBox), ("D4", 1.0, MusicBox)]),
    e!(1.0, [("G5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("F5", 1.0, MusicBox), ("F4", 1.0, MusicBox)]),
    e!(1.0, [("D5", 1.0, MusicBox), ("G4", 1.0, MusicBox)]),
    e!(1.0, [("C5", 4.0, MusicBox), ("E4", 1.0, MusicBox)]),
    e!(1.0, [("G4", 1.0, MusicBox)]),
    e!(1.0, [("C4", 2.0, MusicBox)]),
);

/// Song: *Silent Night*
pub const SILENT_NIGHT: MidiRaw = m!(
    "Slient Night",
    88.0,
    e!(
        0.0,
        [
            ("C5", 1.5, BrightAcousticPiano),
            ("E5", 1.5, BrightAcousticPiano),
            ("G5", 1.5, BrightAcousticPiano),
            ("C3", 0.5, BrightAcousticPiano),
            ("G5", 1.5, Flute),
        ]
    ),
    e!(0.5, [("G3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("C4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("A5", 0.5, BrightAcousticPiano),
            ("A5", 0.5, Flute),
            ("D4", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("G5", 1.0, BrightAcousticPiano),
            ("G5", 1.0, Flute),
            ("E5", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("G4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("E5", 3.0, BrightAcousticPiano),
            ("E5", 3.0, Flute),
            ("B2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("G3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("B3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("D4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("E4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("G4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("Bb4", 1.5, BrightAcousticPiano),
            ("D5", 1.5, BrightAcousticPiano),
            ("G5", 1.5, BrightAcousticPiano),
            ("G5", 1.5, Flute),
            ("Bb2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("G3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("Bb3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("A5", 0.5, BrightAcousticPiano),
            ("A5", 0.5, Flute),
            ("D4", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("G5", 1.0, BrightAcousticPiano),
            ("G5", 1.0, Flute),
            ("G4", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("B3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("Bb4", 3.0, BrightAcousticPiano),
            ("Db5", 3.0, BrightAcousticPiano),
            ("E5", 3.0, BrightAcousticPiano),
            ("E5", 3.0, Flute),
            ("A2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("E3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("A3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("C4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("E4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("G4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("F5", 2.0, BrightAcousticPiano),
            ("A5", 2.0, BrightAcousticPiano),
            ("D6", 2.0, BrightAcousticPiano),
            ("D6", 2.0, Flute),
            ("D2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("A2", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("D3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("E3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("F5", 1.0, BrightAcousticPiano),
            ("D6", 1.0, BrightAcousticPiano),
            ("D6", 1.0, Flute),
            ("F3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("A3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("F5", 3.0, BrightAcousticPiano),
            ("B5", 3.0, BrightAcousticPiano),
            ("B5", 3.0, Flute),
            ("G2", 0.5, BrightAcousticPiano),
        ]
    ),
    e!(0.5, [("D3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("G3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("A3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("B3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("D4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("E5", 2.0, BrightAcousticPiano),
            ("G5", 2.0, BrightAcousticPiano),
            ("C6", 2.0, BrightAcousticPiano),
            ("C6", 2.0, Flute),
            ("A2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("E3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("A3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("B3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("E5", 1.0, BrightAcousticPiano),
            ("C6", 1.0, BrightAcousticPiano),
            ("C6", 1.0, Flute),
            ("C4", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("E4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("D5", 3.0, BrightAcousticPiano),
            ("G5", 3.0, BrightAcousticPiano),
            ("G5", 3.0, Flute),
            ("B2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("G3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("A3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("B3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("D4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("B3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("E5", 2.0, BrightAcousticPiano),
            ("A5", 2.0, BrightAcousticPiano),
            ("A5", 2.0, Flute),
            ("F2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("C3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("F3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("G3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("E5", 1.0, BrightAcousticPiano),
            ("A6", 1.0, BrightAcousticPiano),
            ("A6", 1.0, Flute),
            ("A3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("C4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("E5", 1.5, BrightAcousticPiano),
            ("G5", 1.5, BrightAcousticPiano),
            ("C6", 1.5, BrightAcousticPiano),
            ("C6", 1.5, Flute),
            ("G2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("D3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("G3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("B5", 0.5, BrightAcousticPiano),
            ("B5", 0.5, Flute),
            ("A3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("A5", 1.0, BrightAcousticPiano),
            ("A5", 1.0, Flute),
            ("B3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("D4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("E5", 1.5, BrightAcousticPiano),
            ("G5", 1.5, BrightAcousticPiano),
            ("G5", 1.5, Flute),
            ("E2", 0.5, BrightAcousticPiano),
        ]
    ),
    e!(0.5, [("B2", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("E3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("A5", 0.5, BrightAcousticPiano),
            ("A5", 0.5, Flute),
            ("G3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("G5", 1.0, BrightAcousticPiano),
            ("G5", 1.0, Flute),
            ("B3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("D4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("E5", 3.0, BrightAcousticPiano),
            ("E5", 3.0, Flute),
            ("A2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("E3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("A3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("B3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("C4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("E4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("A5", 0.5, BrightAcousticPiano),
            ("A5", 0.5, Flute),
            ("D2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("E5", 0.5, BrightAcousticPiano),
            ("E5", 0.5, Flute),
            ("F2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("A5", 0.5, BrightAcousticPiano),
            ("A5", 0.5, Flute),
            ("D3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("E5", 0.5, BrightAcousticPiano),
            ("E5", 0.5, Flute),
            ("E3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("A5", 0.5, BrightAcousticPiano),
            ("A5", 0.5, Flute),
            ("F3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("E5", 0.5, BrightAcousticPiano),
            ("E5", 0.5, Flute),
            ("A3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("C6", 0.5, BrightAcousticPiano),
            ("C6", 0.5, Flute),
            ("F2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("E5", 0.5, BrightAcousticPiano),
            ("E5", 0.5, Flute),
            ("C3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("G5", 0.5, BrightAcousticPiano),
            ("G5", 0.5, Flute),
            ("F3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("B5", 0.5, BrightAcousticPiano),
            ("B5", 0.5, Flute),
            ("G3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("A5", 1.0, BrightAcousticPiano),
            ("A5", 1.0, Flute),
            ("A3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("C4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("G5", 0.5, BrightAcousticPiano),
            ("G5", 0.5, Flute),
            ("E2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("D5", 0.5, BrightAcousticPiano),
            ("D5", 0.5, Flute),
            ("B2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("E5", 0.5, BrightAcousticPiano),
            ("E5", 0.5, Flute),
            ("E3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("A5", 0.5, BrightAcousticPiano),
            ("A5", 0.5, Flute),
            ("G3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(
        0.5,
        [
            ("G5", 1.0, BrightAcousticPiano),
            ("G5", 1.0, Flute),
            ("B3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("D4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("E5", 3.0, BrightAcousticPiano),
            ("E5", 3.0, Flute),
            ("A2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("E3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("A3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("B3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("C4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("E4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("F5", 2.0, BrightAcousticPiano),
            ("A5", 2.0, BrightAcousticPiano),
            ("D6", 2.0, BrightAcousticPiano),
            ("D6", 2.0, Flute),
            ("D2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("A2", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("D3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("E3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("F3", 0.5, BrightAcousticPiano),
            ("F5", 1.0, BrightAcousticPiano),
            ("D6", 1.0, BrightAcousticPiano),
            ("D6", 1.0, Flute),
        ]
    ),
    e!(0.5, [("A3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("F5", 1.5, BrightAcousticPiano),
            ("F6", 1.5, BrightAcousticPiano),
            ("F6", 1.5, Flute),
            ("G2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("D4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("G4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("A4", 0.5, BrightAcousticPiano),
            ("E6", 0.5, BrightAcousticPiano),
            ("E6", 0.5, Flute)
        ]
    ),
    e!(
        0.5,
        [
            ("D6", 1.0, BrightAcousticPiano),
            ("D6", 1.0, Flute),
            ("B3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("D4", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("E5", 3.0, BrightAcousticPiano),
            ("C6", 3.0, BrightAcousticPiano),
            ("C6", 3.0, Flute),
            ("C3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("G3", 3.0, BrightAcousticPiano)]),
    e!(0.5, [("C4", 3.0, BrightAcousticPiano)]),
    e!(0.5, [("D4", 3.0, BrightAcousticPiano)]),
    e!(0.5, [("E4", 3.0, BrightAcousticPiano)]),
    e!(0.5, [("G4", 3.0, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("E3", 3.0, BrightAcousticPiano),
            ("D4", 3.0, BrightAcousticPiano),
            ("D5", 3.0, BrightAcousticPiano),
            ("Bb5", 3.0, BrightAcousticPiano),
            ("E6", 3.0, BrightAcousticPiano),
            ("E6", 3.0, Flute)
        ]
    ),
    e!(
        3.0,
        [
            ("E5", 1.5, BrightAcousticPiano),
            ("C6", 1.5, BrightAcousticPiano),
            ("C6", 1.5, Flute),
            ("G2", 3.0, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("E3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("G3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("C4", 0.5, BrightAcousticPiano),
            ("G5", 0.5, BrightAcousticPiano),
            ("G5", 0.5, Flute)
        ]
    ),
    e!(
        0.5,
        [
            ("G3", 0.5, BrightAcousticPiano),
            ("E5", 1.0, BrightAcousticPiano),
            ("E5", 1.0, Flute)
        ]
    ),
    e!(0.5, [("E3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("B4", 1.5, BrightAcousticPiano),
            ("G5", 1.5, BrightAcousticPiano),
            ("G5", 1.5, Flute),
            ("G2", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("D3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("G3", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("A3", 0.5, BrightAcousticPiano),
            ("F5", 0.5, BrightAcousticPiano),
            ("F5", 0.5, Flute)
        ]
    ),
    e!(
        0.5,
        [
            ("B3", 1.0, BrightAcousticPiano),
            ("D5", 1.0, BrightAcousticPiano),
            ("D5", 1.0, Flute)
        ]
    ),
    e!(
        1.0,
        [
            ("C5", 3.0, BrightAcousticPiano),
            ("C5", 3.0, Flute),
            ("C3", 0.5, BrightAcousticPiano)
        ]
    ),
    e!(0.5, [("G3", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("C4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("D4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("E4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("G4", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("C5", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("D5", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("E5", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("G5", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("C6", 0.5, BrightAcousticPiano)]),
    e!(0.5, [("D6", 0.5, BrightAcousticPiano)]),
    e!(
        0.5,
        [
            ("C5", 6.0, BrightAcousticPiano),
            ("G5", 6.0, BrightAcousticPiano),
            ("C6", 6.0, BrightAcousticPiano),
            ("D6", 6.0, BrightAcousticPiano),
            ("E6", 6.0, BrightAcousticPiano),
            ("G6", 6.0, BrightAcousticPiano),
            ("C7", 6.0, BrightAcousticPiano),
            ("C7", 6.0, Flute)
        ]
    ),
);
