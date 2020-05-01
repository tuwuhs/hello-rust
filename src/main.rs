
use std::{collections::VecDeque, io::{stdout, Write}};
use crossterm::{execute, style, cursor,
    terminal::{self, ClearType}, 
    event::{self, Event, KeyCode},
};
use rand::Rng;
use std::time::Duration;

const COLS: i32 = 32;
const ROWS: i32 = 32;

#[derive(Debug, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Down, Up, Right, Left
}

impl Direction {
    fn opposite(&self) -> Direction {
        match *self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right
        }
    }
}

struct Snake {
    points: VecDeque<Point>,
    last_dir: Direction,
}

impl Snake {
    fn new(init_x: i32, init_y: i32) -> Snake {
        let mut points = VecDeque::new();
        points.push_back(Point { x: init_x, y: init_y });
        points.push_back(Point { x: init_x + 1, y: init_y });
        points.push_back(Point { x: init_x + 2, y: init_y });    
        
        Snake { points, last_dir: Direction::Right }
    }

    fn update(&mut self, dir: Direction) -> bool {
        let xy = self.points.back().unwrap();
        
        if self.last_dir.opposite() == dir {
            return false;
        }

        let new_xy = match dir {
            // Y is top to bottom
            Direction::Down => Point{ 
                x: xy.x, 
                y: (xy.y + 1).rem_euclid(ROWS) 
            },
            Direction::Up => Point{ 
                x: xy.x, 
                y: (xy.y - 1).rem_euclid(ROWS) 
            },

            // X is left to right
            Direction::Right => Point{ 
                x: (xy.x + 1).rem_euclid(COLS), 
                y: xy.y 
            },
            Direction::Left => Point{ 
                x: (xy.x - 1).rem_euclid(COLS), 
                y: xy.y 
            },
        };

        self.last_dir = dir;
        self.points.push_back(new_xy);
        
        true
    }

    fn no_apple(&mut self) {
        self.points.pop_front();
    }

    fn head(&self) -> &Point {
        self.points.back().unwrap()
    }

    fn collide_with_me(&self, q: &Point) -> bool {
        self.points
            .iter()
            .any(|p| {
                p.x == q.x 
                && p.y == q.y
            })
    }

    fn hit_myself(&self) -> bool {
        let q = self.head();
        self.points
            .iter()
            .enumerate()
            .any(|(i, p)| {
                i != self.points.len() - 1  // Skip head
                && p.x == q.x 
                && p.y == q.y
            })
    }
}

fn display(snake: &Snake, apple: &Point)
{
    let mut grid = [["."; COLS as usize]; ROWS as usize];
    
    // Whatever floats my boat...
    grid[apple.y as usize][apple.x as usize] = "X";
    
    for xy in snake.points.iter() {
        grid[xy.y as usize][xy.x as usize] = "O";
    }

    let a = grid
        .iter()
        .map(|y| y.join(" "))
        .collect::<Vec<_>>()
        .join("\n");

    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        style::Print(&a)
    ).unwrap();
}

fn clear() {
    execute!(
        stdout(),
        terminal::Clear(ClearType::All),
        cursor::MoveTo(0, 0),
    ).unwrap();
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut my_snake = Snake::new(COLS / 2, ROWS / 2);
    let mut apple = Point{ 
        x: rng.gen::<i32>().rem_euclid(COLS),
        y: rng.gen::<i32>().rem_euclid(ROWS) 
    };
    
    while my_snake.collide_with_me(&apple) {
        apple.x = rng.gen::<i32>().rem_euclid(COLS);
        apple.y = rng.gen::<i32>().rem_euclid(ROWS);
    }

    clear();
    display(&my_snake, &apple);
    
    loop {
        if event::poll(Duration::from_millis(50)).unwrap() {
            if let Event::Key(event) = event::read().unwrap() {
                let dir = match event.code {
                    KeyCode::Esc => break,
                    KeyCode::Down => Direction::Down,
                    KeyCode::Up => Direction::Up,
                    KeyCode::Right => Direction::Right,
                    KeyCode::Left => Direction::Left,
                    _ => continue
                };
                if !my_snake.update(dir) {
                    continue;
                }
            }
        } else {
            my_snake.update(my_snake.last_dir);
        }

        if *my_snake.head() == apple {
            loop {
                apple.x = rng.gen::<i32>().rem_euclid(COLS);
                apple.y = rng.gen::<i32>().rem_euclid(ROWS);
                if !my_snake.collide_with_me(&apple) {
                    break;
                }
            }
        } else {
            my_snake.no_apple();
        }

        if my_snake.hit_myself() {
            break;
        }

        display(&my_snake, &apple);
    }
    
    println!("Hello, world!");
}
