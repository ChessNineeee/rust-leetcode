use std::collections::HashMap;

pub fn lemonade_change(bills: Vec<i32>) -> bool {
    let mut change_5 = 0;
    let mut change_10 = 0;

    for bill in bills.into_iter() {
        match bill {
            5 => {
                change_5 += 1;
            }
            10 => {
                if change_5 == 0 {
                    return false;
                }
                change_5 -= 1;
                change_10 += 1;
            }
            20 => {
                if change_10 >= 1 && change_5 >= 1 {
                    change_5 -= 1;
                    change_10 -= 1;
                    continue;
                }

                if change_5 >= 3 {
                    change_5 -= 3;
                    continue;
                }

                return false;
            }
            _ => unreachable!("never get there"),
        };
    }
    true
}
fn main() {
    println!("{}", lemonade_change(vec![5, 5, 10, 10, 20]));
}
