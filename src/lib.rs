use std::hash::Hash;
use std::collections::{HashMap, hash_map::Entry};


pub struct CountCache<K, V> {
    cache: HashMap<K, (V, usize)>,
}


impl<K, V> CountCache<K, V> 
where K: Clone + Eq + Hash,
      V: Clone
{
    pub fn new() -> CountCache<K, V> {
        CountCache {
            cache: HashMap::new()
        }
    }

    pub fn insert(&mut self, key: K, value: V, count: usize) {
        self.cache.insert(key, (value, count));
    }

    pub fn increment(&mut self, key: &K, by: usize) {
        if let Some((_, count)) = self.cache.get_mut(key) {
            *count += by;
        }
    }


    pub fn has_key(&mut self, key: K) -> bool {
        match self.cache.entry(key) {
            Entry::Occupied(_) => true,
            Entry::Vacant(_) => false,
        }
    }


    pub fn get(&mut self, key: &K) -> Result<V, ()> {
        let result = if let Some((value, count)) = self.cache.get_mut(key) {
            *count -= 1;
            let result = Ok(value.clone());
            result
        } else {
            Err(())
        };
        if let Some((_, count)) = self.cache.get(key) {
            if *count == 0 {
                self.cache.remove(key);
            }
        } 
        result
    }
}

#[cfg(test)]
mod tests {
    use super::CountCache;

    #[test]
    fn test_increment_and_count() {
        let mut ccache = CountCache::new();

        ccache.insert("test", 10, 2);
        assert!(ccache.has_key("test"));
        assert_eq!(ccache.get(&"test").expect("Err"), 10);
        ccache.increment(&"test", 1);
        assert_eq!(ccache.get(&"test").expect("Err"), 10);
        assert_eq!(ccache.get(&"test").expect("Err"), 10);
        assert!(ccache.get(&"test").is_err());
    }
}
