use std::fs;

fn main() {
    let s = fs::read_to_string("./data/rosalind_dna.txt").unwrap();
    let a = s.matches('A').count();
    let c = s.matches('C').count();
    let g = s.matches('G').count();
    let t = s.matches('T').count();
    println!("{} {} {} {}", a, c, g, t);
}