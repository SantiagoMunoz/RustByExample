enum myenum{
    index(i32),
    value(f64),
    description(String),
}

fn main() {

    //Oldschool way
    let mut v1 : Vec<i32> = Vec::new();

    v1.push(32);
    v1.push(14);
    v1.push(7);
    
    //Macro to create vector with initial values. Infers type.
    let v2 = vec![14, 15, 16];

    let elem_three : &i32  = &v1[2];
    let elem_two : Option<&i32> = v1.get(1);
    
    //The following crashes the program
    //let nonexistent : &i32 = &v1[9];

    //The following does not crash
    let nonexistent = v1.get(27);

    let mut v3 = vec![
        myenum::index(1), 
        myenum::value(32),
        myenum::description(String::from("I feel good"))];
}
