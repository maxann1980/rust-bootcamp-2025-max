fn main() {
    let s1 = String::from("Hello world!");
    println!("size of s1 |{}| is {}", s1, get_str_len(&s1));
}

fn get_str_len(s: &str) -> usize {
    s.len()
}
