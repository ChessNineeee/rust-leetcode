use std::collections::HashSet;

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut count = 0;
    let mut flowerbed = flowerbed;
    let mut total = 0;

    flowerbed.push(0);
    flowerbed.insert(0, 0);

    for it in flowerbed.iter() {
        if *it == 0 {
            count += 1;
            continue;
        }

        total += (count - 1) / 2;
        count = 0;
    }

    total += (count - 1) / 2;
    return total >= n;
}
fn main() {
    println!("{}", can_place_flowers(vec![1, 0, 0, 0], 1));
}
