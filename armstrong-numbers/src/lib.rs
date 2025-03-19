pub fn is_armstrong_number(num: u32) -> bool {
    let mut temp = num;
    let n = digits(num);
    (0..n)
        .map(|_| {
            let digit = temp % 10;
            temp /= 10;
            digit.pow(n)
        })
        .sum::<u32>()
        == num
}

// Get number of digits
fn digits(x: u32) -> u32 {
    (x as f32).log10().floor() as u32 + 1
}
