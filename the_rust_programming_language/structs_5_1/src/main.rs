fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );

    user1.email = String::from("anotheremail@example.com");

    // add a user2 with same data as user1 except for email using struct update syntax
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
