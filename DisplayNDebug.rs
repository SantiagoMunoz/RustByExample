use std::fmt;   //Import fmt

#[derive(Debug)]
struct MinMax(i32, i32);

//Implement Display for Point
impl fmt::Display for MinMax{
    fn fmt(&self, f : &mut fmt::Formatter) ->fmt::Result{
        write!(f,"({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D{
    x: f64,
    y: f64,
}
    
impl fmt::Display for Point2D{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({},{})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Complex{
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex{
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result{
        write!(f,"{}+{}i",self.real, self.imag)
    }
}

fn main(){
    let minmax = MinMax(0, 12);

    println!("Minmax struct");
    println!("  Display: {}", minmax);
    println!("  Debug: {:?}", minmax);

    let point2d = Point2D{x : 2.2, y: 4.4};

    println!("Point2d struct");
    println!("  Display: {}", point2d);
    println!("  Debug: {:?}", point2d);
    
    let complex = Complex{real: 1.0, imag: 2.5};

    println!("Point2d struct");
    println!("  Display: {}", complex);
    println!("  Debug: {:?}", complex);

}
