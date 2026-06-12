use std::fs;

fn main() {
    let s = fs::read_to_string("./data/rosalind_rna.txt").unwrap();

    println!("{}", s.trim().replace('T', "U"));
}