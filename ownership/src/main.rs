fn main() {

    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}",s);

    let s2 = s.clone(); //Otherwise ownership gets moved and s stops being valid

    println!("{}", s);

    let x = 1;
    let y = x; //Stack vars do not suffer from this (size known at compile time)
    println!("{}", x);
}
