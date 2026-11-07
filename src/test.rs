// This binary only for tests, not for install.

use noteplay::core::NotePlayer;
use noteplay::midi::*;
use noteplay::songs::BIRTHDAY_SONG;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    MidiPlayer::play(&BIRTHDAY_SONG.to_runtime(), &NotePlayer::new()?);

    Ok(())
}
