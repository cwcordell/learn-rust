fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );

    user1.email = String::from("anotheremail@example.com");

    // add a user2 with same data as user1 except for email using struct update syntax
    // note that user1 is no longer valid since the username field was moved to user2
    // specifying a username field would not invalidate user1 because active and sign_in_count
    // implement the Copy trait, since they are scalar types
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
