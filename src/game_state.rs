#[derive(Copy,Clone, PartialEq, Debug)]
pub enum Player {
  Cross,
  Circle
}

pub type GameBoard = Vec<Vec<Option<Player>>>;

#[derive(Copy,Clone, PartialEq, Debug)]
pub enum Winner {
  Cross,
  Circle,
  Draw
}

#[derive(Debug, Clone)]
pub struct GameState {
  pub board: GameBoard,
  pub next_player: Player,
  pub winner: Option<Winner>,
  pub required_row_length: usize,
  pub error: Option<LuffarError>
}

#[derive(Debug, Clone)]
pub enum LuffarError {
  InvalidPosition,
  GameOver
}
