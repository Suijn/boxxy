pub trait Storage<T> {
    fn insert(&mut self, key: String, document: T) -> bool;
    fn get(&self, key: String) -> Option<&T>;
}
