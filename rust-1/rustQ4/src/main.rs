struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() { 
    let rect = Rect {
        width: 30,
        height: 50,
    };
    println!("The area of the reactangle is {}", rect.area());
}