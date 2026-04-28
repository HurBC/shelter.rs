use crate::v1::{
    manager::UserManager,
    traits::{Manager, Selector},
};

mod domain;
mod v1;

fn main() {
    let manager = UserManager::new();

    let mut builder = manager.builder();

    builder
        .set_first_name("Hur".to_string())
        .set_last_name("C".to_string());

    let _ = manager.create(builder);

    let mut builder = manager.builder();

    builder
        .set_last_name("C".to_string())
        .set_first_name("Bru".to_string());

    let _ = manager.create(builder);

    let mut builder = manager.builder();

    builder
        .set_first_name("Ignacio".to_string())
        .set_last_name("Flores".to_string());

    let _ = manager.create(builder);

    manager
        .select()
        .where_first_name("Hur")
        .or()
        .where_last_name("Flores")
        .for_each(|user| println!("USER {}", user.first_name));
}
