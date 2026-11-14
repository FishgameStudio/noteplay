//! Parser for the musical note.

use crate::core::types::Semitones;

use super::Frequency;

/// Parse note and give a frequency.
/// # Panics
/// If the format of the note is invalid.
pub fn note2freq(note: &str) -> Frequency {
    let note = note.trim().to_ascii_uppercase(); // All uppercase
    let mut chars = note.chars().peekable();
    let base = chars
        .next()
        .expect("Invalid note format: unexpected end while parsing base");
    let mut alter = 0;
    if chars.peek() == Some(&'#') || chars.peek() == Some(&'B') {
        let c = chars
            .next()
            .expect("Invalid note format: unexpected end while parsing accidental");
        if c == '#' {
            alter = 1;
        } else if c == 'B' {
            alter = -1;
        }
    }
    let oct_str: String = chars.collect();
    let octave: i32 = oct_str
        .parse()
        .expect("Invalid note format: failed to parse octave");
    let semitone = match base {
        'C' => 0,
        'D' => 2,
        'E' => 4,
        'F' => 5,
        'G' => 7,
        'A' => 9,
        'B' => 11,
        semitone => panic!("Invalid note format: unknown semitone: {semitone}"),
    };
    let n = semitone + alter;
    let total = n + (octave - 4) * 12 - 9;

    Frequency::new_unchecked(440.0 * ((total as f32) / 12.0).exp2())
}

/// Parse scientific pitch notation ("C4", "A#4", "Db5") into semitone offset (A4 = 0).
/// Stable const fn, no floating point approximation.
///
/// # Panics
/// Invalid byte format, unknown base note, bad octave digit.
pub const fn note2semi(note: &str) -> Semitones {
    let b = note.as_bytes();
    let len = b.len();
    assert!(len >= 2, "note string too short");

    let mut idx = 0;
    let base_byte = b[idx];
    idx += 1;

    let base_semi = match base_byte {
        b'C' | b'c' => 0,
        b'D' | b'd' => 2,
        b'E' | b'e' => 4,
        b'F' | b'f' => 5,
        b'G' | b'g' => 7,
        b'A' | b'a' => 9,
        b'B' | b'b' => 11,
        _ => panic!("unknown note base char"),
    };

    let mut alter = 0;
    if idx < len {
        match b[idx] {
            b'#' => {
                alter = 1;
                idx += 1;
            }
            b'b' | b'B' => {
                alter = -1;
                idx += 1;
            }
            _ => {}
        }
    }

    let mut octave = 0;
    while idx < len {
        let digit = b[idx];
        assert!(digit >= b'0' && digit <= b'9', "invalid octave digit");
        octave = octave * 10 + (digit - b'0') as i32;
        idx += 1;
    }

    (base_semi + alter) + (octave - 4) * 12 - 9
}

/// Runtime: convert semi offset(A4=0) to exact frequency Hz
pub fn semi2freq(semi: Semitones) -> Frequency {
    Frequency::new_unchecked(440.0 * (semi as f32 / 12.0).exp2())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
        (a - b).abs() < eps
    }

    #[test]
    fn test_note2freq() {
        const EPS: f32 = 0.2;
        assert!(approx_eq(note2freq("A4").get_freq(), 440.00, EPS));
        assert!(approx_eq(note2freq("C5").get_freq(), 523.2511, EPS));
        assert!(approx_eq(note2freq("G4").get_freq(), 391.9954, EPS));
        assert!(approx_eq(note2freq("B6").get_freq(), 1975.5332, EPS));
        assert!(approx_eq(note2freq("E4").get_freq(), 329.6276, EPS));
        assert!(approx_eq(note2freq("Eb4").get_freq(), 311.127, EPS));
        assert!(approx_eq(note2freq("G#5").get_freq(), 830.6094, EPS));
    }

    #[test]
    #[should_panic(expected = "Invalid note format: unexpected end while parsing base")]
    fn note2freq_should_panic_if_empty() {
        note2freq("");
    }

    #[test]
    #[should_panic(expected = "Invalid note format: unknown semitone: 1")]
    fn note2freq_should_panic_if_format_is_invalid() {
        note2freq("123123");
    }

    #[test]
    #[should_panic(expected = "Invalid note format: unknown semitone: K")]
    fn note2freq_should_panic_if_base_is_unknown() {
        note2freq("K4");
    }
}
