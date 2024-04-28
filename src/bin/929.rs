use std::collections::HashSet;

pub fn num_unique_emails(emails: Vec<String>) -> i32 {
    let emails = emails;
    emails
        .into_iter()
        .map(|str| {
            let sub_strs: Vec<&str> = str.split('@').collect();
            let domain = sub_strs[1];
            let local = sub_strs[0];

            let local_valid: Vec<&str> = local.split('+').collect();
            let local_valid: String = local_valid[0].chars().filter(|c| *c != '.').collect();
            local_valid + "@" + domain
        })
        .collect::<HashSet<String>>()
        .len() as i32
}
fn main() {
    println!(
        "{}",
        num_unique_emails(vec![
            "test.email+alex@leetcode.com".to_string(),
            "test.e.mail+bob.cathy@leetcode.com".to_string(),
            "testemail+david@lee.tcode.com".to_string()
        ])
    )
}
