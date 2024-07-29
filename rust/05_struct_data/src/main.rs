struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("khhandrea"),
        email: String::from("khhandrea@gmail.com"),
        sign_in_count: 1,
    };

    let user2 = build_user(String::from("khhandrea"), String::from("khhandrea@gmail.com"));

    let user3 = User {
        email: String::from("khhandrea2@gmail.com"),
        ..user1
    };
}
