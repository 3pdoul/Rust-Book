fn main() {
    let mut s1 = String::from("hello");
    let y = first_word(&s1);
    println!("{y}");
    s1.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
        return &s[0..i];
    }
    }
    &s[..]
}
