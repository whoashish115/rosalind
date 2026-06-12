use std::fs;

fn main() {
    let s = fs::read_to_string("data/rosalind_revc.txt").unwrap();
    let mut ans = String::new();
    for c in s.trim().chars().rev() {
        if c == 'A' {
            ans.push('T');
        } else if c == 'T' {
            ans.push('A');
        } else if c == 'C' {
            ans.push('G');
        } else {
            ans.push('C');
        }
    }

    println!("{}", ans);
}