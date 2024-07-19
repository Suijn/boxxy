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

    fn get(&self, key: String) -> Option<&T> {
        let document = self.db.get(&key);
        match document {
            None => None,
            Some(document) => Some(document),
        }
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

    #[test]
    fn get() {
        let mut storage = InMemoryStorage { db: HashMap::new() };
        storage.insert("foo".to_string(), "bar");
        let value = storage.get("foo".to_string());
        assert_eq!(value, Some(&"bar"));
    }

    #[test]
    fn get__returns_none__key_not_found() {
        let storage: InMemoryStorage<String> = InMemoryStorage { db: HashMap::new() };
        let value = storage.get("non_existing_key".to_string());
        assert_eq!(value, None);
    }
}
