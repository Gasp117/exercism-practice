pub fn build_proverb(list: &[&str]) -> String {
    let mut song = String::new();
    for (pos, piece) in list.iter().enumerate() {
        match pos {
            pos if pos == list.len() - 1 => song.push_str(&last_verse(list[0])),
            _ => song.push_str(&basic_verse(piece, list[pos + 1])),
        }
    }
    song
}

fn basic_verse(cur_piece: &str, old_piece: &str) -> String {
    format!("For want of a {cur_piece} the {old_piece} was lost.\n")
}

fn last_verse(piece: &str) -> String {
    format!("And all for the want of a {piece}.")
}
