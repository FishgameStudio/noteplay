use noteplay::prelude::*;

#[test]
fn midi_meta_defaults_are_empty() {
    let meta = MidiMeta::new();
    assert!(meta.song_name.is_none());
    assert!(meta.copyright.is_none());
    assert!(meta.author.is_none());
}

#[test]
fn default_meta_contains_crate_metadata() {
    assert!(DEFAULT_META.song_name.is_none());
    assert_eq!(
        DEFAULT_META.copyright.unwrap().trim_end_null(),
        "Copyright (c) 2026 FishgameStudio"
    );
    assert_eq!(DEFAULT_META.author.unwrap().trim_end_null(), "Nicola Grey");
}

#[test]
fn midi_builder_helpers_set_the_expected_fields() {
    let tempo_only = Midi::with_tempo(Tempo::new_unchecked(90.0));
    assert_eq!(tempo_only.tempo.get_tempo(), 90.0);
    assert!(tempo_only.events.is_empty());
    assert!(tempo_only.meta.song_name.is_none());

    let meta = MidiMeta {
        song_name: Some(FixedString::<48>::from_str_const("Title")),
        copyright: Some(FixedString::<96>::from_str_const("Copyright 2026")),
        author: Some(FixedString::<32>::from_str_const("Alice")),
    };
    let with_meta = Midi::with_meta(meta);
    assert_eq!(with_meta.meta.song_name.unwrap().trim_end_null(), "Title");
    assert_eq!(with_meta.meta.author.unwrap().trim_end_null(), "Alice");
    assert_eq!(
        with_meta.meta.copyright.unwrap().trim_end_null(),
        "Copyright 2026"
    );
}

#[test]
fn fixed_string_truncates_and_zero_pads_data() {
    let text = FixedString::<8>::from_str_const("ABCDEFGH");
    assert_eq!(text.trim_end_null(), "ABCDEFGH");

    let truncated = FixedString::<5>::from_str_const("Hello World");
    assert_eq!(truncated.trim_end_null(), "Hello");

    let with_padding = FixedString::<6>::from_str_const("Hi");
    assert_eq!(with_padding.buf[0], 'H');
    assert_eq!(with_padding.buf[1], 'i');
    assert_eq!(with_padding.buf[2], '\0');
    assert_eq!(with_padding.buf[3], '\0');
    assert_eq!(with_padding.buf[4], '\0');
    assert_eq!(with_padding.buf[5], '\0');
}
