pub enum State<T> {
    Start,
    Middle(T),
    End,
}

impl<T> From<T> for State<T> {
    fn from(it: T) -> Self {
        Self::Middle(it)
    }
}
