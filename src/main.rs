use std::cell::RefCell;

struct UserModel {
    first_name: String,
    last_name: String,
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

    fn set_first_name(&mut self, value: String) -> &mut Self {
        self.first_name = Some(value);

        self
    }

    fn set_last_name(&mut self, value: String) -> &mut Self {
        self.last_name = Some(value);

        self
    }

    fn set_age(&mut self, value: u8) -> &mut Self {
        self.age = Some(value);

        self
    }
}

impl From<UserBuilder> for UserModel {
    fn from(value: UserBuilder) -> Self {
        let UserBuilder {
            first_name,
            last_name,
            age,
            is_active,
        } = value;

        Self {
            first_name: first_name.unwrap_or("".to_string()),
            last_name: last_name.unwrap_or("".to_string()),
            is_active: is_active.unwrap_or(true),
            age: age.unwrap_or_default(),
        }
    }
}

type Predicte<T> = Box<dyn Fn(&T) -> bool>;
type Filters<T> = Vec<Predicte<T>>;

enum Conector<T> {
    And(Filters<T>, Option<Box<Conector<T>>>),
    Or(Filters<T>, Option<Box<Conector<T>>>),
}

struct UserSelector<'a> {
    manager: &'a UserManager,
    filters: Conector<UserModel>,
}

impl<'a> UserSelector<'a> {
    fn new(manager: &'a UserManager) -> Self {
        Self {
            manager,
            filters: Conector::And(Vec::new(), None),
        }
    }

    fn where_first_name(mut self, value: &str) -> Self {
        let value = value.to_string();

        match &mut self.filters {
            Conector::And(f, _) | Conector::Or(f, _) => {
                f.push(Box::new(move |u| u.first_name == value));
            }
        }

        self
    }

    fn or(mut self) -> Self {
        self.filters = Conector::Or(Vec::new(), Some(Box::new(self.filters)));

        self
    }

    fn inspect(self) -> () {}
}

struct UserManager {
    _users: RefCell<Vec<UserModel>>,
    _current_user: Option<UserModel>,
}

impl UserManager {
    fn new() -> Self {
        Self {
            _users: RefCell::new(Vec::new()),
            _current_user: None,
        }
    }

    fn builder(&self) -> UserBuilder {
        UserBuilder::new()
    }

    fn create(&self, builder: UserBuilder) -> &UserManager {
        self._users.borrow_mut().push(builder.into());

        self
    }

    fn select() {}
}

fn main() {
    let manager = UserManager::new();

    let mut builder = manager.builder();

    builder
        .set_first_name("Franco".to_string())
        .set_last_name("Carrasco".to_string());

    manager.create(builder);

    let users = manager._users.borrow();

    let user = users.first();

    if let Some(user) = user {
        println!("The user is {}", user.first_name);
    } else {
        println!("Hello, world!");
    }

    let selector = UserSelector::new(&manager);

    selector
        .where_first_name("Franco")
        .or()
        .where_first_name("Mish");
}
