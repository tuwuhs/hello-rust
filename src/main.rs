
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
struct Adjust<I, F> {
    iter: I,
    idx: usize,
    f: F,
    count: usize,
}

impl<B, I: Iterator, F> Iterator for Adjust<I, F>
where
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        // let a = self.iter.next()?;
        // let i = self.count;
        // self.count += 1;
        // if i == self.idx {
        //     Some((&mut self.f)(a))
        // } else {
        //     Some((&mut self.f)(a - 1))
        // }
        
        self.iter.next().map(&mut self.f)
    }
}

impl<B, I: Iterator, F> Adjust<I, F> 
where
    F: FnMut(I::Item) -> B
{
    fn new(iter: I, idx: usize, f: F) -> Adjust<I, B> {
        let g = |x| f(x);
        Adjust { iter, idx, f: g, count: 0 }
    }
}

trait Adjustable {
    fn adjust<F>(self, idx: usize, f: F) -> Adjust<Self, F> where Self: Sized;
}

// impl<I> Adjustable for I where I: Iterator, I: Sized {
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

    for x in Adjust::new(a.iter(), 2, |x| 2*x) {
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
