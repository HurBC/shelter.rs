use std::cell::RefCell;

mod domain;
mod v1;

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

impl<T> Conector<T> {
    fn apply(&self, value: &T) -> bool {
        match self {
            Conector::And(items, conector) => {
                let has_match = items.iter().all(|f| f(&value));

                if !has_match {
                    return false;
                }

                match conector {
                    Some(conector) => conector.apply(value),
                    None => has_match,
                }
            }
            Conector::Or(items, conector) => {
                let has_match = items.iter().any(|f| f(&value));

                if has_match {
                    return true;
                }

                match conector {
                    Some(conector) => conector.apply(value),
                    None => has_match,
                }
            }
        }
    }

    fn inspect(self) {
        if let Conector::And(_, _) = self {
            println!("AND CONNECTOR");
        }

        if let Conector::Or(_, _) = self {
            println!("OR CONNECTOR");
        }

        match self {
            Conector::And(items, connect) | Conector::Or(items, connect) => {
                println!("FILTERS LENGHT {}", items.len());

                if let Some(connector) = connect {
                    connector.inspect();
                }
            }
        }
    }
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

    fn where_last_name(mut self, value: &str) -> Self {
        let value = value.to_string();

        match &mut self.filters {
            Conector::And(f, _) | Conector::Or(f, _) => {
                f.push(Box::new(move |u| u.last_name == value));
            }
        }

        self
    }

    fn where_age(mut self, value: u8) -> Self {
        match &mut self.filters {
            Conector::And(f, _) | Conector::Or(f, _) => {
                f.push(Box::new(move |u| u.age == value));
            }
        }

        self
    }

    fn or(mut self) -> Self {
        self.filters = Conector::Or(Vec::new(), Some(Box::new(self.filters)));

        self
    }

    fn inspect(self) {
        self.filters.inspect();
    }

    fn for_each<F>(self, mut f: F)
    where
        F: FnMut(&UserModel),
    {
        let borrowed = self.manager._users.borrow();

        borrowed
            .iter()
            .filter(|u| self.filters.apply(*u))
            .for_each(|u| f(u));
    }
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

    fn select(&self) -> UserSelector<'_> {
        UserSelector::new(self)
    }
}

fn main() {
    let manager = UserManager::new();

    let mut builder = manager.builder();

    builder
        .set_first_name("Hur".to_string())
        .set_last_name("C".to_string());

    manager.create(builder);

    let mut builder = manager.builder();

    builder
        .set_last_name("C".to_string())
        .set_first_name("Bru".to_string())
        .set_age(20);

    manager.create(builder);

    let mut builder = manager.builder();

    builder
        .set_first_name("Ignacio".to_string())
        .set_last_name("Flores".to_string())
        .set_age(20);

    manager.create(builder);

    manager
        .select()
        // .where_first_name("Hur")
        // .or()
        .where_age(20)
        .for_each(|user| println!("USER {}", user.first_name));
}
