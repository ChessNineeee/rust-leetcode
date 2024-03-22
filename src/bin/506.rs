use std::collections::BTreeSet;
pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut set = BTreeSet::new();
    for (idex, value) in score.into_iter().enumerate() {
        set.insert((value, idex));
    }

    let mut res = Vec::new();
    res.resize(set.len(), String::new());

    let medals_name = vec!["Gold Medal", "Silver Medal", "Bronze Medal"];

    let mut rank = 0;
    for (value, idx) in set.into_iter().rev() {
        if rank < medals_name.len() {
            res[idx] = medals_name[rank].to_string();
        } else {
            res[idx] = (rank + 1).to_string();
        }

        rank += 1;
    }
    res
}

fn main() {
    println!("{:?}", find_relative_ranks(vec![10, 3, 8, 9, 4]));
}
