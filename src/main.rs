struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user(email: String, username: String) -> User {
    User { active: true, username, email, sign_in_count: 1 }
}
fn main() {
    let mut user1 = build_user(String::from("hi"), String::from("hi"));

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("Hi {}", user1.email)
}
