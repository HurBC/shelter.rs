use std::cell::Ref;

use crate::v2::{logical::Filters, manager::Manager};

pub trait Selector<'manager, TModel> {
    type Manager: Manager<TModel>;

    fn new(manager: &'manager Self::Manager) -> Self;

    // Read only methods
    fn for_each<F>(self, f: F)
    where
        F: FnMut(&TModel);

    fn first(self) -> Option<Ref<'manager, TModel>>;
    fn pick_firsts(self, count: usize) -> &'manager [TModel];
    fn last(self) -> Option<Ref<'manager, TModel>>;
    fn pick_lasts(self, count: usize) -> &'manager [TModel];

    fn filters(&mut self) -> &mut Filters<TModel>;

    // Update methods

    // Update all items
    // fn update_a<F>(self, f: F)
    // where
    //     F: FnMut(&mut TModel);
}
