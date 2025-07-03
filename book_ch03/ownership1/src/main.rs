fn main() {
    // type string STACK
    let str1 = "Hello world";
    println!("{}", str1);

    // String type uses HEAP
    let mut str_string = String::from("Duck ");
    str_string.push_str("tales");

    println!("{}", str_string);

    copy_symple_types();
    copy_complex_types();

    clone_data();
}

fn copy_symple_types() {
    let x = 5;
    let y = x;
    println!("x = {} , y = {}", x, y);
}

fn copy_complex_types() {
    let name = String::from("Jack");
    let name1 = name;

    // name is not valid anymore. Only name1
    println!("name  - {}", name1);
}

fn clone_data() {
    let name = String::from("Jack");
    let mut name1 = name.clone();

    name1.insert_str(0, "Hello ");
    println!("name  - {}", name);
    println!("name1  - {}", name1);
}
