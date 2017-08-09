fn main() {

    if more_than_5(4) == true {
        println!("Yay!");
    }else{
        println!("Nay!");
    }
    
    let text = if more_than_5(5){
        "Yay!"
    }else{
        "Nay!"
    };
    println!("{}",text);

    while_loop();
    super_for();
    super_for_2();
}

fn more_than_5(x: u32) -> bool{
    if x > 5 {
        true
    }else{
        false
    }
}

fn while_loop(){
    let mut var: u8 = 0;
    while var < 10{
        var = var + 1;
        println!("The value of var is {}", var);
    }
}

fn super_for(){
    let x = ["my", "body", "is", "ready"];

    for elem in x.iter(){
        println!("{}", elem);
    }
}

fn super_for_2(){
    for i in 0..4{
        println!("{}", i);
    }

}
