use std::cell::RefCell;

use crate::v1::{
    model::{UserBuilder, UserModel},
    selector::UserSelector,
    traits::{Builder, Manager},
};

pub struct UserManager {
    _users: RefCell<Vec<UserModel>>,
    _current_user: Option<UserModel>,
}

impl Manager<UserModel> for UserManager {
    type Builder = UserBuilder;
    type Selector<'a> = UserSelector<'a>;

    fn new() -> Self {
        Self {
            _users: RefCell::new(Vec::new()),
            _current_user: None,
        }
    }

    fn builder(&self) -> Self::Builder {
        UserBuilder::new()
    }

    fn create(&self, builder: Self::Builder) -> Result<&Self, String> {
        let user = builder.build();

        match user {
            Ok(user) => {
                self._users.borrow_mut().push(user);

                Ok(self)
            }
            Err(err) => Err(err),
        }
    }

    fn select(&self) -> Self::Selector<'_> {
        UserSelector::new(self)
    }
}
