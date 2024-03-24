pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
    let mut left = 0;
    let mut right = (letters.len() - 1) as i32;

    loop {
        if right < left {
            break;
        }

        let mid = left + (right - left) / 2;

        if letters[mid as usize] <= target {
            left = mid + 1;
            continue;
        }

        right = mid - 1;
        continue;
    }

    if left < letters.len() as i32 {
        letters[left as usize]
    } else {
        letters[0]
    }
}
fn main() {
    println!("{}", next_greatest_letter(vec!['x', 'x', 'y', 'y'], 'z'))
}
