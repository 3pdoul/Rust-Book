fn main() {
    println!("Hello, world!");
    another_fuciton(11, 'm');
}

fn another_fuciton(x: i32, unit_label: char){
    println!("the measurement is: {x} {unit_label}");
}
