use std::collections::HashMap;

pub struct InMemoryStorage<T> {
    db: HashMap<String, T>,
}
