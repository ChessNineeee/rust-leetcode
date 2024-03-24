use std::collections::HashSet;

pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
    let A: i32 = alice_sizes.iter().sum();
    let B: i32 = bob_sizes.iter().sum();

    let bob_set: HashSet<i32> = bob_sizes.into_iter().collect();

    // A - x + y = B - y + x
    for x in alice_sizes.into_iter() {
        let y = (B - A + 2 * x) / 2;
        if bob_set.contains(&y) {
            return vec![x, y];
        }
    }

    unreachable!("never")
}
fn main() {
    println!("{:?}", fair_candy_swap(vec![2], vec![1, 3]));
}
