use std::cell::Ref;

use crate::{
    testing::v2::{manager::UserManager, model::User},
    v2::{logical::Filters, manager::Manager, selector::Selector},
};

pub struct UserSelector<'manager> {
    filters: Filters<User>,
    manager: &'manager UserManager,
}

impl<'manager> Selector<'manager, User> for UserSelector<'manager> {
    type Manager = UserManager;

    fn new(manager: &'manager Self::Manager) -> Self {
        Self {
            filters: Filters::new(),
            manager,
        }
    }

    fn for_each<F>(self, f: F)
    where
        F: FnMut(&User),
    {
        let borrowed_users = self.manager.get_objects().borrow();

        borrowed_users
            .iter()
            .filter(|u| self.filters.apply(*u))
            .for_each(f);
    }

    fn first(self) -> Option<Ref<'manager, User>> {
        let borrowed_users = self.manager.get_objects().borrow();

        let index = borrowed_users.iter().position(|u| self.filters.apply(u))?;

        Some(Ref::map(borrowed_users, |users| &users[index]))
    }

    fn pick_firsts(self, count: usize) -> &'manager [User] {
        todo!()
    }

    fn last(self) -> Option<Ref<'manager, User>> {
        let borrowed_users = self.manager.get_objects().borrow();

        let index = borrowed_users.iter().rposition(|u| self.filters.apply(u))?;

        Some(Ref::map(borrowed_users, |users| &users[index]))
    }

    fn pick_lasts(self, count: usize) -> &'manager [User] {
        todo!()
    }

    fn filters(&mut self) -> &mut Filters<User> {
        &mut self.filters
    }
}
