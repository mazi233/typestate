struct User {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>
}

impl User {
    fn new(
        email: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>
    ) -> Self {
        Self { email, first_name, last_name }
    }
}

fn main() {
    let greyblake = User::new(
        Some("example@example.com".to_string()),
        Some("Sergey".to_string()),
        None,
    );
}
