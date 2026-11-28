use noteplay::Error;
use noteplay::midi::parse_chord;
use noteplay::soundfont::Timbre;

#[test]
fn parse_chord_builds_expected_major_and_minor_triads() {
    let major = parse_chord("C", 4, 1.0, Timbre::Default, 90)
        .unwrap()
        .get_normal_unchecked();
    assert_eq!(major.notes.len(), 3);
    assert_eq!(major.notes[0].2, Timbre::Default);
    assert_eq!(major.notes[0].3, 90);

    let minor = parse_chord("Am", 5, 0.75, Timbre::Cello, 80)
        .unwrap()
        .get_normal_unchecked();
    assert_eq!(minor.notes.len(), 3);
    assert_eq!(minor.notes[0].2, Timbre::Cello);
    assert_eq!(minor.notes[0].3, 80);
    assert!((minor.notes[0].0.get_freq() - 880.0).abs() < 1.0);
}

#[test]
fn parse_chord_handles_extended_and_altered_variants() {
    assert_eq!(
        parse_chord("Csus2", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        3
    );
    assert_eq!(
        parse_chord("Csus4", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        3
    );
    assert_eq!(
        parse_chord("Cmaj7", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        4
    );
    assert_eq!(
        parse_chord("Cm7b5", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        4
    );
    assert_eq!(
        parse_chord("C7", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        4
    );
    assert_eq!(
        parse_chord("Cadd9", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        4
    );
    assert_eq!(
        parse_chord("Cdim", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        3
    );
    assert_eq!(
        parse_chord("Caug", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        3
    );
    assert_eq!(
        parse_chord("Cmaj9", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        5
    );
    assert_eq!(
        parse_chord("C7b9", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        5
    );
    assert_eq!(
        parse_chord("C13", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        4
    );
    assert_eq!(
        parse_chord("C#maj7", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        4
    );
    assert_eq!(
        parse_chord("CBmaj7", 4, 1.0, Timbre::Default, 90)
            .unwrap()
            .get_normal_unchecked()
            .notes
            .len(),
        4
    );
}

#[test]
fn parse_chord_rejects_invalid_inputs() {
    let empty = parse_chord("", 4, 1.0, Timbre::Default, 90);
    assert!(matches!(empty, Err(Error::InvalidChord(_))));

    let bad_root = parse_chord("Xmaj7", 4, 1.0, Timbre::Default, 90);
    assert!(matches!(bad_root, Err(Error::InvalidChord(_))));

    let bad_suffix = parse_chord("Cxyz", 4, 1.0, Timbre::Default, 90);
    assert!(
        matches!(bad_suffix, Err(Error::InvalidChord(msg)) if msg.contains("Unknown chord suffix"))
    );

    let invalid_sus = parse_chord("Csus4m", 4, 1.0, Timbre::Default, 90);
    assert!(
        matches!(invalid_sus, Err(Error::InvalidChord(msg)) if msg.contains("sus chord cannot be minor"))
    );
}
