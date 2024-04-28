use std::collections::{HashMap, HashSet};

fn judge_when_vote_empty(n: i32) -> i32 {
    if n == 1 {
        return n;
    }
    return -1;
}

pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    if trust.is_empty() {
        return judge_when_vote_empty(n);
    }

    let judge_vote = n - 1;
    let mut vote_map = HashMap::new();
    let mut voters = HashSet::new();

    for vote in trust.into_iter() {
        let voter = vote[0];
        let votee = vote[1];
        voters.insert(voter);
        vote_map
            .entry(votee)
            .and_modify(|val| {
                *val += 1;
            })
            .or_insert(1);
    }

    match vote_map
        .into_iter()
        .find(|(votee, val)| *val == judge_vote && !voters.contains(votee))
    {
        Some((key, _)) => key,
        None => -1,
    }
}

fn main() {
    println!(
        "{}",
        find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]])
    );
}
