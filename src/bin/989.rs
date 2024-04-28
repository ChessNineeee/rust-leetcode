fn get_new_k_and_op(k: i32) -> (i32, i32) {
    (k / 10, k % 10)
}

pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
    let mut num = num;
    let mut k = k;
    let mut idx = num.len() - 1;

    loop {
        if k == 0 {
            break;
        }

        if idx == usize::MAX {
            break;
        }

        let (new_k, op1) = get_new_k_and_op(k);
        let op2 = num[idx];

        let plus = (op1 + op2) / 10;
        num[idx] = (op1 + op2) % 10;

        idx = idx.checked_sub(1).or_else(|| Some(usize::MAX)).unwrap();
        k = new_k + plus;
    }

    if k == 0 {
        return num;
    }

    loop {
        if k == 0 {
            break;
        }

        let (new_k, op) = get_new_k_and_op(k);
        num.insert(0, op);

        k = new_k;
    }
    num
}
fn main() {
    println!("{:?}", add_to_array_form(vec![9, 9, 9], 999));
}
