# Changelog

## [0.2.0] - 2026-11-21

### Added
- New `sf2` feature and `soundfont` module, backed by `rustysynth`; GM Level 1 `Timbre` enum for instrument selection
- Built-in `TimGM6mb.sf2` under `builtin_sf2` (behind `sf2` feature), exposed as `TIMGM6MB` constant
- `NotePlayer` APIs: `set_soundfont`, `set_soundfont_from_bytes`, `set_timbre`, `get_timbre`
- Velocity parameter added to all note playback methods (`play_freq`, `play_note`, their async/repeat variants)
- `is_valid_note` validation helper in `core`
- `prelude` module for convenient bulk imports
- `FixedString` generic fixed-length string type for QMid metadata, with const conversion
- QMid metadata support: song name, author, copyright; version bumped to `0x0003`
- `midi` utilities: `parse_chord` (supports major/minor, sus, 7/9/13, altered extensions), `play_qmid_file`, `get_metadata`
- New `event` / `event_const` / `midi` / `midi_const` macro overloads accepting timbre, velocity and metadata
- `MidiPlayer::loop_play` for endless synchronous playback
- New example `play_music` replacing the old internal `test` binary
- New built-in songs: `JINGLE_BELLS`, `SILENT_NIGHT`; existing songs now carry metadata and multi-timbre arrangements
- New test suite: chord parsing, frequency conversion, macros, metadata, MIDI roundtrip, noteplayer playback
- `CONTRIBUTING.md` rule: new folders must have a descriptive `README.md`
- CI workflow runs on both `main` and `dev` branches
- Crate metadata: description, `repository`, `license = "MIT"`

### Changed
- Version bumped from `0.1.0` -> `0.2.0`
- `full` feature now enables both `songs` and `sf2`
- QMid binary format changed: header now contains metadata fields; each note stores `(freq, duration, timbre, velocity)`
- `serialize` now takes `&Midi` by reference instead of taking ownership
- `MidiPlayer::play` now takes `&mut NotePlayer`
- All note-playing signatures gained velocity; sine wave is only used for `Timbre::Default`
- `core`, `midi`, `soundfont`, `builtin_sf2`, `prelude` modules now use `include_str!` module-level docs
- README example updated to show soundfont setup; docs.rs link fixed
- Various typo fixes
- Error enum expanded with chord, synthesizer, soundfont, timbre and fixed-string parse variants

### Removed
- Old internal `src/test.rs` binary

### Fixed
- Panic conditions documented for playback APIs when using non-default timbre without a loaded soundfont
- Articulation and remaining-playback timing logic preserved while adapting to new note structure
- `NotePlayer` now correctly routes sine-wave or SoundFont synthesis depending on timbre
