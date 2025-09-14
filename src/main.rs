use std::fs::File;
use std::io::BufReader;

use anyhow::Ok;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use rodio::{Decoder, OutputStreamBuilder};

const OWO: &str = r#"
  $$\                                       $$\                            $$\               
  $$ |                                      $$ |                           $$ |              
$$$$$$\    $$$$$$\   $$$$$$\  $$$$$$\$$$$\  $$$$$$$\   $$$$$$\   $$$$$$\ $$$$$$\    $$$$$$$\ 
\_$$  _|  $$  __$$\ $$  __$$\ $$  _$$  _$$\ $$  __$$\ $$  __$$\  \____$$\\_$$  _|  $$  _____|
  $$ |    $$$$$$$$ |$$ |  \__|$$ / $$ / $$ |$$ |  $$ |$$$$$$$$ | $$$$$$$ | $$ |    \$$$$$$\  
  $$ |$$\ $$   ____|$$ |      $$ | $$ | $$ |$$ |  $$ |$$   ____|$$  __$$ | $$ |$$\  \____$$\ 
  \$$$$  |\$$$$$$$\ $$ |      $$ | $$ | $$ |$$$$$$$  |\$$$$$$$\ \$$$$$$$ | \$$$$  |$$$$$$$  |
   \____/  \_______|\__|      \__| \__| \__|\_______/  \_______| \_______|  \____/ \_______/ 
                                                                                                                      
"#;

fn main() -> anyhow::Result<()> {
    println!("{}", OWO);
    println!("Controls: A=Kick | S=Snare | D=Hi-Hat | Q=Quit");
    enable_raw_mode()?;

    let mut stream = OutputStreamBuilder::open_default_stream()?;
    stream.log_on_drop(false);
    let mixer = stream.mixer();

    loop {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char('a') => {
                        play(mixer, "assets/kick.mp3")?;
                    }
                    KeyCode::Char('s') => {
                        play(mixer, "assets/snare.mp3")?;
                    }
                    KeyCode::Char('d') => {
                        play(mixer, "assets/hat.mp3")?;
                    }
                    KeyCode::Char('q') => {
                        drop(stream);
                        disable_raw_mode()?;
                        return Ok(());
                    }
                    _ => (),
                }
            }
        }
    }
}

fn play(mixer: &rodio::mixer::Mixer, path: &str) -> anyhow::Result<()> {
    let file = BufReader::new(File::open(path)?);
    let source = Decoder::new(file)?; // convert samples to correct format
    mixer.add(source);
    Ok(())
}
