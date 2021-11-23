struct User {
    id: String,
    email: String,
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

    fn builder(id: impl Into<String>, email: impl Into<String>) -> UserBuilder {
        UserBuilder::new(id, email)
    }
}

struct UserBuilder {
    id: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>
}

impl UserBuilder {
    fn new(id: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            email: email.into(),
            first_name: None,
            last_name: None,
        }
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
        let Self { id, email, first_name, last_name } = self;
        User { id, email, first_name, last_name }
    }
}

fn main() {
    let greyblake = User::builder("13", "greyblake@example.com")
        .first_name("Sergey")
        .build();
}
