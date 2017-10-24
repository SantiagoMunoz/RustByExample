#[derive(Debug)]
struct Rectangle{
    length: u32,
    width: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, rect : &Rectangle) -> bool {
        (rect.length < self.length) && (rect.width < self.width)
    }
}

fn main() {
    let rect1 = Rectangle{ length : 50, width : 30 };
    println!("Rect1 is {:?}", rect1);
    println!("The area of the rectangle is {}", rect1.area());
    let rect2 = Rectangle{ length : 20, width : 20};
    if rect1.can_hold(&rect2){
        println!("yay!");
    }else{
        println!("nay!");
    }
}


