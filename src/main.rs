//const LIMIT: usize = 1_000_000_000;
const LIMIT: usize = 1_000;

fn main() {
    // https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Overview
    let mut out = vec![true; LIMIT + 2]; // [0] is n = 0, so the first two elements are junk because p0 = 2
    eprintln!("Allocated out");

    let mut p = 2; // current prime number
    loop {
        // can start at p^2 because anything lower will already be marked
        for mult in out.iter_mut().skip(p * p).step_by(p) {
            *mult = false;
        }
        let idx = out
            .iter()
            .enumerate()
            .skip(p + 1)
            .find(|(_, &b)| b)
            .map(|(n, _)| n); // have to add 1 to match what's in skip() because position() is relative
        match idx {
            Some(next) => p += next,
            None => break,
        };
    }

    let primes = out
        .iter()
        .enumerate()
        .filter_map(|(idx, is_prime)| is_prime.then(|| idx + 2));
    for p in primes.take(20) {
        println!("{}", p);
    }
}
