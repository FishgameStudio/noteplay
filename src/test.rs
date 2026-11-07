// This binary only for tests, not for install.

use noteplay::core::NotePlayer;
use noteplay::midi::*;
use noteplay::songs::TEST;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    MidiPlayer::play(&TEST.to_runtime(), &NotePlayer::new()?);

    Ok(())
}
