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

pub struct GameState {
  board: GameBoard,
  next_player: Player,
  winner: Option<Winner>
}

#[derive(Debug)]
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
  iterate_board(board, player, required_row_length, |x, y, board|board[y][x])
}

fn check_rows(board: &GameBoard, player: Option<Player>, required_row_length: usize) -> bool {
  iterate_board(board, player, required_row_length, |x, y, board|board[x][y])
}

fn check_diagonal(board: &GameBoard, player: Option<Player>, required_row_length: usize) -> bool {
  iterate_board(board, player, required_row_length, |x, y, board|board[x][(x + y) % board.len()]) ||
  iterate_board(board, player, required_row_length, |x, y, board| {
    let val = y as i16 - x as i16;
    let length = board.len() as i16;
    let val = ((val % length) + length) % length;
    board[val as usize][x]
  })
}

fn iterate_board(board: &GameBoard, player: Option<Player>, required_row_length: usize, cell_getter: fn(usize, usize, &GameBoard) -> Option<Player>) -> bool {
  (0..board.len()).fold(false, |prev_state, y| {
    prev_state ||
    (0..board.len())
    .map(|count| cell_getter(count, y, board))
    .scan(0, |current_count, cell| count_same(current_count, cell, player))
    .skip_while(|count| *count < required_row_length)
    .next()
    .is_some()
  })
}

fn check_win_condition(board: &GameBoard, player: Player) -> bool {
  let required_row_length = 3; //TODO: add to GameState
  let player = Some(player);

  check_columns(&board, player, required_row_length) ||
    check_rows(&board, player, required_row_length) ||
    check_diagonal(&board, player, required_row_length)
}

fn get_winner(board: &GameBoard, player: Player) -> Option<Winner> {
  if has_no_cells_left(&board) {
    Some(Winner::Draw)
  } else if check_win_condition(&board, player) {
    match player {
      Player::Circle => Some(Winner::Circle),
      Player::Cross => Some(Winner::Cross)
    }
  } else {
    None
  }
}

pub fn make_turn(game_state: GameState, y_pos: usize, x_pos: usize) -> Result<GameState, LuffarError> {
  if game_state.winner.is_some() {
    return Err(LuffarError::GameOver);
  }

  //TODO: check input
  //TODO: make board bigger if needed
  
  let board = update_board(&game_state, y_pos, x_pos);
  let winner = get_winner(&board, game_state.next_player);

  Ok(GameState {
    board: board,
    next_player: opposite_player(&game_state.next_player),
    winner: winner
  })
}

fn update_board(game_state: &GameState, y_pos: usize, x_pos: usize) -> GameBoard {
  let mut board = game_state.board.clone();
  board[y_pos][x_pos] = Some(game_state.next_player);
  board
}

fn create_initial_board(size: usize) -> GameBoard {
  vec![vec![None; size]; size]
}

pub fn start() -> GameState {
  GameState {
    board: create_initial_board(5),
    next_player: Player::Cross,
    winner: None
  }
}

#[cfg(test)]
mod tests {
  use start;
  use Player;
  use Winner;
  use make_turn;
  
  #[test]
  fn inital_board_is_five() {
    let game_state = start();
    assert_eq!(game_state.board.len(), 5);
  }
  
  #[test]
  fn cross_starts() {
    let game_state = start();
    assert_eq!(game_state.next_player, Player::Cross);
  }

  #[test]
  fn make_turn_changes_player() {
    let game_state = start();
    let game_state = make_turn(game_state, 0, 0);
    assert_eq!(game_state.unwrap().next_player, Player::Circle);
  }

  #[test]
  fn no_winner() {
    let game_state = start();
    assert_eq!(game_state.winner, None);
  }

  #[test]
  fn column_win() {
    let game_state = start();
    let game_state = make_turn(game_state, 0, 0);
    let game_state = make_turn(game_state.unwrap(), 1, 0);
    let game_state = make_turn(game_state.unwrap(), 0, 1);
    let game_state = make_turn(game_state.unwrap(), 1, 1);
    let game_state = make_turn(game_state.unwrap(), 0, 2);
    assert_eq!(game_state.unwrap().winner.unwrap(), Winner::Cross);
  }

  #[test]
  fn row_win() {
    let game_state = start();
    let game_state = make_turn(game_state, 0, 0);
    let game_state = make_turn(game_state.unwrap(), 0, 1);
    let game_state = make_turn(game_state.unwrap(), 1, 0);
    let game_state = make_turn(game_state.unwrap(), 1, 1);
    let game_state = make_turn(game_state.unwrap(), 2, 0);
    assert_eq!(game_state.unwrap().winner.unwrap(), Winner::Cross);
  }

  #[test]
  fn diagonal_win() {
    let game_state = start();
    let game_state = make_turn(game_state, 1, 0);
    let game_state = make_turn(game_state.unwrap(), 1, 1);
    let game_state = make_turn(game_state.unwrap(), 2, 1);
    let game_state = make_turn(game_state.unwrap(), 0, 2);
    let game_state = make_turn(game_state.unwrap(), 3, 2);
    assert_eq!(game_state.unwrap().winner.unwrap(), Winner::Cross);
  }

  #[test]
  fn diagonal_win2() {
    let game_state = start();
    let game_state = make_turn(game_state, 4, 1);
    let game_state = make_turn(game_state.unwrap(), 1, 0);
    let game_state = make_turn(game_state.unwrap(), 3, 2);
    let game_state = make_turn(game_state.unwrap(), 1, 3);
    let game_state = make_turn(game_state.unwrap(), 2, 3);
    assert_eq!(game_state.unwrap().winner.unwrap(), Winner::Cross);
  }
}
