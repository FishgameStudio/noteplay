# Noteplay

![Stars](https://img.shields.io/github/stars/FishgameStudio/noteplay)
![Forks](https://img.shields.io/github/forks/FishgameStudio/noteplay)
[![Crates.io](https://img.shields.io/crates/v/noteplay)](https://crates.io/crates/noteplay)
![Downloads](https://img.shields.io/crates/d/noteplay)
![License](https://img.shields.io/crates/l/noteplay)
[![BuildStatus](https://github.com/FishgameStudio/noteplay/actions/workflows/ci.yml/badge.svg)](https://github.com/FishgameStudio/noteplay/actions/workflows/ci.yml)

**Noteplay** is a Rust crate for turning musical notes and simple arrangements into sound. It can play individual notes through the system's default audio output, schedule notes together into chords and sequences, and play short songs at a chosen tempo. It's intended as a straightforward starting point for adding musical playback to Rust projects - not as a full digital audio workstation.

### ❓ What it can do?

- Play notes written in scientific pitch notation, such as `C4`, `G#5`, and `Db5`, or play a frequency directly.
- Play notes asynchronously or block until a note has finished.
- Describe music as events containing one or more notes, with beat-based timing and tempo.
- Convert a `MidiRaw` definition into a runtime `Midi` sequence and play it.
- Serialize and deserialize sequences using Noteplay's compact **QMid** format.
- Optionally include ready-made songs, such as *Twinkle Twinkle Little Star* and *Ode to Joy*.

## 🚀 Getting Started

### 📦 Installation

Install `noteplay` crate by cargo:

```sh
cargo add noteplay
```

Or add this to your `Cargo.toml`:

```toml
noteplay = { version = "0.1.0", features = ["full"] }
```

### 💡 Usage

```rust,no_run
use noteplay::songs::*;
use noteplay::midi::MidiPlayer;
use noteplay::core::NotePlayer;

fn main() {
    // Play "Birthday Song"
    MidiPlayer::play(&BIRTHDAY_SONG.to_runtime(), &NotePlayer::new().unwrap());
}
```
The module `songs` included in the feature `songs`.
Click [here](https://doc.rs/noteplay/latest/noteplay) to view the full documentation.

## 📃 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
