struct User {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>
}

fn main() {
    let greyblake = User {
        email: Some("example@example.com".to_string()),
        first_name: Some("Sergey".to_string()),
        last_name: None,
    };
}
