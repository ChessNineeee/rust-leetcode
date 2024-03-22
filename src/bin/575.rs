use std::collections::HashSet;

pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
    let max_eat = candy_type.len() / 2;
    let set: HashSet<i32> = candy_type.into_iter().collect();
    max_eat.min(set.len()) as i32
}
fn main() {
    println!("{}", distribute_candies(vec![1,1,2,3]))
}
