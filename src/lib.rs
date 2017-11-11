#[derive(Copy,Clone, PartialEq, Debug)]
pub enum Player {
  Cross,
  Circle
}

type GameBoard = Vec<Vec<Option<Player>>>;

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

fn opposite_player(player: &Player) -> Player {
  match *player {
    Player::Cross => Player::Circle,
    Player::Circle => Player::Cross
  }
}

fn has_no_cells_left(board: &GameBoard) -> bool {
  return board.iter().fold(
    true, |acc, rows| acc && rows.iter().fold(
        true, |acc, &cell| acc && cell.is_none())
    );
}

fn count_same(same_count: &mut usize, cell: Option<Player>, player: Option<Player>) -> Option<usize> {
  if cell == player {
    *same_count = *same_count + 1;
  } else {
    *same_count = 0;
  }

  Some(*same_count)
}

fn check_columns(board: &GameBoard, player: Option<Player>, required_row_length: usize) -> bool {
  (0..board[0].len()).fold(false, |won, x| {
    won ||
    (0..board.len())
    .map(|y| board[y][x])
    .scan(0, |current_count, cell| count_same(current_count, cell, player))
    .skip_while(|count| *count < required_row_length)
    .next()
    .is_some()
  })
}

fn check_rows(board: &GameBoard, player: Option<Player>, required_row_length: usize) -> bool {
  (0..board.len()).fold(false, |won, y| {
    won ||
    (0..board[0].len())
    .map(|x| board[y][x])
    .scan(0, |current_count, cell| count_same(current_count, cell, player))
    .skip_while(|count| *count < required_row_length)
    .next()
    .is_some()
  })
}

fn check_diagonal(board: &GameBoard, player: Option<Player>, required_row_length: usize) -> bool {
  (0..board[0].len()).fold(false, |won, x| {
    won ||
    (0..board.len())
    .map(|y| board[y][(y + x) % board[0].len()]) //This hack will not work if board is fixed size
    .scan(0, |current_count, cell| count_same(current_count, cell, player))
    .skip_while(|count| *count < required_row_length)
    .next()
    .is_some()
  })
  ||
  (2..board[0].len()).fold(false, |won, x| {
    won ||
    (0..x + 1)
    .map(|y| board[y][(x - y)])
    .scan(0, |current_count, cell| count_same(current_count, cell, player))
    .skip_while(|count| *count < required_row_length)
    .next()
    .is_some()
  })
  ||
  (2..board[0].len()).fold(false, |won, x| {
    won ||
    (0..x + 1)
    .map(|y| board[board.len() - y - 1][board[0].len() - (x - y)- 1])
    .scan(0, |current_count, cell| count_same(current_count, cell, player))
    .skip_while(|count| *count < required_row_length)
    .next()
    .is_some()
  })
}

fn check_win_condition(board: &GameBoard, player: Player, required_row_length: usize) -> bool {
  let player = Some(player);

  check_columns(&board, player, required_row_length) ||
    check_rows(&board, player, required_row_length) ||
    check_diagonal(&board, player, required_row_length)
}

fn get_winner(board: &GameBoard, player: Player, required_row_length: usize) -> Option<Winner> {
  if has_no_cells_left(&board) {
    Some(Winner::Draw)
  } else if check_win_condition(&board, player, required_row_length) {
    match player {
      Player::Circle => Some(Winner::Circle),
      Player::Cross => Some(Winner::Cross)
    }
  } else {
    None
  }
}

fn error(game_state: GameState, error: LuffarError) -> GameState {
  return GameState {
    error: Some(error),
    ..game_state
  }
}

pub fn do_turn(game_state: GameState, y_pos: usize, x_pos: usize) -> GameState {
  if game_state.winner.is_some() {
    return error(game_state, LuffarError::GameOver);
  }

  if y_pos >= game_state.board.len() || x_pos >= game_state.board.len() {
    return error(game_state, LuffarError::InvalidPosition);
  }

  if game_state.board[y_pos][x_pos].is_some() {
    return error(game_state, LuffarError::InvalidPosition);
  }
  
  let board = update_board(&game_state, y_pos, x_pos);
  let winner = get_winner(&board, game_state.next_player, game_state.required_row_length);

  GameState {
    board: board,
    next_player: opposite_player(&game_state.next_player),
    winner: winner,
    required_row_length: game_state.required_row_length,
    error: None
  }
}

fn update_board(game_state: &GameState, y_pos: usize, x_pos: usize) -> GameBoard {
  let mut board = game_state.board.clone();
  
  board[y_pos][x_pos] = Some(game_state.next_player);

  board
}

fn create_initial_board(size: usize) -> GameBoard {
  vec![vec![None; size]; size]
}

pub fn start(board_size: usize, required_row_length: usize) -> GameState {
  GameState {
    board: create_initial_board(board_size),
    next_player: Player::Cross,
    winner: None,
    required_row_length: required_row_length,
    error: None
  }
}

#[cfg(test)]
mod tests {
  use start;
  use Player;
  use Winner;
  use do_turn;
  
  #[test]
  fn inital_board_is_five() {
    let game_state = start(5, 3);
    assert_eq!(game_state.board.len(), 5);
  }
  
  #[test]
  fn cross_starts() {
    let game_state = start(5, 3);
    assert_eq!(game_state.next_player, Player::Cross);
  }

  #[test]
  fn do_turn_changes_player() {
    let game_state = start(5, 3);
    let game_state = do_turn(game_state, 0, 0);
    assert_eq!(game_state.next_player, Player::Circle);
  }

  #[test]
  fn no_winner() {
    let game_state = start(5, 3);
    assert_eq!(game_state.winner, None);
  }

  #[test]
  fn row_win() {
    let game_state = start(5, 3);
    let game_state = do_turn(game_state, 0, 0);
    let game_state = do_turn(game_state, 1, 0);
    let game_state = do_turn(game_state, 0, 1);
    let game_state = do_turn(game_state, 1, 1);
    let game_state = do_turn(game_state, 0, 2);
    assert_eq!(game_state.winner.unwrap(), Winner::Cross);
  }

  #[test]
  fn column_win() {
    let game_state = start(5, 3);
    let game_state = do_turn(game_state, 0, 0);
    let game_state = do_turn(game_state, 0, 1);
    let game_state = do_turn(game_state, 1, 0);
    let game_state = do_turn(game_state, 1, 1);
    let game_state = do_turn(game_state, 2, 0);
    assert_eq!(game_state.winner.unwrap(), Winner::Cross);
  }

  #[test]
  fn diagonal_win() {
    let game_state = start(5, 3);
    let game_state = do_turn(game_state, 1, 0);
    let game_state = do_turn(game_state, 1, 1);
    let game_state = do_turn(game_state, 2, 1);
    let game_state = do_turn(game_state, 0, 2);
    let game_state = do_turn(game_state, 3, 2);
    assert_eq!(game_state.winner.unwrap(), Winner::Cross);
  }

  #[test]
  fn diagonal_win2() {
    let game_state = start(5, 3);
    let game_state = do_turn(game_state, 4, 1);
    let game_state = do_turn(game_state, 1, 0);
    let game_state = do_turn(game_state, 3, 2);
    let game_state = do_turn(game_state, 1, 3);
    let game_state = do_turn(game_state, 2, 3);
    assert_eq!(game_state.winner.unwrap(), Winner::Cross);
  }
}
