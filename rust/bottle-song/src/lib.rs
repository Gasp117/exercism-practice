const NUMBERS: [&str; 11] = [
    "no", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
];

pub fn recite(start: u32, remove: u32) -> String {
    ((start + 1 - remove)..(start + 1))
        .rev()
        .map(|n| {
            let cur_num = NUMBERS[n as usize];
            let rem_num = &NUMBERS[(n - 1) as usize].to_lowercase();
            let cur_bottle = bottles(n);
            let rem_bottle = bottles(n - 1);
            create_verse(cur_num, rem_num, cur_bottle, rem_bottle)
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn create_verse<'a>(cur_num: &str, rem_num: &str, cur_bottle: &str, rem_bottle: &str) -> String {
    format!(
        "{cur_num} green {cur_bottle} hanging on the wall,\n\
        {cur_num} green {cur_bottle} hanging on the wall,\n\
        And if one green bottle should accidentally fall,\n\
        There'll be {rem_num} green {rem_bottle} hanging on the wall.",
    )
}

fn bottles<'a>(n: u32) -> &'a str {
    match n {
        1 => "bottle",
        _ => "bottles",
    }
}
