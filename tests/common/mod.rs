#![allow(dead_code)]
use noteplay::prelude::*;

#[inline]
pub fn assert_midi_eq(a: &Midi, b: &Midi) {
    assert_eq!(a.tempo.get_tempo(), b.tempo.get_tempo());
    assert_eq!(a.meta, b.meta);
    assert_eq!(a.events, b.events);
}
#[inline]
pub fn has_output_device() -> bool {
    use rodio::cpal::default_host;
    use rodio::cpal::traits::HostTrait;
    default_host().default_output_device().is_some()
}
