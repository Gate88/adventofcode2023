use std::{hash::Hash, collections::HashMap};

pub trait Grouper: IntoIterator {
    fn group_by<K, V, F>(self, mut f: F) -> HashMap<K, Vec<V>>
    where
        K: Eq + Hash,
        Self: IntoIterator<Item = V> + Sized,
        F: FnMut(&Self::Item) -> K,
    {
        let mut result: HashMap<K, Vec<V>> = HashMap::new();
        for item in self {
            let key = f(&item);
            if result.get(&key).is_none() {
                result.insert(key, vec![item]);
            } else {
                result.get_mut(&key).unwrap().push(item)
            }
        }
    
        result
    }
}

impl<T> Grouper for T where T: IntoIterator {}