use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum TicTacToeError {
    #[error("TileOutOfBounds")]
    TileOutOfBounds = 1000,

    #[error("TileAlreadySet")]
    TileAlreadySet,

    #[error("GameAlreadyOver")]
    GameAlreadyOver,

    #[error("NotPlayersTurn")]
    NotPlayersTurn,

    #[error("GameAlreadyStarted")]
    GameAlreadyStarted,
}
