# termbeats

A terminal-based drum machine built with Rust.

## Installation

### Using Homebrew (macOS)

```bash
brew tap dawksh/termbeats
brew install termbeats
```

### From Source

Make sure you have Rust installed, then:

```bash
git clone https://github.com/dawksh/termbeats.git
cd termbeats
cargo build --release
```

## Usage

Run `termbeats` in your terminal:

```bash
termbeats
```

### Controls

- **A** - Kick drum
- **S** - Snare drum
- **D** - Hi-hat
- **Q** - Quit

## Dependencies

- Rust 1.70+
- Audio output device (built-in speakers/headphones work fine)

## License

MIT
