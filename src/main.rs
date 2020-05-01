
use crossterm::event::{Event, read, KeyCode};

const XMIN: u32 = 0;
const YMIN: u32 = 0;
const XMAX: u32 = 31;
const YMAX: u32 = 31;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

fn update(xy: Point, key_code: KeyCode) -> Point {
    match key_code {
        KeyCode::Up => Point{ x: xy.x, y: xy.y.min(YMAX - 1) + 1 },
        KeyCode::Down => Point{ x: xy.x, y: xy.y.max(YMIN + 1) - 1 },
        KeyCode::Right => Point{ x: xy.x.min(XMAX - 1) + 1, y: xy.y },
        KeyCode::Left => Point{ x: xy.x.max(XMIN + 1) - 1, y: xy.y },
        _ => xy,
    }
}

fn main() {
    let mut xy: Point = Point { x: 0, y: 0 };
    loop {
        if let Event::Key(event) = read().unwrap() {
            xy = match event.code {
                KeyCode::Esc => break,
                key_code => update(xy, key_code),
            };
            println!("{:?} {:?}", event, xy);
        }
    }
    
    println!("Hello, world!");
}
