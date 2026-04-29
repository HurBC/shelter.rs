use chrono::{DateTime, NaiveDate, Utc};

use crate::v1::traits::Builder;

pub struct UserModel {
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

pub struct UserBuilder {
    first_name: Option<String>,
    last_name: Option<String>,
    birth_date: Option<NaiveDate>,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            first_name: None,
            last_name: None,
            birth_date: None,
        }
    }

    pub fn set_first_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.first_name = Some(value.into());

        self
    }

    pub fn set_last_name<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.last_name = Some(value.into());

        self
    }

    pub fn set_birth_date(&mut self, value: &str) -> &mut Self {
        self.birth_date = Some(NaiveDate::parse_from_str(value, "%d-%m-%Y").expect("Invalid date"));

        self
    }
}

impl Builder<UserModel> for UserBuilder {
    fn build(self) -> Result<UserModel, String> {
        let first_name = self.first_name.ok_or("The first name is missing")?;

        let last_name = self.last_name.ok_or("The last name is missing")?;

        Ok(UserModel {
            first_name,
            last_name,
            birth_date: self.birth_date,
            created_at: Utc::now(),
        })
    }
}
