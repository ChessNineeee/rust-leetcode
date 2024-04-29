fn sub_var(var: &mut usize) {
    if let Some(v) = var.checked_sub(1) {
        *var = v;
    } else {
        *var = usize::MAX;
    }
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut tail = (m + n - 1) as usize;
    let mut i = (m - 1) as usize;
    let mut j = (n - 1) as usize;
    loop {
        if i == usize::MAX && j == usize::MAX {
            break;
        }

        if i == usize::MAX {
            nums1[tail] = nums2[j];
            sub_var(&mut tail);
            sub_var(&mut j);
            continue;
        }

        if j == usize::MAX {
            nums1[tail] = nums1[i];
            sub_var(&mut tail);
            sub_var(&mut i);
            continue;
        }

        if nums1[i] > nums2[j] {
            nums1[tail] = nums1[i];
            sub_var(&mut tail);
            sub_var(&mut i);
            continue;
        }

        nums1[tail] = nums2[j];
        sub_var(&mut tail);
        sub_var(&mut j);
    }

    return;
}
fn main() {
    let mut vec1 = vec![1, 2, 3, 4, 5, 6, 7, 0, 0, 0];
    let mut vec2 = vec![1, 2, 3];
    merge(&mut vec1, 7, &mut vec2, 3);
    println!("{:?}", vec1);
}
