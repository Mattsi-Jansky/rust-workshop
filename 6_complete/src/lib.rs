#[derive(Clone, Copy, Debug, PartialEq)]
enum Cell {
    Nort,
    Cross,
    None,
}

#[derive(Debug)]
pub enum MoveResult {
    Ongoing(Game),
    IllegalMove,
    WinFirstPlayer(String),
    WinSecondPlayer(String),
    Draw(String),
}

impl MoveResult {
    pub fn unwrap(self) -> Game {
        match self {
            MoveResult::Ongoing(game) => game,
            _ => {
                panic!("Cannot play from this state")
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Game {
    state: [Cell; 9],
    is_first_player_turn: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            state: [
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::None,
            ],
            is_first_player_turn: true,
        }
    }

    pub fn render(&self) -> String {
        render_state(self.state)
    }

    pub fn make_move(&self, x: usize, y: usize) -> MoveResult {
        let mut state = self.state;
        if matches!(state[x + (y * 3)], Cell::None) {
            state[x + (y * 3)] = match self.is_first_player_turn {
                true => Cell::Nort,
                false => Cell::Cross,
            };
            if is_win_state(state) {
                if self.is_first_player_turn {
                    MoveResult::WinFirstPlayer(render_state(state))
                } else {
                    MoveResult::WinSecondPlayer(render_state(state))
                }
            } else if !state.contains(&Cell::None) {
                MoveResult::Draw(render_state(state))
            } else {
                MoveResult::Ongoing(Game {
                    state,
                    is_first_player_turn: !self.is_first_player_turn,
                })
            }
        } else {
            MoveResult::IllegalMove
        }
    }
}

fn is_win_state(state: [Cell; 9]) -> bool {
    let mut result = false;

    for i in 0..=2 {
        test_win_horizontal(&state, &mut result, i);
        test_win_vertical(state, &mut result, i)
    }
    test_win_diagonal(state, &mut result);

    result
}

fn test_win_horizontal(state: &[Cell; 9], result: &mut bool, i: usize) {
    let row = i * 3;
    if !matches!(state[row], Cell::None) {
        let cell_type = state[row];
        if state[row + 1] == cell_type && state[row + 2] == cell_type {
            *result = true;
        }
    }
}

fn test_win_vertical(state: [Cell; 9], result: &mut bool, i: usize) {
    if !matches!(state[i], Cell::None) {
        let cell_type = state[i];
        if state[i + 3] == cell_type && state[i + 6] == cell_type {
            *result = true;
        }
    }
}

fn test_win_diagonal(state: [Cell; 9], result: &mut bool) {
    if !matches!(state[4], Cell::None) {
        let cell_type = state[4];
        if (state[0] == cell_type && state[8] == cell_type)
            || (state[2] == cell_type && state[6] == cell_type)
        {
            *result = true;
        }
    }
}

fn render_state(state: [Cell; 9]) -> String {
    let mut result = String::new();

    for (i, cell) in state.iter().enumerate() {
        result.push_str(match cell {
            Cell::Nort => "O",
            Cell::Cross => "X",
            Cell::None => " ",
        });

        let is_last_character_in_line = (i + 1) % 3 == 0;
        let is_not_last_line = i < 8;
        if  is_last_character_in_line && is_not_last_line {
            result.push('\n')
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_blank_board() {
        let game = Game::new();

        let actual = game.render();

        assert_eq!(concat!("   \n", "   \n", "   "), actual)
    }

    #[test]
    fn first_move_should_place_x_on_screen() {
        let mut game = Game::new();

        game = game.make_move(0, 0).unwrap();

        let actual = game.render();
        assert_eq!(concat!("O  \n", "   \n", "   "), actual)
    }

    #[test]
    fn second_move_should_place_o_on_screen() {
        let mut game = Game::new();

        game = game.make_move(0, 0).unwrap();
        game = game.make_move(1, 0).unwrap();

        let actual = game.render();
        assert_eq!(concat!("OX \n", "   \n", "   "), actual)
    }

    #[test]
    fn move_on_second_row() {
        let mut game = Game::new();

        game = game.make_move(0, 1).unwrap();

        let actual = game.render();
        assert_eq!(concat!("   \n", "O  \n", "   "), actual)
    }

    #[test]
    fn cannot_play_in_position_already_played_in() {
        let mut game = Game::new();

        game = game.make_move(0, 0).unwrap();
        let result = game.make_move(0, 0);

        assert!(matches!(result, MoveResult::IllegalMove))
    }

    #[test]
    fn win_first_player_horizontal() {
        let mut game = Game::new();

        game = game.make_move(0, 0).unwrap();
        game = game.make_move(0, 1).unwrap();
        game = game.make_move(1, 0).unwrap();
        game = game.make_move(1, 1).unwrap();
        let result = game.make_move(2, 0);

        if let MoveResult::WinFirstPlayer(display) = result {
            assert_eq!(concat!("OOO\n", "XX \n", "   "), display)
        } else {
            panic!("Expected WinFirstPlayer, got {:?}", result)
        }
    }

    #[test]
    fn win_first_player_horizontal_second_row() {
        let mut game = Game::new();

        game = game.make_move(0, 1).unwrap();
        game = game.make_move(0, 0).unwrap();
        game = game.make_move(1, 1).unwrap();
        game = game.make_move(1, 0).unwrap();
        let result = game.make_move(2, 1);

        if let MoveResult::WinFirstPlayer(display) = result {
            assert_eq!(concat!("XX \n", "OOO\n", "   "), display)
        } else {
            panic!("Expected WinFirstPlayer, got {:?}", result)
        }
    }

    #[test]
    fn win_second_player_horizontal() {
        let mut game = Game::new();

        game = game.make_move(0, 0).unwrap();
        game = game.make_move(0, 1).unwrap();
        game = game.make_move(1, 0).unwrap();
        game = game.make_move(1, 1).unwrap();
        game = game.make_move(2, 2).unwrap();
        let result = game.make_move(2, 1);

        if let MoveResult::WinSecondPlayer(display) = result {
            assert_eq!(concat!("OO \n", "XXX\n", "  O"), display)
        } else {
            panic!("Expected WinSecondPlayer, got {:?}", result)
        }
    }

    #[test]
    fn win_first_player_vertical() {
        let mut game = Game::new();

        game = game.make_move(0, 0).unwrap();
        game = game.make_move(1, 0).unwrap();
        game = game.make_move(0, 1).unwrap();
        game = game.make_move(1, 1).unwrap();
        let result = game.make_move(0, 2);

        if let MoveResult::WinFirstPlayer(display) = result {
            assert_eq!(concat!("OX \n", "OX \n", "O  "), display)
        } else {
            panic!("Expected WinFirstPlayer, got {:?}", result)
        }
    }

    #[test]
    fn win_first_player_vertical_second_column() {
        let mut game = Game::new();

        game = game.make_move(1, 0).unwrap();
        game = game.make_move(2, 0).unwrap();
        game = game.make_move(1, 1).unwrap();
        game = game.make_move(2, 1).unwrap();
        let result = game.make_move(1, 2);

        if let MoveResult::WinFirstPlayer(display) = result {
            assert_eq!(concat!(" OX\n", " OX\n", " O "), display)
        } else {
            panic!("Expected WinFirstPlayer, got {:?}", result)
        }
    }

    #[test]
    fn win_second_player_vertical() {
        let mut game = Game::new();

        game = game.make_move(0, 0).unwrap();
        game = game.make_move(2, 0).unwrap();
        game = game.make_move(0, 1).unwrap();
        game = game.make_move(2, 1).unwrap();
        game = game.make_move(1, 1).unwrap();
        let result = game.make_move(2, 2);

        if let MoveResult::WinSecondPlayer(display) = result {
            assert_eq!(concat!("O X\n", "OOX\n", "  X"), display)
        } else {
            panic!("Expected WinSecondPlayer, got {:?}", result)
        }
    }

    #[test]
    fn win_first_player_diagonal() {
        let mut game = Game::new();

        game = game.make_move(0, 0).unwrap();
        game = game.make_move(1, 0).unwrap();
        game = game.make_move(1, 1).unwrap();
        game = game.make_move(2, 1).unwrap();
        let result = game.make_move(2, 2);

        if let MoveResult::WinFirstPlayer(display) = result {
            assert_eq!(concat!("OX \n", " OX\n", "  O"), display)
        } else {
            panic!("Expected WinFirstPlayer, got {:?}", result)
        }
    }

    #[test]
    fn win_first_player_diagonal_opposite_direction() {
        let mut game = Game::new();

        game = game.make_move(2, 0).unwrap();
        game = game.make_move(1, 0).unwrap();
        game = game.make_move(1, 1).unwrap();
        game = game.make_move(2, 1).unwrap();
        let result = game.make_move(0, 2);

        if let MoveResult::WinFirstPlayer(display) = result {
            assert_eq!(concat!(" XO\n", " OX\n", "O  "), display)
        } else {
            panic!("Expected WinFirstPlayer, got {:?}", result)
        }
    }

    #[test]
    fn win_second_player_diagonal() {
        let mut game = Game::new();

        game = game.make_move(1, 0).unwrap();
        game = game.make_move(0, 0).unwrap();
        game = game.make_move(2, 1).unwrap();
        game = game.make_move(1, 1).unwrap();
        game = game.make_move(1, 2).unwrap();
        let result = game.make_move(2, 2);

        if let MoveResult::WinSecondPlayer(display) = result {
            assert_eq!(concat!("XO \n", " XO\n", " OX"), display)
        } else {
            panic!("Expected WinSecondPlayer, got {:?}", result)
        }
    }

    #[test]
    fn given_all_cells_filled_and_not_winstate_is_draw() {
        let mut game = Game::new();

        game = game.make_move(0, 0).unwrap();
        game = game.make_move(1, 0).unwrap();
        game = game.make_move(2, 0).unwrap();
        game = game.make_move(0, 1).unwrap();
        game = game.make_move(0, 2).unwrap();
        game = game.make_move(1, 1).unwrap();
        game = game.make_move(2, 1).unwrap();
        game = game.make_move(2, 2).unwrap();
        let result = game.make_move(1, 2);

        if let MoveResult::Draw(display) = result {
            assert_eq!(concat!("OXO\n", "XXO\n", "OOX"), display)
        } else {
            panic!("Expected Draw, got {:?}", result)
        }
    }
}
