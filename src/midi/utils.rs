//! Helpful utilities to construct your `QMid` file.

use crate::Error;
use crate::core::{Frequency, note2semi};
use crate::midi::types::{Event, Midi, Note};
use crate::soundfont::Timbre;

/// Parse chords and generate a corresponding [`Event`] that stores the chord.
///
/// Supported marks: maj / m / dim / aug / sus2 / sus4 / maj7 / m7 / m7b5 / maj9 / m9 / add2 / add4 / add9
///
/// Extension sounds: 7,6,9,11,13; variation sounds: b5 #5 b9 #9 #11 b13
///
/// # Panics
/// If used a 'm' after a 'sus'.
///
/// # Examples
/// ```
/// # use noteplay::midi::parse_chord;
/// # use noteplay::soundfont::Timbre;
/// # fn main() {
/// let c_maj = parse_chord("C", 4, 1.0, Timbre::Default, 100).unwrap();
/// assert_eq!(c_maj.notes.len(), 3);
/// let cm7b5 = parse_chord("Cm7b5", 4, 1.0, Timbre::Default, 100).unwrap();
/// assert_eq!(cm7b5.notes.len(), 4);
/// # }
/// ```
///
/// ```should_panic
/// # use noteplay::midi::parse_chord;
/// # use noteplay::soundfont::Timbre;
/// # fn main() {
/// // Cannot use a 'm' after a 'sus'
/// let _ = parse_chord("Csus4m", 4, 1.0, Timbre::Default, 100).unwrap(); // <--- PANICS
/// # }
/// ```
pub fn parse_chord(
    chord: &str,
    root_octave: i32,
    dur_beat: f32,
    timbre: Timbre,
    velocity: i32,
) -> Result<Event, Error> {
    if chord.is_empty() {
        return Err(Error::InvalidChord("Chord is empty".into()));
    }
    let chord = chord.trim().to_ascii_uppercase();
    let mut it = chord.chars().peekable();

    // Base
    let base = it
        .next()
        .ok_or_else(|| Error::InvalidChord("Unknown root".into()))?;
    if !matches!(base, 'C' | 'D' | 'E' | 'F' | 'G' | 'A' | 'B') {
        return Err(Error::InvalidChord(format!("Unknown chord root: {base}")));
    }

    let mut acc = "";
    let mut offset = 1;
    if let Some(&c) = it.peek()
        && (c == '#' || c == 'B')
    {
        acc = &chord[offset..offset + 1];
        offset += 1;
    }

    let root_note = format!("{}{}{}", base, acc, root_octave);
    let root_semi = note2semi(&root_note);
    let mut suffix = &chord[offset..];

    let mut tones = vec![0, 4, 7]; // Default major
    let mut is_sus = false;

    while !suffix.is_empty() {
        if suffix.starts_with("MAJ7") {
            tones.push(11);
            suffix = &suffix[4..];
        } else if suffix.starts_with("MAJ9") {
            tones.push(11);
            tones.push(14);
            suffix = &suffix[4..];
        } else if suffix.starts_with("M7b5") {
            if is_sus {
                return Err(Error::InvalidChord("sus chord cannot have m7b5".into()));
            }
            tones = vec![0, 3, 6, 10];
            suffix = &suffix[4..];
        } else if suffix.starts_with("MADD9") {
            if is_sus {
                return Err(Error::InvalidChord("sus chord cannot be madd9".into()));
            }
            tones = vec![0, 3, 7];
            tones.push(14);
            suffix = &suffix[5..];
        } else if suffix.starts_with("SUS4") {
            tones = vec![0, 5, 7];
            is_sus = true;
            suffix = &suffix[4..];
        } else if suffix.starts_with("SUS2") {
            tones = vec![0, 2, 7];
            is_sus = true;
            suffix = &suffix[4..];
        } else if suffix.starts_with("ADD9") {
            tones.push(14);
            suffix = &suffix[4..];
        } else if suffix.starts_with("ADD4") {
            tones.push(5);
            suffix = &suffix[4..];
        } else if suffix.starts_with("ADD2") {
            tones.push(2);
            suffix = &suffix[4..];
        } else if suffix.starts_with("B13") {
            tones.push(20);
            suffix = &suffix[3..];
        } else if suffix.starts_with("#11") {
            tones.push(18);
            suffix = &suffix[3..];
        } else if suffix.starts_with("B5") {
            if let Some(p) = tones.iter().position(|&x| x == 7) {
                tones[p] = 6;
            }
            suffix = &suffix[2..];
        } else if suffix.starts_with("#5") {
            if let Some(p) = tones.iter().position(|&x| x == 7) {
                tones[p] = 8;
            }
            suffix = &suffix[2..];
        } else if suffix.starts_with("B9") {
            tones.push(13);
            suffix = &suffix[2..];
        } else if suffix.starts_with("#9") {
            tones.push(15);
            suffix = &suffix[2..];
        } else if suffix.starts_with("M7") {
            if is_sus {
                return Err(Error::InvalidChord("sus chord cannot be m7".into()));
            }
            tones = vec![0, 3, 7, 10];
            suffix = &suffix[2..];
        } else if suffix.starts_with("M9") {
            if is_sus {
                return Err(Error::InvalidChord("sus chord cannot be m9".into()));
            }
            tones = vec![0, 3, 7, 10, 14];
            suffix = &suffix[2..];
        } else if suffix.starts_with("MAJ") {
            if is_sus {
                return Err(Error::InvalidChord("sus chord cannot have maj".into()));
            }
            tones = vec![0, 4, 7];
            suffix = &suffix[3..];
        } else if suffix.starts_with("DIM7") {
            if is_sus {
                return Err(Error::InvalidChord("sus chord cannot be dim7".into()));
            }
            tones = vec![0, 3, 6, 9];
            suffix = &suffix[4..];
        } else if suffix.starts_with("DIM") {
            if is_sus {
                return Err(Error::InvalidChord("sus chord cannot be dim".into()));
            }
            tones = vec![0, 3, 6];
            suffix = &suffix[3..];
        } else if suffix.starts_with("AUG") {
            if is_sus {
                return Err(Error::InvalidChord("sus chord cannot be aug".into()));
            }
            tones = vec![0, 4, 8];
            suffix = &suffix[3..];
        } else if suffix.starts_with("13") {
            tones.push(21);
            suffix = &suffix[2..];
        } else if suffix.starts_with("11") {
            tones.push(17);
            suffix = &suffix[2..];
        } else if suffix.starts_with("7") {
            tones.push(10);
            suffix = &suffix[1..];
        } else if suffix.starts_with("6") {
            tones.push(9);
            suffix = &suffix[1..];
        } else if suffix.starts_with("9") {
            tones.push(14);
            suffix = &suffix[1..];
        } else if suffix.starts_with("M") {
            if is_sus {
                return Err(Error::InvalidChord("sus chord cannot be minor".into()));
            }
            tones = vec![0, 3, 7];
            suffix = &suffix[1..];
        } else {
            return Err(Error::InvalidChord(format!(
                "Unknown chord suffix: {suffix}"
            )));
        }
    }

    let mut out: Vec<Note> = Vec::new();
    for off in tones {
        let t = (root_semi + off) as f32 / 12.0;
        let f = 440.0 * t.exp2();
        out.push((Frequency::new_unchecked(f), dur_beat, timbre, velocity));
    }

    Ok(Event::with_notes(out))
}

/// Play .qmid files.
pub fn play_qmid_file<P>(path: P, sf2_bytes: Vec<u8>) -> Result<(), Error>
where
    P: AsRef<std::path::Path>,
{
    let bytes: Vec<u8> = std::fs::read(path)?;
    let sf = crate::soundfont::loader::load_soundfont_from_bytes(&sf2_bytes)?;
    let midi = crate::midi::deserializer::deserialize(bytes)?;
    let mut np = crate::core::note_player::NotePlayer::new()?;
    np.set_sf(sf)?;
    crate::midi::player::MidiPlayer::play(&midi, &mut np);
    Ok(())
}
/// Get metadata from .qmid files.
pub fn get_metadata<P>(path: P) -> Result<crate::midi::MidiMeta, Error>
where
    P: AsRef<std::path::Path>,
{
    let bytes: Vec<u8> = std::fs::read(path)?;
    let midi = crate::midi::deserializer::deserialize(bytes)?;
    Ok(midi.meta)
}

/// Merge two midi as one midi, the audio track will be stacked.
/// # Errors
/// If the tempo or meta is not same.
pub fn merge_two_midi(a: &Midi, b: &Midi) -> Result<Midi, Error> {
    #[inline]
    fn error(msg: &str) -> Result<Midi, Error> {
        Err(Error::MergeMidi(msg.into()))
    }

    if a.tempo != b.tempo {
        return error("tempos are different");
    }
    if a.meta != b.meta {
        return error("metadata are different");
    }

    /// Convert delta event list -> (`absolute_beat`, `notes`)
    fn to_absolute(events: &[Event]) -> Vec<(f32, Vec<Note>)> {
        let mut out = Vec::new();
        let mut now = 0.0;
        for ev in events {
            now += ev.delta_beat;
            out.push((now, ev.notes.clone()));
        }
        out
    }

    let abs_a = to_absolute(&a.events);
    let abs_b = to_absolute(&b.events);

    // Merge two sorted lists (merge sort)
    let mut i = 0;
    let mut j = 0;
    let mut merged_abs = Vec::new();
    while i < abs_a.len() && j < abs_b.len() {
        let (ta, na) = &abs_a[i];
        let (tb, nb) = &abs_b[j];

        if *ta < *tb {
            merged_abs.push((*ta, na.clone()));
            i += 1;
        } else if *tb < *ta {
            merged_abs.push((*tb, nb.clone()));
            j += 1;
        } else {
            // same time: stack notes together
            let mut all = na.clone();
            all.extend(nb.clone());
            merged_abs.push((*ta, all));
            i += 1;
            j += 1;
        }
    }
    while i < abs_a.len() {
        merged_abs.push(abs_a[i].clone());
        i += 1;
    }
    while j < abs_b.len() {
        merged_abs.push(abs_b[j].clone());
        j += 1;
    }

    // Convert absolute list back to delta-beat Event stream
    let mut events = Vec::new();
    let mut prev = 0.0;
    for (t, notes) in merged_abs {
        let delta = t - prev;
        events.push(Event {
            delta_beat: delta,
            notes,
        });
        prev = t;
    }

    Ok(Midi {
        meta: a.meta,
        tempo: a.tempo,
        events,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Error;

    #[test]
    fn test_parse_chord_basic() {
        // C major
        let ev = parse_chord("C", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 3);

        // Cm minor
        let ev = parse_chord("Cm", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 3);

        // Csus4
        let ev = parse_chord("Csus4", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 3);

        // Csus2
        let ev = parse_chord("Csus2", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 3);

        // Cmaj7
        let ev = parse_chord("Cmaj7", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 4);

        // Cm7b5
        let ev = parse_chord("Cm7b5", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 4);

        // C7
        let ev = parse_chord("C7", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 4);

        // Cadd9
        let ev = parse_chord("Cadd9", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 4);

        // Cdim
        let ev = parse_chord("Cdim", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 3);

        // Caug
        let ev = parse_chord("Caug", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 3);

        // C#maj7 sharp root
        let ev = parse_chord("C#maj7", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 4);

        // CBmaj7 flat root
        let ev = parse_chord("CBmaj7", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 4);
    }

    #[test]
    fn test_parse_chord_extended() {
        // Cmaj9
        let ev = parse_chord("Cmaj9", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 5);

        // C7b9
        let ev = parse_chord("C7b9", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 5);

        // C7#9
        let ev = parse_chord("C7#9", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 5);

        // C13
        let ev = parse_chord("C13", 4, 1.0, Timbre::Default, 90).unwrap();
        assert_eq!(ev.notes.len(), 4);
    }

    #[test]
    fn test_empty_chord() {
        let res = parse_chord("", 4, 1.0, Timbre::Default, 90);
        assert!(matches!(res, Err(Error::InvalidChord(_))));
    }

    #[test]
    fn test_sus_cannot_follow_minor() {
        let res = parse_chord("Csus4m", 4, 1.0, Timbre::Default, 90);
        assert!(
            matches!(res, Err(Error::InvalidChord(msg)) if msg.contains("sus chord cannot be minor"))
        );
    }

    #[test]
    fn test_unknown_suffix() {
        let res = parse_chord("Cxyz", 4, 1.0, Timbre::Default, 90);
        assert!(
            matches!(res, Err(Error::InvalidChord(msg)) if msg.contains("Unknown chord suffix"))
        );
    }

    use crate::core::types::{Frequency, Tempo};
    use crate::midi::types::{Event, Midi, Note};
    use crate::soundfont::types::Timbre;

    fn note_test(hz: f32) -> Note {
        (Frequency::new_unchecked(hz), 0.25, Timbre::Default, 80)
    }

    fn midi_test(tempo_bpm: f32, events: Vec<Event>) -> Midi {
        Midi {
            tempo: Tempo::new_unchecked(tempo_bpm),
            events,
            meta: crate::midi::types::DEFAULT_META,
        }
    }

    #[test]
    fn merge_same_time_stack_notes() {
        let note1 = note_test(440.0);
        let note2 = note_test(523.25);

        let a = midi_test(
            120.0,
            vec![Event {
                delta_beat: 1.0,
                notes: vec![note1],
            }],
        );
        let b = midi_test(
            120.0,
            vec![Event {
                delta_beat: 1.0,
                notes: vec![note2],
            }],
        );

        let out = merge_two_midi(&a, &b).unwrap();
        assert_eq!(out.events.len(), 1);
        let ev = &out.events[0];
        assert!((ev.delta_beat - 1.0).abs() < 1e-5);
        assert_eq!(ev.notes.len(), 2);
    }

    #[test]
    fn merge_events_in_sequence() {
        let n1 = note_test(440.0);
        let n2 = note_test(493.88);
        let n3 = note_test(523.25);

        let a = midi_test(
            120.0,
            vec![
                Event {
                    delta_beat: 1.0,
                    notes: vec![n1],
                },
                Event {
                    delta_beat: 1.0,
                    notes: vec![n2],
                },
            ],
        );
        let b = midi_test(
            120.0,
            vec![Event {
                delta_beat: 1.5,
                notes: vec![n3],
            }],
        );

        let out = merge_two_midi(&a, &b).unwrap();
        assert_eq!(out.events.len(), 3);
        // times: 1.0, 1.5, 2.0
    }

    #[test]
    fn merge_one_empty() {
        let n = note_test(440.0);
        let a = midi_test(
            120.0,
            vec![Event {
                delta_beat: 2.0,
                notes: vec![n],
            }],
        );
        let b = midi_test(120.0, Vec::new());

        let out = merge_two_midi(&a, &b).unwrap();
        assert_eq!(out.events.len(), 1);
        assert!((out.events[0].delta_beat - 2.0).abs() < 1e-5);
    }

    #[test]
    fn reject_different_tempo() {
        let a = midi_test(120.0, Vec::new());
        let b = midi_test(140.0, Vec::new());
        let res = merge_two_midi(&a, &b);
        assert!(res.is_err());
    }

    #[test]
    fn reject_different_meta() {
        let a = midi_test(120.0, Vec::new());
        let mut b = midi_test(120.0, Vec::new());
        b.meta.song_name = Some(crate::midi::fixed_string::FixedString::from_str_const(
            "test",
        ));
        let res = merge_two_midi(&a, &b);
        assert!(res.is_err());
    }
}
