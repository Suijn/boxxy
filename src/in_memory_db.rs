use crate::traits::Storage;
use std::collections::HashMap;

pub struct InMemoryStorage<T> {
    db: HashMap<String, T>,
}

impl<T> Storage<T> for InMemoryStorage<T> {
    fn insert(&mut self, key: String, document: T) -> bool {
        self.db.insert(key, document);
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert() {
        let mut storage = InMemoryStorage { db: HashMap::new() };
        storage.insert("foo".to_string(), "bar");
        let document = storage.db.get("foo");
        assert_eq!(document, Some(&"bar"));
    }
}
