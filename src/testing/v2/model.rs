use chrono::NaiveDate;

use crate::{testing::v2, v2::builder::Builder};

pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub birthday: Option<NaiveDate>,
    pub is_active: bool,
}

pub struct UserBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    birthday: Option<NaiveDate>,
}

impl UserBuilder {
    pub fn first_name<S: Into<String>>(mut self, value: S) -> Self {
        self.first_name = Some(value.into());

        self
    }
    pub fn last_name<S: Into<String>>(mut self, value: S) -> Self {
        self.last_name = Some(value.into());

        self
    }

    pub fn birthday(mut self, value: &str) -> Self {
        println!("SETTING BDAY WITH {}", value);

        let date = NaiveDate::parse_from_str(value, "dd-mm-yyyy");

        if let Ok(date) = date {
            println!("Mih");

            self.birthday = Some(date);
        }

        self
    }
}

impl Builder<User> for UserBuilder {
    fn build(self) -> Result<User, String> {
        Ok(User {
            first_name: self.first_name.ok_or("The first name is required")?,
            last_name: self.last_name.ok_or("The last name is required")?,
            birthday: self.birthday,
            is_active: true,
        })
    }

    fn new() -> Self {
        Self {
            first_name: None,
            last_name: None,
            birthday: None,
        }
    }
}
