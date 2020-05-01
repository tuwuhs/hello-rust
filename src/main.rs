
// use crossterm::event::{Event, read, KeyCode};

// const XMIN: u32 = 0;
// const YMIN: u32 = 0;
// const XMAX: u32 = 31;
// const YMAX: u32 = 31;

// #[derive(Debug)]
// struct Point {
//     x: u32,
//     y: u32,
// }

// fn update(xy: Point, key_code: KeyCode) -> Point {
//     match key_code {
//         KeyCode::Up => Point{ x: xy.x, y: xy.y.min(YMAX - 1) + 1 },
//         KeyCode::Down => Point{ x: xy.x, y: xy.y.max(YMIN + 1) - 1 },
//         KeyCode::Right => Point{ x: xy.x.min(XMAX - 1) + 1, y: xy.y },
//         KeyCode::Left => Point{ x: xy.x.max(XMIN + 1) - 1, y: xy.y },
//         _ => xy,
//     }
// }

// See https://burgers.io/extending-iterator-trait-in-rust
// and https://burgers.io/wrapped-iterators-in-rust
struct Update<I: Iterator> {
    iter: I,
    idx: usize,
    val: I::Item,
    count: usize,
}

impl<I: Iterator> Iterator for Update<I>
{
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        let a = self.iter.next()?;
        let i = self.count;
        self.count += 1;
        if i == self.idx {
            Some(self.val)
        } else {
            Some(a)
        }
    }
}

impl<I: Iterator> Update<I>
{
    fn new(iter: I, idx: usize, val: I::Item) -> Update<I> {
        Update { iter, idx, val, count: 0 }
    }
}

// trait Updatable {
//     fn adjust(self, idx: usize, val: Self::Item) -> Update<Self> where Self: Sized;
// }

// impl<I> Updatable for I where I: Iterator, I: Sized {
//     fn adjust<F>(self, idx: usize, f: F) -> Adjust<Self, F> where Self: Sized {
//         Adjust { iter: self, idx, f, count: 0 }
//     }
// }

fn main() {
    // let mut map: [[char; 4]; 4] = [['.'; 4]; 4];
    // // map[0][1] = 'X';
    // println!("{:?}", map);

    let a = [123, 3426, 23, 124, 6345, 27, 33];
    // let b: Vec<_> = a.iter().enumerate().my_map(|(i, x)| if i == 2 { 0 } else { *x } ).collect();
    let b: Vec<_> = a.iter().map(|x| x).collect();
    println!("{:?} {:?}", a, b);

    for x in Update::new(a.iter(), 2, &4) {
        println!("{:?}", x);
    }

    // for x in a.iter().adjust(1, |x| 2*x) {
    //     println!("{:?}", x);
    // }

    // return;

    // let mut xy: Point = Point { x: 0, y: 0 };
    // loop {
    //     if let Event::Key(event) = read().unwrap() {
    //         xy = match event.code {
    //             KeyCode::Esc => break,
    //             key_code => update(xy, key_code),
    //         };
    //         println!("{:?} {:?}", event, xy);
    //     }
    // }
    
    // println!("Hello, world!");
}
