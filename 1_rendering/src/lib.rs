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
        let mut result = String::new();

        unimplemented!()
    }

    pub fn make_move(&self, x: usize, y: usize) -> MoveResult {
        let mut new_state = self.state;

        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_render_blank_board() {
        let game = Game::new();

        let actual = game.render();

        assert_eq!(
            concat!(
                "   \n",
                "   \n",
                "   "),
            actual)
    }

    // #[test]
    // fn first_move_should_place_x_on_screen() {
    //     let mut game = Game::new();
    //
    //     game = game.make_move(0, 0).unwrap();
    //
    //     let actual = game.render();
    //     assert_eq!(concat!("O  \n", "   \n", "   "), actual)
    // }

}
