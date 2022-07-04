# Step Four - Draws

## Introduction

If we find that all cells have been used and we are not in a win state yet, then we should declare a draw- by returning `MoveResult::Draw`.

After completing this step, your Tic Tac Toe game will be fully-featured! Play it with `cargo run`

## Goals

* Add new tests for the test cases below and make them pass

## Test Cases

| Description                                     | Input                                                                                                                                                                                             | Output                                                  |
|-------------------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|---------------------------------------------------------|
| given_all_cells_filled_and_not_winstate_is_draw | `make_move(0, 0)`<br/>`make_move(1, 0)`</br>`make_move(2, 0)`<br/>`make_move(0, 1)`<br/>`make_move(0, 2)`<br/>`make_move(1, 1)`<br/>`make_move(2, 1)`<br/>`make_move(2, 2)`<br/>`make_move(1, 2)` | `Draw`<br/><pre>OXO\n</pre><pre>XXO\n</pre><pre>OOX</pre> |


## Hints

* This logic can be chained off of your win state conditionals in the `make_move` function.
