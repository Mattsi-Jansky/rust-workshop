# Step Three - Horizontal & Vertical Win States

## Introduction

This time we are going to return `WinFirstPlayer` or `WinSecondPlayer` if one of them gets three pieces in a vertical or horizontal row.

After completing this step, you should be able to play a full game of Tic Tac Toe with `cargo run` and get a win!

If you did not complete the previous step start from `3_horizontal_and_vertical_win_states`. Otherwise, continue from the same project.

## Goals

* Add new tests for the test cases below and make them pass

## Test Cases

*Note:* `WinFirstPlayer` and `WinSecondPlayer` take a `String` that should contain an ASCII rendering (i.e. output of `render` method) of the final state of the game. Until now `Output` has only included the expected render outcome, but where another state such as `WinFirstPlayer` is specified you should assert that the output is also of the correct enum variant.

| Description                             | Input                                                                                                                           | Output                                                                 |
|-----------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------------------------------|
| win_first_player_horizontal             | `make_move(0, 0)`<br/>`make_move(0, 1)`</br>`make_move(1, 0)`<br/>`make_move(1, 1)`<br/>`make_move(2, 0)`                       | `WinFirstPlayer`<br/><pre>OOO\n</pre><pre>XX \n</pre><pre>   </pre> |
| win_first_player_horizontal_second_row  | `make_move(0, 1)`<br/>`make_move(0, 0)`</br>`make_move(1, 1)`<br/>`make_move(1, 0)`<br/>`make_move(2, 1)`                       | `WinFirstPlayer`<br/><pre>XX \n</pre><pre>OOO\n</pre><pre>   </pre>                      |
| win_second_player_horizontal            | `make_move(0, 0)`<br/>`make_move(0, 1)`</br>`make_move(1, 0)`<br/>`make_move(1, 1)`<br/>`make_move(2, 2)`<br/>`make_move(2, 1)` | `WinSecondPlayer`<br/><pre>OO \n</pre><pre>XXX\n</pre><pre>   </pre>                      |
| win_first_player_vertical               | `make_move(0, 0)`<br/>`make_move(1, 0)`</br>`make_move(0, 1)`<br/>`make_move(1, 1)`<br/>`make_move(0, 2)`                       | `WinFirstPlayer`<br/><pre>OX \n</pre><pre>OX \n</pre><pre>O  </pre>                      |
| win_first_player_vertical_second_column | `make_move(1, 0)`<br/>`make_move(2, 0)`</br>`make_move(1, 1)`<br/>`make_move(2, 1)`<br/>`make_move(1, 2)`                       | `WinFirstPlayer`<br/><pre> OX\n</pre><pre> OX\n</pre><pre> O </pre>                      |
| win_second_player_vertical              | `make_move(0, 0)`<br/>`make_move(2, 0)`</br>`make_move(0, 1)`<br/>`make_move(2, 1)`<br/>`make_move(1, 1)`<br/>`make_move(2, 2)` | `WinSecondPlayer`<br/><pre>O X\n</pre><pre>OOX\n</pre><pre>  X</pre>                      |

## Hints

* This assertion is different to the previous tests. Previously you had to check the type if it was an `IllegalMove`, or otherwise check the render output. This time, you have to check both.
* The `if let` syntax is helpful for extracting the `String` from `WinFirstPlayer` or `WinSecondPlayer`
* You can use `panic!("reason")` to fail a test
* You can compare to _instances_ of an enum with `==`, such as `let a = Cell::None; let b = Cell::None; a == b`. But you _can't_ use `==` to compare an instance of an enum to an enum type, such as `a == Cell::None`. For that you need to use `matches!`.
* Remember that you can use `matches!(variable, Enum::Variant)` to check if a variable is of a particular enum variant
