use noteplay::core::NotePlayer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = NotePlayer::new()?;
    p.play_note("C4", 1.0);
    Ok(())
}
