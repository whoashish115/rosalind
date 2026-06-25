use std::{collections::HashMap, fs};

fn main() {
    let rna = fs::read_to_string("data/rosalind_prot.txt").unwrap();
    let rna = rna.trim();

    let codons = [
        ("UUU","F"),("CUU","L"),("AUU","I"),("GUU","V"),
        ("UUC","F"),("CUC","L"),("AUC","I"),("GUC","V"),
        ("UUA","L"),("CUA","L"),("AUA","I"),("GUA","V"),
        ("UUG","L"),("CUG","L"),("AUG","M"),("GUG","V"),
        ("UCU","S"),("CCU","P"),("ACU","T"),("GCU","A"),
        ("UCC","S"),("CCC","P"),("ACC","T"),("GCC","A"),
        ("UCA","S"),("CCA","P"),("ACA","T"),("GCA","A"),
        ("UCG","S"),("CCG","P"),("ACG","T"),("GCG","A"),
        ("UAU","Y"),("CAU","H"),("AAU","N"),("GAU","D"),
        ("UAC","Y"),("CAC","H"),("AAC","N"),("GAC","D"),
        ("UAA","Stop"),("CAA","Q"),("AAA","K"),("GAA","E"),
        ("UAG","Stop"),("CAG","Q"),("AAG","K"),("GAG","E"),
        ("UGU","C"),("CGU","R"),("AGU","S"),("GGU","G"),
        ("UGC","C"),("CGC","R"),("AGC","S"),("GGC","G"),
        ("UGA","Stop"),("CGA","R"),("AGA","R"),("GGA","G"),
        ("UGG","W"),("CGG","R"),("AGG","R"),("GGG","G"),
    ];

    let table: HashMap<&str, &str> = codons.iter().copied().collect();

    let mut protein = String::new();

    for chunk in rna.as_bytes().chunks(3) {
        if chunk.len() < 3 {
            break;
        }

        let codon = std::str::from_utf8(chunk).unwrap();
        let aa = table[codon];

        if aa == "Stop" {
            break;
        }

        protein.push_str(aa);
    }

    println!("{}", protein);
}