struct User {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>
}

impl User {
    // fn new(
    //     email: Option<String>,
    //     first_name: Option<String>,
    //     last_name: Option<String>
    // ) -> Self {
    //     Self { email, first_name, last_name }
    // }
    fn builder() -> UserBuilder {
        UserBuilder::new()
    }
}

struct UserBuilder {
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>
}

impl UserBuilder {
    fn new() -> Self {
        Self {
            email: None,
            first_name: None,
            last_name: None,
        }
    }

    fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = Some(first_name.into());
        self
    }

    fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    fn build(self) -> User {
        let Self { email, first_name, last_name } = self;
        User { email, first_name, last_name }
    }
}

fn main() {
    let greyblake = User::builder()
        .email("example@example.com")
        .first_name("Sergey")
        .build();
}
