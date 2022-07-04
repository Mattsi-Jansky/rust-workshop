# Step Four - Diagonal Win States

## Introduction

Now we're going to extend the previous functionality to also account for diagonal checks. This should be a shorter step.

If you did not complete the previous step start from `4_diagonal_win_states`. Otherwise, continue from the same project.

## Goals

* Add new tests for the test cases below and make them pass

## Test Cases

| Description                                  | Input                                                                                                                           | Output                                                              |
|----------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------|
| win_first_player_diagonal                    | `make_move(0, 0)`<br/>`make_move(1, 0)`</br>`make_move(1, 1)`<br/>`make_move(2, 1)`<br/>`make_move(2, 2)`                       | `WinFirstPlayer`<br/><pre>OX \n</pre><pre>OX \n</pre><pre>  O</pre> |
| win_first_player_diagonal_opposite_direction | `make_move(2, 0)`<br/>`make_move(1, 0)`</br>`make_move(1, 1)`<br/>`make_move(2, 1)`<br/>`make_move(0, 2)`                       | `WinFirstPlayer`<br/><pre> XO\n</pre><pre> OX\n</pre><pre>O  </pre> |
| win_second_player_diagonal                   | `make_move(1, 0)`<br/>`make_move(0, 0)`</br>`make_move(2, 1)`<br/>`make_move(1, 1)`<br/>`make_move(1, 2)`<br/>`make_move(2, 2)` | `WinSecondPlayer`<br/><pre>XO \n</pre><pre>XO \n</pre><pre>OX </pre> |

## Hints

* There are only two possible win states to check here, and it is simpler and faster to check them manually than to try do it algorithmically.

## Run tests

Use `cargo test` to run the tests.

## Run application

Use `cargo run` to run your game.
