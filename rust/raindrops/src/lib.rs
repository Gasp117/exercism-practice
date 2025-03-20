const NUMBERS: [u32; 3] = [3, 5, 7];

pub fn raindrops(num: u32) -> String {
    let sound = NUMBERS
        .iter()
        .filter(|n| num % *n == 0)
        .fold(String::new(), |mut sound, n| {
            match *n {
                3 => sound.push_str("Pling"),
                5 => sound.push_str("Plang"),
                7 => sound.push_str("Plong"),
                _ => (),
            }
            sound
        });

    if sound.is_empty() {
        return num.to_string();
    }

    sound
}
