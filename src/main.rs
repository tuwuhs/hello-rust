
use crossterm::event::{Event, read, KeyCode};

fn main() {
    let _a = 25;
    let b = [1, 2, 3, 4];

    loop {
        // match read().unwrap() {
        //     Event::Key(event) => println!("{:?}", event.code),
        //     _ => (),
        // }

        if let Event::Key(event) = read().unwrap() {
            match event.code {
                KeyCode::Up => println!("YEAH"),
                KeyCode::Esc => break,
                _ => (),
            }
        }
    }
    
    println!("Hello, world! {:?}", b);
}
