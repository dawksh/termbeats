use anyhow::Ok;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink, stream};

fn main() -> anyhow::Result<()> {
    println!("🎶 termbeat 🎶");
    println!("Controls: A=Kick | S=Snare | D=Hi-Hat | Q=Quit");
    enable_raw_mode()?;
    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('a') => println!("Kick"),
                    KeyCode::Char('s') => println!("Snare"),
                    KeyCode::Char('d') => println!("Hi-Hat"),
                    KeyCode::Char('q') => break,
                    _ => (),
                }
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}
