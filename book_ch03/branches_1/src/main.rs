fn main() {
    let number =  {
        let x = 3;
        x + 1
    };

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let condition = true;
    let var = if condition {"TRUE"} else {"FALSE"};
    println!("Condition is : {}",var);
}