use std::time::Duration;

use noteplay::prelude::*;
mod common;
use common::has_output_device;

#[test]
fn noteplayer_construction_will_fail_if_theres_no_output_device() {
    assert_eq!(NotePlayer::new().is_ok(), has_output_device());
}

#[test]
fn noteplayer_set_soundfont_fails_if_the_target_sf2_file_doesnt_exist() {
    if !has_output_device() {
        return;
    }
    let mut np = NotePlayer::new().unwrap();
    np.set_soundfont("NONEXIST_SF2_FILE.sf2").unwrap_err();
}

#[test]
fn noteplayer_set_soundfont_fails_if_the_bytes_is_invalid() {
    if !has_output_device() {
        return;
    }
    let mut np = NotePlayer::new().unwrap();
    np.set_soundfont_from_bytes(&[0]).unwrap_err();
}

#[test]
#[should_panic]
fn noteplayer_panics_if_playing_invalid_notes() {
    assert!(has_output_device());
    let np = NotePlayer::new().unwrap();
    np.play_note("", Duration::from_secs_f32(1.0), 100);
}

#[test]
#[should_panic]
fn noteplayer_panics_playing_non_default_timbre_without_setting_soundfonts() {
    assert!(has_output_device());
    let mut np = NotePlayer::new().unwrap();
    np.set_timbre(Timbre::Cello);
    np.play_freq(
        Frequency::new_unchecked(440.0),
        Duration::from_secs_f32(1.0),
        100,
    );
}
