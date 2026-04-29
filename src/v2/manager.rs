use std::cell::RefCell;

pub trait Manager<T> {
    type Builder: ToString;
    type Selector<'a>: Selctor
    where
        Self: 'a;

    fn new() -> Self;

    // Provides methods to create Entetity instances
    fn builder(&self) -> Self::Builder;
    fn create(&self, builder: Self::Builder) -> Result<bool, String>;
    fn b_create(&self, builders: Vec<Self::Builder>) -> Result<bool, String>;

    fn get_objects(&self) -> RefCell<Vec<T>>;

    // Provide Selector for T
    fn selector(&self) -> Self::Selector<'_>;
}
