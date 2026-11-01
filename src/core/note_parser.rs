//! Parser for the musical note.

pub(crate) fn note2freq(note: &str) -> f32 {
    let note = note.trim().to_ascii_uppercase(); // All uppercase
    let mut chars = note.chars().peekable();
    let base = chars
        .next()
        .expect("Invalid note format: unexpected end while parsing base");
    let mut sharp = false;
    if chars.peek() == Some(&'#') || chars.peek() == Some(&'B') {
        let c = chars
            .next()
            .expect("Invalid note format: unexpected end while parsing accidental");
        if c == '#' {
            sharp = true;
        } else if c == 'B' {
            sharp = false;
        }
    }
    let oct_str: String = chars.collect();
    let octave: i32 = oct_str
        .parse()
        .expect("Invalid note format: failed to parse oectave");
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
    let mut n = semitone;
    if sharp {
        n += 1;
    }
    let total = n + (octave - 4) * 12 - 9;

    440.0 * 2.0f32.powf(total as f32 / 12.0)
}
