fn main() {
    let x = 5;
    println!("Variable x is ; {x}");
    //x = 10; // This line will cause a compile-time error because `x` is immutable by default.
    //println!("Variable x is ; {x}");

    let mut y = 10; // this variable can be changed
    println!("Variable y is ; {y}");
    y = 20;
    println!("Variable y is : {y}");
}
