# Step Two - Validating Moves

## Introduction

This step will be a bit shorter! All you need to do is validate that a move is legal, and return `IllegalMove` if it is not.

If you did not complete the previous step start from `2_validate_moves`. Otherwise, continue from the same project.

## Goals

* Add a new test for the test case below and make it pass
  * The `make_move` function should notice that a piece has already been played on `0,0` and so return `IllegalMove` instead of `OngoingGame`.

## Test Cases

| Input | Output   |
|-------|----------|
| `make_move(0, 0)`<br/>`make_move(0, 0)`      | `IllegalMove` |

## Hints

* You will need to change the assertion of your new test to expect to see `IllegalMove`. You will not need (nor be able to) call `render` at all.
  * Note how the use of the `MoveResult` enum prevents you from being able to call `make_move` if the previous move was illegal. This prevents you from being able to produce an incorrect program. Compile-time checks like these are Rust's bread & butter. 
* You can use `matches!` to check whether a variable matches an enum. For example, `matches!(strawberry, FoodTypes::Berry)`

## Run tests

Use `cargo test` to run the tests.

## Run application

Use `cargo run` to run your game.
