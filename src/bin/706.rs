use std::collections::LinkedList;

struct MyHashMap {
    nodes: Vec<LinkedList<(i32, i32)>>,
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap {
            nodes: vec![LinkedList::new(); 10009],
        }
    }

    fn loc(key: i32) -> usize {
        (key % 10009) as usize
    }

    fn put(&mut self, key: i32, value: i32) {
        let bucket_idx = Self::loc(key);
        let bucket = self.nodes.get_mut(bucket_idx).unwrap();

        match bucket.iter_mut().find(|(k, _)| *k == key) {
            Some((_, v)) => *v = value,
            None => bucket.push_back((key, value)),
        }
    }

    fn get(&self, key: i32) -> i32 {
        let bucket_idx = Self::loc(key);
        let bucket = self.nodes.get(bucket_idx).unwrap();

        match bucket.iter().find(|(k, _)| *k == key) {
            Some((_, v)) => *v,
            None => -1,
        }
    }

    fn remove(&mut self, key: i32) {
        let bucket_idx = Self::loc(key);
        let bucket = self.nodes.get_mut(bucket_idx).unwrap().clone();
        let bucket: LinkedList<(i32, i32)> =
            bucket.into_iter().filter(|(k, _)| *k != key).collect();
        self.nodes[bucket_idx] = bucket;
    }
}

fn main() {}
