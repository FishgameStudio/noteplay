//! Parser for the musical note.

pub(crate) fn note2freq(note: &str) -> f32 {
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

    440.0 * 2.0f32.powf(total as f32 / 12.0)
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
        assert!(approx_eq(note2freq("A4"), 440.00, EPS));
        assert!(approx_eq(note2freq("C5"), 523.2511, EPS));
        assert!(approx_eq(note2freq("G4"), 391.9954, EPS));
        assert!(approx_eq(note2freq("B6"), 1975.5332, EPS));
        assert!(approx_eq(note2freq("E4"), 329.6276, EPS));
        assert!(approx_eq(note2freq("Eb4"), 311.127, EPS));
        assert!(approx_eq(note2freq("G#5"), 830.6094, EPS));
    }

    #[test]
    #[should_panic(expected = "Invalid note format: unexpected end while parsing base")]
    fn test_note2freq_panic1() {
        note2freq("");
    }

    #[test]
    #[should_panic(expected = "Invalid note format: unknown semitone: 1")]
    fn test_note2freq_panic2() {
        note2freq("123123");
    }

    #[test]
    #[should_panic(expected = "Invalid note format: unknown semitone: K")]
    fn test_note2freq_panic3() {
        note2freq("K4");
    }
}
