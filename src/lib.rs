mod win_check;
mod game_state;

pub use game_state::{GameState, Player, GameBoard, Winner, LuffarError};

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

fn get_winner(board: &GameBoard, player: Player, required_row_length: usize) -> Option<Winner> {
  if has_no_cells_left(&board) {
    Some(Winner::Draw)
  } else if win_check::check_win_condition(&board, player, required_row_length) {
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

  if y_pos >= game_state.board.len() || x_pos >= game_state.board[0].len() {
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

fn expand_board_if_needed(mut board: GameBoard, y_pos:usize, x_pos: usize) -> GameBoard {
  if y_pos == 0 {
    let ln = board[0].len();
    board.insert(0, vec![None; ln]);
  } else if y_pos == board.len() - 1 {
    let ln = board[0].len();
    board.push(vec![None; ln]);
  }

  if x_pos == 0 {
    for row in &mut board {
      row.insert(0, None);
    }
  } else if x_pos == board[0].len() - 1 {
    for row in &mut board {
      row.push(None);
    }
  }
  board
}

fn update_board(game_state: &GameState, y_pos: usize, x_pos: usize) -> GameBoard {
  let mut board = game_state.board.clone();
  
  board[y_pos][x_pos] = Some(game_state.next_player);

  expand_board_if_needed(board, y_pos, x_pos)
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
  use do_turn;
  use expand_board_if_needed;
  
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
  fn board_is_extended_top() {
    let board = vec![
      vec![None, Some(Player::Circle), None],
      vec![None, None, None],
      vec![None, None, None],
    ];
    let board = expand_board_if_needed(board, 0, 1);
    assert_eq!(board[0][1], None);
    assert_eq!(board.len(), 4);
    assert_eq!(board[0].len(), 3);
  }

  #[test]
  fn board_is_extended_bottom() {
    let board = vec![
      vec![None, None, None],
      vec![None, None, None],
      vec![None, Some(Player::Circle), None],
    ];
    let board = expand_board_if_needed(board, 2, 1);
    assert_eq!(board[2][1], Some(Player::Circle));
    assert_eq!(board.len(), 4);
    assert_eq!(board[0].len(), 3);
  }

  #[test]
  fn board_is_extended_left() {
    let board = vec![
      vec![None, None, None],
      vec![Some(Player::Circle), None, None],
      vec![None, None, None],
    ];
    let board = expand_board_if_needed(board, 1, 0);
    assert_eq!(board[1][0], None);
    assert_eq!(board[0].len(), 4);
    assert_eq!(board.len(), 3);
  }

  #[test]
  fn board_is_extended_right() {
    let board = vec![
      vec![None, None, None],
      vec![None, None, Some(Player::Circle)],
      vec![None, None, None],
    ];
    let board = expand_board_if_needed(board, 1, 2);
    assert_eq!(board[1][2], Some(Player::Circle));
    assert_eq!(board[0].len(), 4);
    assert_eq!(board.len(), 3);
  }
}
