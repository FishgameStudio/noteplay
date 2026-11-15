// This binary only for tests, not for install.

use std::time::Duration;

use noteplay::builtin_sf2::TIMGM6MB;
use noteplay::core::NotePlayer;
use noteplay::soundfont::Timbre;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut np = NotePlayer::new()?;
    np.set_soundfont_from_bytes(TIMGM6MB)?;
    np.set_timbre(Timbre::Violin);
    np.play_note("C5", Duration::from_secs_f32(1.0), 90);
    np.play_note("E5", Duration::from_secs_f32(1.0), 90);
    np.play_note("G5", Duration::from_secs_f32(1.0), 90);
    np.play_note("C6", Duration::from_secs_f32(1.0), 90);
    Ok(())
}
