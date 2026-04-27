use crate::{
    Conector,
    v1::{manager::UserManager, model::UserModel, traits::Selector},
};

pub struct UserSelector<'a> {
    manager: &'a UserManager,
    filters: Conector<UserModel>,
}

impl<'a> UserSelector<'a> {
    pub fn new(manager: &'a UserManager) -> Self {
        Self {
            manager,
            filters: Conector::And(Vec::new(), None),
        }
    }
}

impl<'a> Selector<UserModel> for UserSelector<'a> {
    fn or(self) -> Self {
        todo!()
    }

    fn for_each<F>(self, f: F)
    where
        F: FnMut(&UserModel),
    {
        todo!()
    }
}
