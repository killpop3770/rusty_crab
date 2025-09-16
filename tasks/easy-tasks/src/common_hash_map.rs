use std::hash::{DefaultHasher, Hash, Hasher};

use crate::common_vec::CommonVec;

const CAPACITY_INCREASED_FACTOR: usize = 2;

pub struct CommonHashMap<K, V> {
    buckets: CommonVec<Option<Box<Entry<K, V>>>>,
    size: usize,
    capacity: usize,
    load_factor: f32,
}

#[derive(Clone)]
pub struct Entry<K, V> {
    key: K,
    value: V,
    next_entry: Option<Box<Entry<K, V>>>,
}

impl<K, V> CommonHashMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> CommonHashMap<K, V> {
        // TODO: instead resize_with
        let capacity = 16;
        let mut array = CommonVec::new();

        for _ in 0..capacity {
            array.push(None);
        }
        return CommonHashMap {
            buckets: array,
            size: 0,
            capacity: capacity,
            load_factor: 0.75,
        };
    }

    pub fn insert_or_update(&mut self, key: K, value: V) -> Option<V> {
        if self.is_resize() {
            self.resize();
        }

        // try find entry to update
        let index = self.get_index(&key);
        if let Some(mut current_bucket) = self.buckets.get_mut(index) {
            while let Some(entry) = current_bucket {
                if entry.key == key {
                    let old_value = std::mem::replace(&mut entry.value, value);
                    return Some(old_value);
                }
                current_bucket = &mut entry.next_entry;
            }
        }

        // create new entry
        let mut next_entry_ref = None;
        if let Some(entry) = self.buckets.get_mut(index) {
            next_entry_ref = entry.take();
        };
        let new_entry = Entry {
            key,
            value,
            next_entry: next_entry_ref,
        };
        self.buckets.set(index, Some(Box::new(new_entry)));
        self.increase_size();

        return None;
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let index = self.get_index(key);
        if let Some(mut current) = self.buckets.get_mut(index) {
            while let Some(entry) = current {
                if entry.key == *key {
                    return Some(&entry.value);
                }
                current = &mut entry.next_entry;
            }
        }

        None
    }

    pub fn increase_size(&mut self) {
        self.size += 1;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn get_index(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        return (hasher.finish() as usize) % self.capacity;
    }

    fn is_resize(&self) -> bool {
        return self.size >= (self.load_factor * self.capacity as f32) as usize;
    }

    fn resize(&mut self) {
        let new_capacity = self.capacity * CAPACITY_INCREASED_FACTOR;
        let mut new_buckets = CommonVec::with_capacity(new_capacity);
        // TODO: instead resize_with method
        for _ in 0..new_capacity {
            new_buckets.push(None);
        }

        let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);
        self.capacity = new_capacity;

        for mut bucket in old_buckets {
            while let Some(mut entry) = bucket {
                let next_entry = entry.next_entry.take();

                let new_index = self.get_index(&entry.key) % new_capacity;

                if let Some(target_bucket) = self.buckets.get_mut(new_index) {
                    entry.next_entry = target_bucket.take();
                    *target_bucket = Some(entry);
                }

                bucket = next_entry;
            }
        }
    }
}

#[cfg(test)]
mod test_common_vec {

    use crate::common_hash_map::CommonHashMap;

    #[test]
    fn test_new() {
        let map: CommonHashMap<String, i32> = CommonHashMap::new();
        assert_eq!(map.size(), 0);
    }

    #[test]
    fn test_insert() {
        let mut map: CommonHashMap<String, String> = CommonHashMap::new();
        map.insert_or_update(String::from("keka"), String::from("Chebureka"));
        assert_eq!(map.size(), 1);
    }

    #[test]
    fn test_get() {
        let key = String::from("keka");
        let target_value = String::from("Chebureka");
        let mut map: CommonHashMap<String, String> = CommonHashMap::new();
        map.insert_or_update(key.clone(), target_value.clone());

        let result = map.get(&key).unwrap().to_owned();
        assert_eq!(result, target_value);
    }

    #[test]
    fn test_update() {
        let key = String::from("keka");
        let current_value = String::from("Chebureka");
        let target_value = String::from("Chebureka");
        let mut map: CommonHashMap<String, String> = CommonHashMap::new();
        map.insert_or_update(key.clone(), current_value);
        map.insert_or_update(key.clone(), target_value.clone());

        let result = map.get(&key).unwrap().to_owned();
        assert_eq!(result, target_value);
    }
}
