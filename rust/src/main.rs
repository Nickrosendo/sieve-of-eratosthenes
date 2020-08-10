fn inner_sieve(p: i32, sieve: impl Iterator<Item = i32>) -> impl Iterator<Item = i32> {
    sieve.filter(move |n| (n % p != 0 || *n == p))
}

fn siever(n: i32) {
    let p = 2;
    let sieve = p..=n;
    let mut inner_sieved = inner_sieve(p, sieve);
    for j in inner_sieve(3, &mut inner_sieved) {
        println!("{  }", j);
    }
}

fn main() {
    let n: i32 = 12;
    siever(n);
}
