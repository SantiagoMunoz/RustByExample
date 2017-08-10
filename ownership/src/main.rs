fn main() {

    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}",s);

    let s2 = s.clone(); //Otherwise ownership gets moved and s stops being valid

    println!("{}", s);

    let x = 1;
    let y = x; //Stack vars do not suffer from this (size known at compile time)
    println!("{}", x);
    println!("{}", y);

    take_ownership(s2);
    // println!("{}", s2); //Cant do this since object was moved to the function and not returned

    let s3 = take_n_return_ownership(s);
    println!("{}", s3);
    s = give_ownership();
    let mut s4 = String::from("asd");
    let mut s5 = String::from("df");
    let (s6, s7) = ownership_move_with_tuple(s4,s5);

}

fn take_ownership(some_string : String){
    println!("Function: {}",some_string);
}

fn take_n_return_ownership(some_string: String) -> String{
    println!("Function: {}",some_string);
   
    some_string
}

fn give_ownership() -> String{
    let some_string = String::from("Otra cosa");
    some_string
}

fn ownership_move_with_tuple(m_string : String, m_string_2: String) -> (String, String){
    //yadda yadda
    (m_string, m_string_2)
}
