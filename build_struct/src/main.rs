struct User {
    active: bool,
    username: String,
    email: String, 
    sign_in_count: u64,
}
struct TupleStruct(i32, String, i64);
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Inspector"),
        email: String::from("email@example.com"),
        sign_in_count: 1,
    };
    println!("{0}", user1.active);
    
    let user2 = User {
        active: user1.active,
        username: user1.username,
        ..user1
    };
    println!("{0}", user2.email);
    user1.active = false;
    println!("{0}", user2.active);
    let t = TupleStruct(12, String::from("tuple_Struct"), 14);
}
