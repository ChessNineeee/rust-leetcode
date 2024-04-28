pub fn sort_array_by_parity_ii(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    let mut i = 0;
    let mut j = 1;
    let len = nums.len();

    loop {
        if i >= len {
            break;
        }

        if j >= len {
            break;
        }

        let num_at_j = *nums.get(j).unwrap();
        if num_at_j % 2 == 1 {
            j += 2;
            continue;
        }

        let num_at_i = *nums.get(i).unwrap();
        if num_at_i % 2 == 0 {
            i += 2;
            continue;
        }

        nums.swap(i, j);
        i += 2;
        j += 2;
    }

    nums
}
fn main() {
    println!("{:?}", sort_array_by_parity_ii(vec![4, 2, 5, 7]));
}
