use std::fs;

fn main() {
    let input = fs::read_to_string("data/rosalind_subs.txt").unwrap();
    let mut lines = input.lines();

    let s = lines.next().unwrap().trim();
    let t = lines.next().unwrap().trim();

    let mut ans = Vec::new();

    for i in 0..=s.len() - t.len() {
        if &s[i..i + t.len()] == t {
            ans.push((i + 1).to_string()); 
        }
    }

    println!("{}", ans.join(" "));
}