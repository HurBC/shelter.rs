use chrono::NaiveDate;

use crate::v1::{conector::Connector, manager::UserManager, model::UserModel, traits::Selector};

pub struct UserSelector<'a> {
    manager: &'a UserManager,
    filters: Connector<UserModel>,
}

impl<'a> UserSelector<'a> {
    pub fn new(manager: &'a UserManager) -> Self {
        Self {
            manager,
            filters: Connector::And(Vec::new(), None),
        }
    }

    pub fn where_first_name(mut self, value: &str) -> Self {
        let value = value.to_string();

        match &mut self.filters {
            Connector::And(f, _) | Connector::Or(f, _) => {
                f.push(Box::new(move |u| u.first_name == value));
            }
        }

        self
    }

    pub fn where_last_name(mut self, value: &str) -> Self {
        let value = value.to_string();

        match &mut self.filters {
            Connector::And(f, _) | Connector::Or(f, _) => {
                f.push(Box::new(move |u| u.last_name == value));
            }
        }

        self
    }

    pub fn where_birth_date(mut self, value: &str) -> Self {
        let value = Some(NaiveDate::parse_from_str(value, "%d-%m-%Y").expect("Invalid date"));

        match &mut self.filters {
            Connector::And(f, _) | Connector::Or(f, _) => {
                f.push(Box::new(move |u| {
                    if let (Some(user_bd), Some(value)) = (u.birth_date, value) {
                        user_bd == value
                    } else {
                        false
                    }
                }));
            }
        }

        self
    }
}

impl<'a> Selector<UserModel> for UserSelector<'a> {
    fn or(mut self) -> Self {
        self.filters = Connector::Or(Vec::new(), Some(Box::new(self.filters)));

        self
    }

    fn for_each<F>(self, f: F)
    where
        F: FnMut(&UserModel),
    {
        let borrowed = self.manager._users.borrow();

        borrowed
            .iter()
            .filter(|u| self.filters.apply(*u))
            .for_each(f);
    }

    fn and(mut self) -> Self {
        self.filters = Connector::And(Vec::new(), Some(Box::new(self.filters)));

        self
    }
}
