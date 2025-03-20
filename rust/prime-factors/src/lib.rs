pub fn factors(mut num: u64) -> Vec<u64> {
    let mut fact = Vec::new();

    for n in 2.. {
        while num % n == 0 {
            fact.push(n);
            num /= n;
        }
        if num == 1 {break}
    }

    fact
}
