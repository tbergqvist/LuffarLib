use game_state::{Player, GameBoard};

pub fn check_win_condition(board: &GameBoard, player: Player, required_row_length: usize) -> bool {
  let player = Some(player);

  check_columns(&board, player, required_row_length) ||
    check_rows(&board, player, required_row_length) ||
    check_diagonal(&board, player, required_row_length)
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
    (0..x + 1).filter(|y| *y < board.len())
    .map(|y| board[y][(x - y)])
    .scan(0, |current_count, cell| count_same(current_count, cell, player))
    .skip_while(|count| *count < required_row_length)
    .next()
    .is_some()
  })
  ||
  (2..board[0].len()).fold(false, |won, x| {
    won ||
    (0..x + 1).filter(|y| *y < board.len())
    .map(|y| board[board.len() - y - 1][board[0].len() - (x - y)- 1])
    .scan(0, |current_count, cell| count_same(current_count, cell, player))
    .skip_while(|count| *count < required_row_length)
    .next()
    .is_some()
  })
}

#[cfg(test)]
mod tests {
  use Player;
  use win_check::check_win_condition;

  #[test]
  fn row_win() {
    let board = vec![
      vec![Some(Player::Circle), Some(Player::Circle), Some(Player::Circle)],
      vec![None, None, None],
      vec![None, None, None],
    ];
    assert!(check_win_condition(&board, Player::Circle, 3));
  }
  
  #[test]
  fn column_win() {
    let board = vec![
      vec![Some(Player::Circle), None, None],
      vec![Some(Player::Circle), None, None],
      vec![Some(Player::Circle), None, None],
    ];
    assert!(check_win_condition(&board, Player::Circle, 3));
  }
  #[test]
  fn diagonal_win() {
    let board = vec![
      vec![Some(Player::Circle), None, None],
      vec![None, Some(Player::Circle), None],
      vec![None, None, Some(Player::Circle)],
    ];
    assert!(check_win_condition(&board, Player::Circle, 3));
  }

  #[test]
  fn diagonal_win2() {
    let board = vec![
      vec![None, None, Some(Player::Circle)],
      vec![None, Some(Player::Circle), None],
      vec![Some(Player::Circle), None, None],
    ];
    assert!(check_win_condition(&board, Player::Circle, 3));
  }

  #[test]
  fn diagonal_win_3x4() {
    let board = vec![
      vec![None, Some(Player::Circle), None, None],
      vec![None, None, Some(Player::Circle), None],
      vec![None, None, None, Some(Player::Circle)],
    ];
    assert!(check_win_condition(&board, Player::Circle, 3));
  }

}