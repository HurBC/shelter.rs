use std::cell::RefCell;

use crate::{
    testing::v2::{
        model::{User, UserBuilder},
        selector::UserSelector,
    },
    v2::{builder::Builder, manager::Manager, selector::Selector},
};

pub struct UserManager {
    users: RefCell<Vec<User>>,
}

impl Manager<User> for UserManager {
    type Builder = UserBuilder;

    type Selector<'a>
        = UserSelector<'a>
    where
        Self: 'a;

    fn new() -> Self {
        Self {
            users: RefCell::new(Vec::new()),
        }
    }

    fn builder(&self) -> Self::Builder {
        Self::Builder::new()
    }

    fn create(&self, builder: Self::Builder) -> bool {
        let is_created = builder.build();

        if let Ok(user) = is_created {
            let mut borrowed_users = self.users.borrow_mut();

            borrowed_users.push(user);

            return true;
        }

        false
    }

    fn b_create(&self, builders: Vec<Self::Builder>) -> bool {
        let users: Vec<Result<User, String>> = builders.into_iter().map(|b| b.build()).collect();

        let mut created_users: Vec<User> = users.into_iter().flatten().collect();

        if !created_users.is_empty() {
            let mut borrowed_users = self.users.borrow_mut();

            borrowed_users.append(&mut created_users);

            true
        } else {
            false
        }
    }

    fn get_objects(&self) -> &std::cell::RefCell<std::vec::Vec<User>> {
        &self.users
    }

    fn selector(&self) -> Self::Selector<'_> {
        Self::Selector::new(self)
    }
}
