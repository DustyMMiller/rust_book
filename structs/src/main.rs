fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("User '{}', with email '{}' has {} logins",
    user1.username, user1.email, user1.sign_in_count);

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("User '{}', with email '{}' has {} logins",
    user2.username, user2.email, user2.sign_in_count);

    let user3 = User {
        email: String::from("yetanother@example.com"),
        ..user2
    };
    println!("User '{}', with email '{}' has {} logins",
    user3.username, user3.email, user3.sign_in_count);
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
