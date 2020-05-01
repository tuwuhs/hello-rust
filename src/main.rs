
use std::io::{stdout, Write};
use crossterm::{execute, queue, style, Result,
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

fn update(xy: &Point, key_code: KeyCode) -> Point {
    match key_code {
        // Y is top to bottom
        KeyCode::Down => Point{ x: xy.x, y: xy.y.min(ROWS as u32 - 2) + 1 },
        KeyCode::Up => Point{ x: xy.x, y: xy.y.max(1) - 1 },

        // X is left to right
        KeyCode::Right => Point{ x: xy.x.min(COLS as u32 - 2) + 1, y: xy.y },
        KeyCode::Left => Point{ x: xy.x.max(1) - 1, y: xy.y },

        _ => Point{ x: xy.x, y: xy.y },
    }
}

fn display(xy: &Point)
{
    let mut grid = [["."; COLS]; ROWS];
    
    // for x in grid.iter_mut().flat_map(|r| r.iter_mut()) {
    //     println!{"{:?}", x};
    // }

    // for y in grid.iter() {
    //     y.join(" ");
    //     println!("{:?}", y.iter().collect::<String>());
    // };
    
    // Whatever floats my boat...
    grid[xy.y as usize][xy.x as usize] = "O";

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
    // let mut map: [[char; 4]; 4] = [['.'; 4]; 4];
    // // map[0][1] = 'X';
    // println!("{:?}", map);
    // let (cols, rows) = size().unwrap();

    clear();

    let mut xy: Point = Point { x: 0, y: 0 };
    display(&xy);
    loop {
        if let Event::Key(event) = event::read().unwrap() {
            xy = match event.code {
                KeyCode::Esc => break,
                key_code => update(&xy, key_code),
            };
            // println!("{:?} {:?}", event, &xy);
            display(&xy);
        }
    }
    
    println!("Hello, world!");
}
