use noteplay::core::NotePlayer;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let p = NotePlayer::new()?;
    const SPEED: f32 = 0.5;
    p.repeat_note("C4", SPEED, 2);
    p.repeat_note("G4", SPEED, 2);
    p.repeat_note("A4", SPEED, 2);
    p.play_note("G4", SPEED * 2.0);
    p.repeat_note("F4", SPEED, 2);
    p.repeat_note("E4", SPEED, 2);
    p.repeat_note("D4", SPEED, 2);
    p.play_note("C4", SPEED * 2.0);
    p.repeat_note("G4", SPEED, 2);
    p.repeat_note("F4", SPEED, 2);
    p.repeat_note("E4", SPEED, 2);
    p.play_note("D4", SPEED * 2.0);
    p.repeat_note("G4", SPEED, 2);
    p.repeat_note("F4", SPEED, 2);
    p.repeat_note("E4", SPEED, 2);
    p.play_note("D4", SPEED * 2.0);
    p.repeat_note("C4", SPEED, 2);
    p.repeat_note("G4", SPEED, 2);
    p.repeat_note("A4", SPEED, 2);
    p.play_note("G4", SPEED * 2.0);
    p.repeat_note("F4", SPEED, 2);
    p.repeat_note("E4", SPEED, 2);
    p.repeat_note("D4", SPEED, 2);
    p.play_note("C4", SPEED * 2.0);
    Ok(())
}
