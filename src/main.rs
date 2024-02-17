use std::{io::Write, time::Duration};
use crossterm::{event::{poll, read, Event, KeyCode}, terminal::ClearType, QueueableCommand};
use rand::{thread_rng, Rng};
const BOARD_WIDTH: usize = 30;
const BOARD_HEIGHT: usize = 20;

#[derive(PartialEq)]
enum Tile {
    Floor,
    SnakeHead,
    SnakeBody,
    Food
} 

struct Food {
    x: usize,
    y: usize
}

impl Food {
    fn new() -> Self {
        let mut f = Food{x:0, y:0};
        f.rand();
        f
    }
    fn rand(&mut self) {
        let mut rng = thread_rng();
        let y: usize = rng.gen_range(0..BOARD_HEIGHT);
        let x: usize = rng.gen_range(0..BOARD_WIDTH);
        self.x = x;
        self.y = y;
    }
    
}


#[derive(PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down
}
struct Coords {
    x: usize,
    y: usize
}

struct Player {
    x: usize,
    y: usize,
    direction: Direction,
    segements: Vec<Coords>,
    score: usize
}

impl Player {
    fn new() -> Self {
        let p = Player{
            x: BOARD_WIDTH / 2,
            y: BOARD_HEIGHT / 2,
            direction: Direction::Right,
            score: 0,
            segements: vec![
                Coords{x: (BOARD_WIDTH / 2) - 1, y: BOARD_HEIGHT / 2},
                Coords{x: (BOARD_WIDTH / 2) - 2, y: BOARD_HEIGHT / 2}
            ]
        };
        return p;
    }
    fn detect_collision(&mut self) -> bool {
        for segment in self.segements.iter() {
            if self.x == segment.x && self.y == segment.y {
                return true
            }
        }
        return false
    }
    fn update_pos(&mut self) {

        if self.segements.len() > 0 {
            self.segements.insert(0, Coords{x: self.x, y: self.y});
            if self.score + 3 == self.segements.len() {
                self.segements.pop();
            }

        }

        if self.direction == Direction::Right {
            self.x = ( self.x + 1 ) % BOARD_WIDTH
        }
        if self.direction == Direction::Left {
            if self.x == 0 {
                self.x = BOARD_WIDTH - 1 
            } else {
                self.x = ( self.x - 1 ) % BOARD_WIDTH
            }
        }
        if self.direction == Direction::Up {
            if self.y == 0 {
                self.y = BOARD_HEIGHT -1 
            } else {
                self.y = ( self.y -  1 ) % BOARD_HEIGHT
            }
        }
        if self.direction == Direction::Down {
            self.y = ( self.y + 1 ) % BOARD_HEIGHT
        }
    } 
}

struct Board {
    grid: Vec<Vec<Tile>>
}

impl Board {
    fn new() -> Board {
        let mut grid = Vec::new();
        for _ in 0..BOARD_WIDTH {
            let mut v2 = Vec::new();
            for _ in 0..BOARD_HEIGHT {
                v2.push(Tile::Floor)
            }
            grid.push(v2);
        }
        return Board{grid}
    }   
    fn clear(&mut self) {
        for i in &mut self.grid {
            for j in 0..i.len() {
                i[j] = Tile::Floor
            }
        }
    }
    fn place_head(&mut self, x: usize, y: usize) {
        self.grid[x][y] = Tile::SnakeHead
    }
    fn place_food(&mut self, x: usize, y: usize) {
        self.grid[x][y] = Tile::Food
    }
    fn place_segments(&mut self, segments: &Vec<Coords>) {
        for segment in segments.iter() {
            self.grid[segment.x][segment.y] = Tile::SnakeBody
        }

    }

    fn display(&self, score: &usize) {
        for _ in 0usize..(BOARD_WIDTH + 1) {
            print!("--")
        }
        print!("\n\r");
        for j in 0usize..BOARD_HEIGHT {
            print!("|");
            for i in 0usize..BOARD_WIDTH {
                let tile = &self.grid[i][j];
                match tile {
                    Tile::Floor => print!("  "),
                    Tile::Food => print!("* "),
                    Tile::SnakeHead => print!("S "),
                    Tile::SnakeBody => print!("s ")
                }
            }
            print!("|");
            print!("\n\r");
            

        }
        for _ in 0usize..(BOARD_WIDTH + 1) {
            print!("--")
        }
        print!("\n\r");
        print!("Score: {}", score);
        print!("\n\r");

    }
}

fn clear_screen() {
    let mut out = std::io::stdout();
    out.queue(crossterm::terminal::Clear(ClearType::All)).unwrap();
    out.queue(crossterm::cursor::Hide).unwrap();
    out.queue(crossterm::cursor::MoveTo(0,0)).unwrap();
    out.flush().unwrap();
}


fn main() {
    crossterm::terminal::enable_raw_mode().unwrap();
    clear_screen();
    let mut board = Board::new();
    let mut player = Player::new();
    let mut food = Food::new();
    let mut tick_counter: u64 = 0;
    let mut speed = 15f64;

    loop {
        if poll(Duration::from_millis(speed as u64)).unwrap() {
            match read().unwrap() {
                Event::Key(event) => {
                    match event.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Up | KeyCode::Char('w') => {
                            if player.direction != Direction::Down {
                                player.direction = Direction::Up
                            }
                        },
                        KeyCode::Down | KeyCode::Char('s') => {
                            if player.direction != Direction::Up {
                                player.direction = Direction::Down
                            }
                        },
                        KeyCode::Left | KeyCode::Char('a') => {
                            if player.direction != Direction::Right {
                                player.direction = Direction::Left
                            }
                        },
                        KeyCode::Right | KeyCode::Char('d') =>  {
                            if player.direction != Direction::Left {
                                player.direction = Direction::Right
                            }
                        },
                        _ => ()
                    }
                },
                _ => ()
            }
        } 

        if tick_counter % 10 == 0 {
            board.clear();
            player.update_pos();
            if player.detect_collision() {
                print!("Game Over");
                print!("\n\r");
                break
            }
            if player.x == food.x && player.y == food.y {
                food.rand();
                player.score += 1;
                speed *= 0.97;
            }

            board.place_head(player.x, player.y);
            board.place_food(food.x, food.y);
            board.place_segments(&player.segements);
            clear_screen();
            board.display(&player.score);
        }
        tick_counter += 1;
    }
}
