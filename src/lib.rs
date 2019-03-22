use std::hash::Hash;
use std::collections::HashMap;


pub struct CountCache<K, V> {
    cache: HashMap<K, (V, usize)>,
}


impl<K, V> CountCache<K, V> 
where K: Eq + Hash + Clone,
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
    fn it_works() {
        let mut ccache = CountCache::new();

        ccache.insert("test", 10, 2);
        assert_eq!(ccache.get(&"test").expect("Err"), 10);
        assert_eq!(ccache.get(&"test").expect("Err"), 10);
        assert!(ccache.get(&"test").is_err());
        assert!(ccache.get(&"test").is_err());
    }
}
