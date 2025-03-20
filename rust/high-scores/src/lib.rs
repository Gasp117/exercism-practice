#[derive(Debug)]
pub struct HighScores {
    val: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            val: Vec::from(scores)
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.val
    }

    pub fn latest(&self) -> Option<u32> {
        self.val.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.val.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut copy = self.val.clone();
        copy.sort();
        copy.reverse();
        copy.truncate(3);
        copy
    }
}
