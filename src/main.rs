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
        let m = vec![Tile::Empty; w*h];
        Level { map: m, width: w, height: h}
    }
    
    fn add_room_to_map(&mut self, room: Room) {
        for y in room.y..room.y+room.h {
            for x in room.x..room.x+room.w {
                if x == room.x || y == room.y {
                    self.map[x+self.width*y] = Tile::Wall;
                } else if x == room.x+room.w-1 || y == room.y+room.h-1 {
                    self.map[x+self.width*y] = Tile::Wall;
                } else {
                    self.map[x+self.width*y] = Tile::Floor;
                }
            }
        }
    }
}

struct Room {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Room {
    fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
        Room {x,y,w,h}
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Character {
    x: usize,
    y: usize,
    symbol: char,
    color: Color,
}

impl Character {
    fn new(x: usize, y:usize, symbol:char, color:Color) -> Self {
        Character{x, y, symbol, color}
    }
    
    fn draw(&self) -> StyledContent<String> {
        format!("{}", self.symbol).with(self.color)
    }
    
    fn go(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        };
    }
}

fn main() -> Result<()> {
    let mut quit = false;
    let mut stdout = stdout();
    let _clean_up = CleanUp{};
    
    enable_raw_mode()?;
    stdout.queue(cursor::Hide)?;

    // Manually create a level and rooms until an algorithm is implemented.
    let mut level = Level::new(70, 20);
    let room = Room::new(5, 2, 15, 10);    
    let room2 = Room::new(25, 8, 30, 10);
    let room3 = Room::new(60, 12, 10, 8);
    level.add_room_to_map(room);
    level.add_room_to_map(room2);
    level.add_room_to_map(room3);
    
    let mut player = Character::new(16, 6, '@', Color::Green);

    while !quit {
        // Clear Terminal and reset cursor to the top left
        stdout.queue(Clear(ClearType::All))?;
        stdout.queue(MoveTo(0,0))?;
        
        // Draw Level
        for y in 0..level.height {
            for x in 0..level.width {
                stdout.queue(MoveTo(x as u16, y as u16))?;
                stdout.queue(PrintStyledContent(level.map[x+level.width*y].draw()))?;
            }
        }
        
        // Draw Player
        stdout.queue(MoveTo(player.x as u16, player.y as u16))?;
        stdout.queue(PrintStyledContent(player.draw()))?;
        
        // Flush stdout
        stdout.flush()?;
        
        // Get Input
        let event = read()?;
        if event == Event::Key(KeyCode::Esc.into()) {
            quit = true;
        } else if event == Event::Key(KeyCode::Up.into()) {
            player.go(Direction::Up);
        } else if event == Event::Key(KeyCode::Down.into()) {
            player.go(Direction::Down);
        } else if event == Event::Key(KeyCode::Left.into()) {
            player.go(Direction::Left);
        } else if event == Event::Key(KeyCode::Right.into()) {
            player.go(Direction::Right);
        }
    }
    disable_raw_mode()
}
