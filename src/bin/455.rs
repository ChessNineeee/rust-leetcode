pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
    let mut g = g;
    let mut s = s;
    g.sort();
    s.sort();

    let mut count = 0;
    let mut i = 0;
    let mut j = 0;

    while i < g.len() && j < s.len() {
        if g[i] <= s[j] {
            count += 1;
            i += 1;
            j += 1;
            continue;
        }

        j += 1;
    }

    return count;
}
fn main() {
    println!("{}", find_content_children(vec![1, 2, 3], vec![1,  3]));
}
