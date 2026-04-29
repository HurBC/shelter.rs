use crate::v2::logical::FiltersTrait;

pub trait Selector<'manager, TModel>: FiltersTrait {
    // Read only methods
    fn for_each<F>(self, f: F)
    where
        F: FnMut(&TModel);

    fn get_first(self) -> &'manager TModel;
    fn pick_first(self, count: usize) -> &'manager [TModel];
    fn get_last(self) -> &'manager TModel;
    fn pick_n_last(self, count: usize) -> &'manager [TModel];

    // Update methods

    // Update all items
    // fn update_a<F>(self, f: F)
    // where
    //     F: FnMut(&mut TModel);
}
