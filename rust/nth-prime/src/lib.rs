pub fn nth(n: u32) -> u32 {
    // This may be 'cheating', but is no sense creating specific code for first value, when is well known.
    let mut primes: Vec<u32> = vec![2];

    // Start primes count
    let mut count = 1;
    while (primes.len() as u32) <= n {
        // Only odd numbers can be prime
        count += 2;

        // As soon as the current number is divisible by a prime number, then it is not
        if primes.iter().any(|n| count % n == 0) {
            continue;
        }

        // No division found, store with rest of primes.
        primes.push(count);
    }

    primes[n as usize]
}
