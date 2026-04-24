struct UserModel {
    first_name: &'static str,
    last_name: &'static str,
    is_active: bool,
    age: u8,
}

struct UserBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    is_active: Option<bool>,
    age: Option<u8>,
}

impl UserBuilder {
    fn new() -> Self {
        Self {
            first_name: None,
            last_name: None,
            is_active: None,
            age: None,
        }
    }

    fn set_first_name() {}
}

enum UserManagerList {
    Node(UserModel, Box<UserManagerList>),
    None,
}

struct UserManager {
    _users: UserManagerList,
    _current_user: Option<UserModel>,
}

impl UserManager {
    fn new() -> Self {
        Self {
            _users: UserManagerList::None,
            _current_user: None,
        }
    }

    fn builder() {}
}

fn main() {
    println!("Hello, world!");
}
