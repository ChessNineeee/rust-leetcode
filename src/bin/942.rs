pub fn di_string_match(s: String) -> Vec<i32> {
    let mut res = Vec::new();
    let mut low = 0;
    let mut high = s.len() as i32;

    for char in s.chars() {
        match char {
            'I' => {
                res.push(low);
                low += 1;
            }
            'D' => {
                res.push(high);
                high -= 1;
            }
            _ => unreachable!("never get there"),
        }
    }

    assert_eq!(low, high);

    res.push(low);
    res
}
fn main() {
    println!("{:?}", di_string_match("IDID".to_string()));
}
