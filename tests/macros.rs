use noteplay::midi::{AUTHOR_LEN, COPYRIGHT_LEN, FixedString, MidiMeta, SONG_NAME_LEN};
use noteplay::soundfont::Timbre;
use noteplay::{event, event_const, midi, midi_const};

#[test]
fn event_macro_creates_expected_runtime_event() {
    let ev = event!(1.5, [("C4", 0.5), ("E4", 0.5)]).get_normal_unchecked();
    let ev_with_timbre = event!(0.5, [("G4", 0.5, Cello)]).get_normal_unchecked();
    let ev_with_velocity = event!(0.25, [("B4", 0.5, Cello, 64)]).get_normal_unchecked();

    assert_eq!(ev.delta_beat, 1.5);
    assert_eq!(ev.notes.len(), 2);
    assert_eq!(ev.notes[0].2, Timbre::BrightAcousticPiano);

    assert_eq!(ev_with_timbre.delta_beat, 0.5);
    assert_eq!(ev_with_timbre.notes.len(), 1);
    assert_eq!(ev_with_timbre.notes[0].2, Timbre::Cello);

    assert_eq!(ev_with_velocity.delta_beat, 0.25);
    assert_eq!(ev_with_velocity.notes.len(), 1);
    assert_eq!(ev_with_velocity.notes[0].2, Timbre::Cello);
    assert_eq!(ev_with_velocity.notes[0].3, 64);
}

#[test]
fn midi_macro_sets_tempo_and_default_metadata() {
    let song = midi!(
        "Demo Song",
        120.0,
        event!(0.0, [("C4", 1.0)]),
        event!(1.0, [("E4", 0.5, Cello)]),
    );

    assert_eq!(song.tempo.get_tempo(), 120.0);
    assert_eq!(song.events.len(), 2);
    assert_eq!(song.meta.song_name.unwrap().trim_end_null(), "Demo Song");
    assert_eq!(song.meta.author.unwrap().trim_end_null(), "Nicola Grey");
    assert_eq!(
        song.meta.copyright.unwrap().trim_end_null(),
        "Copyright (c) 2026 FishgameStudio"
    );
}

#[test]
fn const_macros_build_equivalent_runtime_data() {
    let raw_event = event_const!(1.0, [("C4", 0.5), ("G4", 0.5)]).get_normal_unchecked();
    let raw_timbred_event = event_const!(0.5, [("E4", 0.5, Cello)]).get_normal_unchecked();
    let runtime_event = raw_event.to_runtime();

    assert_eq!(runtime_event.delta_beat, 1.0);
    assert_eq!(runtime_event.notes.len(), 2);
    assert_eq!(runtime_event.notes[0].2, Timbre::BrightAcousticPiano);

    let runtime_timbred_event = raw_timbred_event.to_runtime();
    assert_eq!(runtime_timbred_event.notes[0].2, Timbre::Cello);

    let raw_song = midi_const!(
        "Const Song",
        90.0,
        raw_event.to_event(),
        raw_timbred_event.to_event()
    );
    assert_eq!(raw_song.tempo, 90.0);
    assert_eq!(
        raw_song.meta.song_name.unwrap().trim_end_null(),
        "Const Song"
    );
    assert_eq!(raw_song.meta.author.unwrap().trim_end_null(), "Nicola Grey");

    let converted = raw_song.to_runtime();
    assert_eq!(converted.tempo.get_tempo(), 90.0);
    assert_eq!(converted.events.len(), 2);
}

#[test]
fn midi_meta_and_fixed_string_round_trip() {
    let meta = MidiMeta {
        song_name: Some(FixedString::<SONG_NAME_LEN>::from_str_const("Title")),
        copyright: Some(FixedString::<COPYRIGHT_LEN>::from_str_const(
            "Copyright 2026",
        )),
        author: Some(FixedString::<AUTHOR_LEN>::from_str_const("Author")),
    };

    assert_eq!(meta.song_name.unwrap().trim_end_null(), "Title");
    assert_eq!(meta.copyright.unwrap().trim_end_null(), "Copyright 2026");
    assert_eq!(meta.author.unwrap().trim_end_null(), "Author");
}
