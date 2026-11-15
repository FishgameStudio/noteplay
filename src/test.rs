// This binary only for tests, not for install.

use noteplay::core::NotePlayer;
use noteplay::midi::*;
use noteplay::songs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    MidiPlayer::play(&JINGLE_BELLS.to_runtime(), &NotePlayer::new()?);
    Ok(())
}
