fn main() {
    let s = String::from("Hello world!");
    println!("the first word in {} is {}",s, &s[0..first_word(&s)]);


    println!("the first word in {} is {}",s, first_word_ref(&s));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_ref(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[0..s.len()]
}

