pub trait ToBox<T> {
    fn to_box(&self) -> Box<T>;
}

impl<T> ToBox<T> for T
where
    T: Clone,
{
    fn to_box(&self) -> Box<T> {
        return Box::new(self.clone());
    }
}
