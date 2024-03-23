struct MyHashSet {
    bitmap: [i32; 40000],
}

impl MyHashSet {
    fn new() -> Self {
        MyHashSet { bitmap: [0; 40000] }
    }

    fn loc(num: i32) -> (i32, i32) {
        (num / 32, num % 32)
    }

    fn set(&mut self, loc: (i32, i32)) {
        let (bucket_idx, bit_idx) = loc;

        let bucket = self.bitmap[bucket_idx as usize];
        let bucket = bucket | (1 << bit_idx);

        self.bitmap[bucket_idx as usize] = bucket;
    }

    fn unset(&mut self, loc: (i32, i32)) {
        let (bucket_idx, bit_idx) = loc;

        let bucket = self.bitmap[bucket_idx as usize];
        let bucket = bucket & !(1 << bit_idx);

        self.bitmap[bucket_idx as usize] = bucket;
    }

    fn get(&self, loc: (i32, i32)) -> i32 {
        let (bucket_idx, bit_idx) = loc;

        let bucket = self.bitmap[bucket_idx as usize];

        (bucket >> bit_idx) & 1
    }

    fn add(&mut self, key: i32) {
        let loc = Self::loc(key);
        self.set(loc);
    }

    fn remove(&mut self, key: i32) {
        let loc = Self::loc(key);
        self.unset(loc);
    }

    fn contains(&self, key: i32) -> bool {
        let loc = Self::loc(key);
        self.get(loc) == 1
    }
}
fn main() {}
