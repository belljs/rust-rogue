use crossterm::{
    cursor::{self, MoveTo},
    event::{Event, KeyCode, read},
    style::{Color, PrintStyledContent, StyledContent, Stylize},
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

enum Tile {
    Empty,
    Floor,
    Wall,
}

impl Tile {
    fn draw(&self) -> StyledContent<String> {
        match self {
            Tile::Empty => format!(" ").with(Color::Blue),
            Tile::Floor => format!(".").with(Color::Blue),
            Tile::Wall => format!("#").with(Color::Blue),
        }
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
        
        let tile = Tile::Wall;
        stdout.queue(PrintStyledContent(tile.draw()))?;
        
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
