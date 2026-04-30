pub trait Builder<TModel> {
    fn new() -> Self;

    fn build(self) -> Result<TModel, String>;
}
