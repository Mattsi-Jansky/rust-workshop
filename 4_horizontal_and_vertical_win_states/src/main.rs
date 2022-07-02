use std::io;
use tic_tac_toe::{Game, MoveResult};

fn main() {
    let mut game = Game::new();
    let mut result = MoveResult::Ongoing(game);
    println!("{}", game.render());

    while matches!(result, MoveResult::Ongoing(_)) || matches!(result, MoveResult::IllegalMove) {
        println!("{}", game.render());
        let input = parse_input();
        match input {
            Ok((x, y)) => {
                result = game.make_move(x, y);
            }
            Err(error) => println!("{}", error),
        }

        if let MoveResult::Ongoing(new_game) = result {
            game = new_game;
        } else if matches!(result, MoveResult::IllegalMove) {
            println!("Illegal move. Use format `x,y` eg `0,0` (zero-indexed)")
        }
    }

    println!("===Game over===");
    match result {
        MoveResult::WinFirstPlayer(display) => {
            println!("{}\nFirst player (O) wins!", display)
        }
        MoveResult::WinSecondPlayer(display) => {
            println!("{}\nSecond player (X) wins! ", display)
        }
        MoveResult::Draw(display) => {
            println!("{}\nDraw!", display)
        }
        MoveResult::IllegalMove | MoveResult::Ongoing(_) => {
            panic!("Impossible to reach here")
        }
    }
}

fn parse_input() -> Result<(usize, usize), String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            input = input.lines().last().unwrap().to_string();
            if n != 4 || !input.contains(",") {
                Err("Error, bad command. Use format `x,y` eg `0,0` (zero-indexed)".to_string())
            } else {
                let mut input_chars = input.chars();
                let x = input_chars.next().unwrap() as usize - 0x30;
                input_chars.next();
                let y = input_chars.next().unwrap() as usize - 0x30;
                Ok((x, y))
            }
        }
        Err(error) => Err(format!("error: {}. Try again", error)),
    }
}
