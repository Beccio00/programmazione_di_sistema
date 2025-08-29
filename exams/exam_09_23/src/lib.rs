mod cache {
    use std::{collections::HashMap, hash::Hash, sync::{Arc, Mutex}, time::{Duration, Instant}};

    pub struct Cache<K: Eq+Hash, V: Clone> {
        data: Mutex<HashMap<K, (V, Instant)>>,
    }

    impl <K: Eq+Hash, V: Clone> Cache <K, V> {
        pub fn new() -> Self {
            Cache { data: Mutex::new(HashMap::new()) }
        }

        pub fn size(&self) -> usize {
            let data = self.data.lock().unwrap();
            let data_len = data.len();
            drop(data);
            data_len
        }

        pub fn put(&self, k: K, v: V, d: Duration) -> bool {
            let mut data = self.data.lock().unwrap();

            data.insert(k, (v, Instant::now() + d)).is_some()
        }

        pub fn renew(&self, k: &K, d: Duration) -> bool {
            let mut data = self.data.lock().unwrap();

            if let Some(value) = data.get_mut(k) {
                if value.1 > Instant::now() {
                   value.1 = Instant::now() + d;
                   return true; 
                } 
            }

            false
        }

        pub fn get(&self, k: &K) -> Option<Arc<V>> {
            let data = self.data.lock().unwrap();

            match data.get(k) {
                Some((v, d)) => {
                    if *d > Instant::now() {
                        Some(Arc::new(v.clone())) // ✅ Clona il valore
                    } else {
                        None
                    }
                },
                None => None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::cache::Cache;
    use std::{sync::Arc, thread, time::Duration};

    #[test]
    fn test_cache_with_rwlock() {
        let cache = Arc::new(Cache::new());
        
        // Test put e get
        cache.put("key1".to_string(), "value1".to_string(), Duration::from_secs(1));
        
        let result = cache.get(&"key1".to_string());
        assert!(result.is_some());
        assert_eq!(*result.unwrap(), "value1");
        
        // Test size
        assert_eq!(cache.size(), 1);
        
        // Test letture concorrenti
        let mut handles = vec![];
        
        for _ in 0..5 {
            let cache_clone = Arc::clone(&cache);
            let handle = thread::spawn(move || {
                // Letture con Mutex (serializzate ma funzionanti)
                for _ in 0..10 {
                    let _ = cache_clone.get(&"key1".to_string());
                    let _ = cache_clone.size();
                }
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        println!("Test con Mutex completato con successo!");
    }
}