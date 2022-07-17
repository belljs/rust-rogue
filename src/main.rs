use crossterm::{
    cursor::{self, MoveDown, MoveTo},
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

#[derive(Copy, Clone)]
enum Tile {
    Empty,
    Floor,
    Wall,
}

impl Default for Tile {
    fn default() -> Self { Tile::Wall }
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

struct Level {
    map: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Level {
    fn new(w: usize, h: usize) -> Self {
        let m = vec![Tile::Wall; w*h];
        Level { map: m, width: w, height: h}
    }
}

fn main() -> Result<()> {
    let mut quit = false;
    let mut stdout = stdout();
    let _clean_up = CleanUp{};
    
    enable_raw_mode()?;
    stdout.queue(cursor::Hide)?;
    
    // let mut level = Level::new();
    let mut level = Level::new(70, 20);

    while !quit {
        // Clear Terminal and reset cursor to beginning
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0,0))?;
        
        // Draw Level
        for y in 0..level.width+1 {
            for x in 0..level.height+1 {
                stdout.queue(MoveTo(x as u16,y as u16))?;
                stdout.queue(PrintStyledContent(level.map[x + level.width + y].draw()))?;
            }
        }
        
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
