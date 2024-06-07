struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = build_user("asd".to_string(), "asd".to_string());
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("someusername123"),
        ..user1 // must come last to specify that any remaining fields
    };

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1 // Both active and sign_in_count are types that implement the Copy trait
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }

    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
