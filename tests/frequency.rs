use noteplay::prelude::*;

fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

#[test]
fn note2freq_converts_common_notes() {
    const EPS: f32 = 0.2;

    assert!(approx_eq(note2freq("A4").get_freq(), 440.0, EPS));
    assert!(approx_eq(note2freq("C5").get_freq(), 523.2511, EPS));
    assert!(approx_eq(note2freq("G4").get_freq(), 391.9954, EPS));
    assert!(approx_eq(note2freq("B6").get_freq(), 1975.5332, EPS));
    assert!(approx_eq(note2freq("E4").get_freq(), 329.6276, EPS));
    assert!(approx_eq(note2freq("Eb4").get_freq(), 311.127, EPS));
    assert!(approx_eq(note2freq("G#5").get_freq(), 830.6094, EPS));
}

#[test]
fn note2semi_converts_scientific_pitch_notation() {
    assert_eq!(note2semi("C4"), -9);
    assert_eq!(note2semi("A4"), 0);
    assert_eq!(note2semi("G#5"), 11);
    assert_eq!(note2semi("Db5"), 4);
    assert_eq!(note2semi("B7"), 38);
}

#[test]
fn semi2freq_matches_expected_note_frequencies() {
    const EPS: f32 = 0.1;

    assert!(approx_eq(semi2freq(0).get_freq(), 440.0, EPS));
    assert!(approx_eq(semi2freq(9).get_freq(), 739.9889, EPS));
    assert!(approx_eq(semi2freq(12).get_freq(), 880.0, EPS));
    assert!(approx_eq(semi2freq(-12).get_freq(), 220.0, EPS));
}

#[test]
fn is_valid_note_accepts_valid_notes_and_rejects_invalid_ones() {
    assert!(is_valid_note("C4"));
    assert!(is_valid_note("A#4"));
    assert!(is_valid_note("Db5"));
    assert!(is_valid_note("E8"));

    assert!(!is_valid_note("L5"));
    assert!(!is_valid_note("K"));
    assert!(!is_valid_note("C"));
    assert!(!is_valid_note("abc"));
}

#[test]
#[should_panic(expected = "Invalid note format: unexpected end while parsing base")]
fn note2freq_panics_if_empty() {
    note2freq("");
}

#[test]
#[should_panic(expected = "Invalid note format: unknown semitone: K")]
fn note2freq_panics_if_base_is_unknown() {
    note2freq("K4");
}
