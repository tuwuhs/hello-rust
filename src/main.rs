
use getch::Getch;

fn main() {
    let _a = 25;
    let b = [1, 2, 3, 4];
    println!("Hello, world! {:?}", b);

    let key: u8 = Getch::new().getch().unwrap();
    println!("{}", key);
}
