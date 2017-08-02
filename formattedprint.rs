//Exercises about print formatting
fn main(){
    let x = 10;
    let y = 20;

    println!("x = {}", x);  //{} is replaced with any argument
    println!("x = {0} and y = {1} and x= {0} again", x, y); //Multiple arguments
    println!("{first} {third} {second}", first = "A winner", second = "you", third = "is"); //Named arguments
    println!("{0} in binary is {0:b}",x); //Formatting comes after :
    println!("Padding! 1)'{x:>width$}' 2)'{x:>0width$}'",x = 10, width = 6);    //Padding

    println!("Months in a year: {:?}", 12); //:? is for debug printing
    //struct UnPrintable(i32);
    //println!("Unprintable: {:?}", UnPrintable);
    #[derive(Debug)]
    struct Printable(i32);
    println!("Printable: {:?}", Printable(2));

}
