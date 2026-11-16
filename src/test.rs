use noteplay::builtin_sf2::TIMGM6MB;
use noteplay::core::NotePlayer;
use noteplay::midi::*;
use noteplay::songs::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut np = NotePlayer::new()?;
    np.set_soundfont_from_bytes(TIMGM6MB)?;
    MidiPlayer::play(&TEST.to_runtime(), &mut np);
    Ok(())
}
