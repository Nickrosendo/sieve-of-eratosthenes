fn inner_sieve(n: i32) {
    let mut sieve = 2..=n;
    let n_sieve = sieve.filter(|n| !n % 2 == 0);
    for i in n_sieve {
        println!("{}", i);
    }
}

fn siever(n: i32) {
    inner_sieve(n);
}

fn main() {
    let n: i32 = 5;
    siever(n);
}
