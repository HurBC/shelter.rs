pub trait Builder<TModel> {
    fn build(self) -> Result<TModel, String>;
}

pub trait Manager<TModel> {
    type Builder: Builder<TModel>;
    type Selector<'a>: Selector<TModel>
    where
        Self: 'a;

    fn new() -> Self;

    fn builder(&self) -> Self::Builder;
    fn create(&self, builder: Self::Builder) -> Result<&Self, String>;
    fn select(&self) -> Self::Selector<'_>;
}

pub trait Selector<TModel> {
    fn or(self) -> Self;
    fn and(self) -> Self;
    fn for_each<F>(self, f: F)
    where
        F: FnMut(&TModel);
}
