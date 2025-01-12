fn main() {
    let s = String::from("hello, world");
    take_ownership(s.clone());
    let x = 5;
    make_copy(x);
    take_ownership(s);
}

fn take_ownership(some_string: String){
    println!("{some_string}");
}

fn make_copy(some_int: i32){
    println!("{some_int}");
}
