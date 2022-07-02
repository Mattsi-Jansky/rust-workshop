# Rust Workshop Tic-Tac-Toe

This repo contains a very basic Rust programming challenge for beginner workshops, using Tic-Tac-Toe aka Norts and Crosses. It is intended that this workshop is done in-person but can be continued at your leisure in a self-directed learning style, in the highly likely scenario that we do not finish during the session.

The workshop is designed in such a way that practicing Test Driven Development (TDD) is very fitting, and you will find it a lot easier if you write tests before your functionality. 

There are 5 steps. Each step has a project you can start from. These are provided so that if you fall behind or are completely stuck on a particular step, you can move to the next step using the provided project for that step. Otherwise, you can keep extending your project from the previous step. You can also read the next step solution for inspiration if you are stuck.

The workshop is written under the assumption you already have Rust setup on your machine. If not, you should do that first. I also recommend installing a Rust plugin in your favourite IDE.

## Run tests

Use `cargo test` to run the tests. One test is provided to start with in `1_start`, and the later steps each have more tests.

## Run application

Use `cargo run` to run your game. To begin with it will fail because the `render` and `make_move` functions are not implemented. But once you have implemented rendering and it can understand at least some moves, you'll be able to play the game.

## Steps

[Step One - Rendering](steps/1.md)
Step Two - Validating Moves
Step Three - Winning, Horizontally and Vertically
Step Four - Winning, Diagonally
Step Five - Drawing
