struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}


fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user1_1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1_1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("test@mail.ru"), String::from("test user"));

    let user2 = User{
        email: String::from("another@mail.ru"),
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User { 
        active: true, 
        username, 
        email, 
        sign_in_count: 1
    }
}
