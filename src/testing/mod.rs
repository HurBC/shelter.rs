use crate::{
    testing::v2::{manager::UserManager, model::UserBuilder},
    v2::{builder::Builder, manager::Manager, selector::Selector},
};

pub mod v2;

pub fn example() {
    let builder = UserBuilder::new()
        .birthday("hola, soy goku")
        .first_name("Hur")
        .last_name("C");

    let bru = UserBuilder::new()
        .first_name("Bru")
        .last_name("C")
        .birthday("21-04-2005");

    let manager = UserManager::new();

    let mish = manager.builder().last_name("h").first_name("mish");

    manager.create(mish);

    manager.b_create(vec![builder, bru]);

    let mut selector = manager.selector();

    selector
        .filters()
        .push(Box::new(move |u| u.first_name == "Hur"))
        .or()
        .push(Box::new(move |u| u.first_name == "mish"))
        .push(Box::new(move |u| u.first_name == "Bru"));

    let user = selector.last();

    if let Some(user) = user {
        println!("USER {}", user.first_name)
    }

    // selector.for_each(|u| println!("User {}", u.first_name));
}
