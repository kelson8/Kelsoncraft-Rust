use crate::User;

/// Print values from the User struct in main.rs.
pub fn print_struct_values() {
    // Password hash for "Test"
    let test_hashed_password = "$2b$12$myvvDXW2yqZSmO9KuTYd6u6JlZIr5y/MX5H9s1LPu07fy5k1y.b4G";

    // Struct testing
    // https://doc.rust-lang.org/book/ch05-02-example-structs.html
    let test_user = User {
        email: String::from("test@test.com"),
        username: String::from("test"),
        hashed_password: String::from(test_hashed_password)
    };

    println!("{test_user:?}")

    // println!("Email: {}", test_user.email);
    // println!("Username: {}", test_user.username);
    // println!("Hashed Password: {}", test_user.hashed_password);
}

