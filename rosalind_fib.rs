use std::fs;

fn main() {
    let input = fs::read_to_string("data/rosalind_fib.txt").unwrap();

    let mut it = input.split_whitespace();
    let n: usize = it.next().unwrap().parse().unwrap();
    let k: u64 = it.next().unwrap().parse().unwrap();

    if n <= 2 {
        println!("1");
        return;
    }

    let mut a: u64 = 1;
    let mut b: u64 = 1;

    for _ in 3..=n {
        let c = b + k * a;
        a = b;
        b = c;
    }

    println!("{}", b);
}