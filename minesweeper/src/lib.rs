// Minesweeper constants
const MINE: u8 = b'*';
const MINE_CHAR: char = '*';
const EMPTY_CHAR: char = ' ';

// Fields that should be updated when a mine is found
static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];


pub fn annotate(field: &[&str]) -> Vec<String> {
    // Get map height
    let height = field.len() as i32;

    // Run on all rows
    (0..height)
        .map(|y| {
            // Get map width
            let width = field[y as usize].len() as i32;
            // Run on all columns
            (0..width)
                .map(|x| filter_width(field, x, y, width, height))
                .collect()
        })
        .collect()
}

fn filter_width(field: &[&str], x: i32, y: i32, width: i32, height: i32) -> char {
    // If element requested is a mine, return it
    if field[y as usize].as_bytes()[x as usize] == MINE {
        MINE_CHAR
    } else {
        // Update position based on surrounding mines
        match NEIGBOURHOOD_OFFSETS
            .iter()
            .map(|&(ox, oy)| (x + ox, y + oy))
            .filter(|&(x, y)| (0 <= x && x < width) && (0 <= y && y < height))
            .filter(|&(x, y)| field[y as usize].as_bytes()[x as usize] == b'*')
            .count()
        {
            0 => EMPTY_CHAR,
            n => (n as u8 + '0' as u8) as char,
        }
    }
}
