struct User {
    // 'fields' of the struct
    username: String,
    email: String,
    sign_in_count: u32,
    active: bool,
}

fn main() {
    let _u = build_user(String::from("ric.waltman@protonmail.com"), String::from("rwaltman"));

    let _u2 = User {
        email: String::from("test@a.com"),
        username: String::from("test"),
        .._u // struct update syntax: updates remaining fields
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        // 'field init shorthand' initializes struct field names
        // that match parameter names
        email,
        username,

        // Regularly assigning fields
        active: true,
        sign_in_count: 1,
    }
}
