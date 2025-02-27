use bytemuck::{Pod, Zeroable};
use sdk::error::TicTacToeError;
use solana_program::pubkey::Pubkey;

pub struct Game {
    players: [Pubkey; 2],

    turn: u8,

    /// 0: X, 1: O,
    board: [[Option<u8>; 3]; 3],

    /// 0: Active, 1: Tie, 2: Won { winner: Pubkey },
    state: u8,
}

impl Game {
    pub const MAXIMUM_SIZE: usize = (32 * 2) + 1 + (9 * (1 + 1)) + (32 + 1);

    pub fn start(&mut self, players: [Pubkey; 2]) -> Result<(), TicTacToeError> {
        if self.turn != 0 {
            return Err(TicTacToeError::GameAlreadyStarted);
        }

        self.players = players;
        self.turn = 1;

        Ok(())
    }

    pub fn is_active(&self) -> bool {
        // self.state == GameState::Active
        self.state == 0
    }

    fn current_player_index(&self) -> usize {
        ((self.turn - 1) % 2) as usize
    }

    pub fn current_player(&self) -> Pubkey {
        self.players[self.current_player_index()]
    }

    pub fn play(&mut self, tile: &Tile) -> Result<(), TicTacToeError> {
        if self.is_active() {
            return Err(TicTacToeError::GameAlreadyOver);
        }

        match tile {
            tile @ Tile {
                row: 0..=2,
                column: 0..=2,
            } => match self.board[tile.row as usize][tile.column as usize] {
                Some(_) => return Err(TicTacToeError::TileAlreadySet),
                None => {
                    self.board[tile.row as usize][tile.column as usize] =
                        Some(self.current_player_index() as u8);
                }
            },
            _ => return Err(TicTacToeError::TileOutOfBounds),
        }

        self.update_state();

        if self.state == 0 {
            self.turn += 1;
        }

        Ok(())
    }

    fn is_winning_trio(&self, trio: [(usize, usize); 3]) -> bool {
        let [first, second, third] = trio;
        self.board[first.0][first.1].is_some()
            && self.board[first.0][first.1] == self.board[second.0][second.1]
            && self.board[first.0][first.1] == self.board[third.0][third.1]
    }

    fn update_state(&mut self) {
        for i in 0..=2 {
            // three of the same in one row
            if self.is_winning_trio([(i, 0), (i, 1), (i, 2)]) {
                // self.state = GameState::Won {
                //     winner: self.current_player(),
                // };
                self.state = 2;
                return;
            }
            // three of the same in one column
            if self.is_winning_trio([(0, i), (1, i), (2, i)]) {
                // self.state = GameState::Won {
                //     winner: self.current_player(),
                // };
                self.state = 2;
                return;
            }
        }

        // three of the same in one diagonal
        if self.is_winning_trio([(0, 0), (1, 1), (2, 2)])
            || self.is_winning_trio([(0, 2), (1, 1), (2, 0)])
        {
            // self.state = GameState::Won {
            //     winner: self.current_player(),
            // };
            self.state = 2;
            return;
        }

        // reaching this code means the game has not been won,
        // so if there are unfilled tiles left, it's still active
        for row in 0..=2 {
            for column in 0..=2 {
                if self.board[row][column].is_none() {
                    return;
                }
            }
        }

        // game has not been won
        // game has no more free tiles
        // -> game ends in a tie
        // self.state = GameState::Tie;
        self.state = 1;
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, Pod, Zeroable)]
// #[repr(C)]
// pub enum GameState {
//     Active,
//     Tie,
//     Won { winner: Pubkey },
// }

// #[derive(
//     AnchorSerialize, AnchorDeserialize, FromPrimitive, ToPrimitive, Copy, Clone, PartialEq, Eq,
// )]
// pub enum Sign {
//     X,
//     O,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Pod, Zeroable)]
#[repr(C)]
pub struct Tile {
    row: u8,
    column: u8,
}
