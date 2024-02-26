struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    println!("{}, {}, {}, {}", user1.active, user1.username, user1.email, user1.sign_in_count);

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123")
    );

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{}, {}, {}, {}", user2.active, user2.username, user2.email, user2.sign_in_count);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 1,
    // }
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
