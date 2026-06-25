use std::fs;

fn main() {
    let input = fs::read_to_string("data/rosalind_hamm.txt").unwrap();
    let mut lines = input.lines();

    let s = lines.next().unwrap().trim();
    let t = lines.next().unwrap().trim();

    let hamming = s
        .bytes()
        .zip(t.bytes())
        .filter(|(a, b)| a != b)
        .count();

    println!("{}", hamming);
}