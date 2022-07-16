use crossterm::{
    cursor::{self, MoveTo},
    event::{Event, KeyCode, read},
    style::{Color, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType, enable_raw_mode, disable_raw_mode},
    Result,
    QueueableCommand,
};

use std::io::{stdout, Write};

struct CleanUp {}

impl Drop for CleanUp {
    fn drop(&mut self) {
        let mut stdout = stdout();
        stdout.queue(cursor::Show).expect("Unable to show cursor");
        stdout.queue(MoveTo(0,0)).expect("Unable to move cursor");
        stdout.flush().expect("Unable to flush stdout");
    }
}

fn main() -> Result<()> {
    let mut quit = false;
    let mut stdout = stdout();
    let _clean_up = CleanUp{};
    
    enable_raw_mode()?;
    stdout.queue(cursor::Hide)?;
    
    while !quit {
        // Clear Terminal and reset cursor to beginning
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0,0))?;
        
        // Print a green @ symbol to represent our player
        let content = format!("@").with(Color::Green);
        stdout.queue(PrintStyledContent(content))?;
        
        // Flush stdout
        stdout.flush()?;
        
        // Get Input
        let event = read()?;
        if event == Event::Key(KeyCode::Esc.into()) {
            quit = true;
        }
    }
    disable_raw_mode()
}
