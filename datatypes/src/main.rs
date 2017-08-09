
const PI : f32 = 3.14;

fn main() {
    let tupl : (u32, u64, char) = (2, 45, 'B');     //Tuples can have members of multiple data types
    println!("The first member of tupl is {}",tupl.0);
    hobbies_fn(0);
    expression_n_scope();
    println!("Five is {}", return_five());
}

fn hobbies_fn(x:usize){
    let index = ["first", "second", "third"];
    let hobbies = ["drawing", "biking", "programming"];     //Arrays must be of a single type and cannot grow/shrink
    println!("My {} hobby is {}",index[x], hobbies[x]);
}

fn expression_n_scope(){
    let y = {
        let x = 1;
        x+1
    };
    println!("Value of y is {}", y);

}

fn return_five() ->u32 {
    5
}
