pub trait Storage<T> {
    fn insert(&mut self, key: String, document: T) -> bool;
}
