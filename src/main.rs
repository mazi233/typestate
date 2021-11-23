#[derive(Debug)]
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

    fn builder() -> UserBuilder<NoId, NoEmail> {
        UserBuilder::new()
    }
}

struct UserBuilder<I, E> {
    id: I,
    email: E,
    first_name: Option<String>,
    last_name: Option<String>
}

impl UserBuilder<NoId, NoEmail> {
    fn new() -> Self {
        Self {
            id: NoId,
            email: NoEmail,
            first_name: None,
            last_name: None,
        }
    }
}

impl<E> UserBuilder<NoId, E> {
    fn id(self, id: impl Into<String>) -> UserBuilder<Id, E> {
        let Self { email, first_name, last_name, .. } = self;
        UserBuilder {
            id: Id(id.into()),
            email,
            first_name,
            last_name,
        }
    }
}

impl<I> UserBuilder<I, NoEmail> {
    fn email(self, email: impl Into<String>) -> UserBuilder<I, Email> {
        let Self { id, first_name, last_name, .. } = self;
        UserBuilder {
            id,
            email: Email(email.into()),
            first_name,
            last_name,
        }
    }
}

impl<I, E> UserBuilder<I, E> {
    fn first_name(mut self, first_name: impl Into<String>) -> Self {
        self.first_name = Some(first_name.into());
        self
    }

    fn last_name(mut self, last_name: impl Into<String>) -> Self {
        self.last_name = Some(last_name.into());
        self
    }
}

impl UserBuilder<Id, Email> {
    fn build(self) -> User {
        let Self { id, email, first_name, last_name } = self;
        User {
            id: id.0,
            email: email.0,
            first_name,
            last_name,
        }
    }
}

struct Id(String);
struct NoId;

struct Email(String);
struct NoEmail;

fn main() {
    let greyblake = User::builder()
        .email("602281666@qq.com")
        .id("13")
        .first_name("ma")
        .last_name("zi")
        .build();
    
    println!("{:?}", greyblake);
}
