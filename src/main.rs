
use std::{collections::VecDeque, io::{stdout, Write}};
use crossterm::{execute, queue, style, 
    terminal::{self, ClearType, ScrollUp, ScrollDown, SetSize, size}, 
    event::{self, Event, KeyCode},
    cursor
};

const COLS: usize = 32;
const ROWS: usize = 32;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

struct Snake {
    points: VecDeque<Point>,
}

impl Snake {
    fn update(&mut self, key_code: KeyCode) {
        let xy = self.points.back().unwrap();
        let new_xy = match key_code {
            // Y is top to bottom
            KeyCode::Down => Point{ x: xy.x, y: xy.y.min(ROWS as u32 - 2) + 1 },
            KeyCode::Up => Point{ x: xy.x, y: xy.y.max(1) - 1 },

            // X is left to right
            KeyCode::Right => Point{ x: xy.x.min(COLS as u32 - 2) + 1, y: xy.y },
            KeyCode::Left => Point{ x: xy.x.max(1) - 1, y: xy.y },

            _ => Point{ x: xy.x, y: xy.y },
        };
        self.points.push_back(new_xy);
        self.points.pop_front();
    }
}

fn display(snake: &Snake)
{
    let mut grid = [["."; COLS]; ROWS];
    
    // Whatever floats my boat...
    for xy in snake.points.iter() {
        grid[xy.y as usize][xy.x as usize] = "O";
    }

    let a = grid.iter().map(|y| y.join(" ")).collect::<Vec<_>>().join("\n");
    // println!("{}", a);

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
    let mut my_snake: Snake = Snake { points: VecDeque::new() };
    my_snake.points.push_back(Point { x: 0, y: 0 });
    my_snake.points.push_back(Point { x: 1, y: 0 });
    my_snake.points.push_back(Point { x: 2, y: 0 });

    clear();
    display(&my_snake);

    loop {
        if let Event::Key(event) = event::read().unwrap() {
            match event.code {
                KeyCode::Esc => break,
                key_code => my_snake.update(key_code),
            };
            // println!("{:?}", my_snake.points);
            display(&my_snake);
        }
    }
    
    println!("Hello, world!");
}
