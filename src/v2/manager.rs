use std::cell::RefCell;

use crate::v2::{builder::Builder, selector::Selector};

pub trait Manager<TModel> {
    type Builder: Builder<TModel>;
    type Selector<'a>: Selector<'a, TModel>
    where
        Self: 'a;

    fn new() -> Self;

    // Provides methods to create Entetity instances
    fn builder(&self) -> Self::Builder;
    fn create(&self, builder: Self::Builder) -> bool;
    fn b_create(&self, builders: Vec<Self::Builder>) -> bool;

    fn get_objects(&self) -> &RefCell<Vec<TModel>>;

    // Provide Selector for T
    fn selector(&self) -> Self::Selector<'_>;
}
