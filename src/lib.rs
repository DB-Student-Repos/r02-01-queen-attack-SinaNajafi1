#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        // Ensure rank and file are within the valid range (0 to 7, inclusive)
        if (0..8).contains(&rank) && (0..8).contains(&file) {
            Some(ChessPosition { rank, file })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let rank_diff = (self.position.rank - other.position.rank).abs();
        let file_diff = (self.position.file - other.position.file).abs();

        // Check if queens are on the same rank, file, or diagonal
        self.position.rank == other.position.rank ||
        self.position.file == other.position.file ||
        rank_diff == file_diff
    }
}

