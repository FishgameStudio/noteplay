use noteplay::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = serialize(&SILENT_NIGHT.to_runtime());
    fn print_hex(buf: &[u8]) {
        for chunk in buf.chunks(16) {
            for byte in chunk {
                print!("{byte:02x} ");
            }
            println!();
        }
    }
    print_hex(&bytes);
    std::fs::write(
        format!(
            "bin_output/{}.qmid",
            SILENT_NIGHT
                .meta
                .song_name
                .unwrap_or_default()
                .trim_end_null()
        ),
        bytes,
    )?;
    Ok(())
}
