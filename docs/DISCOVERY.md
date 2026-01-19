# Salvo – Discovery

This document captures the agreed‑upon requirements, constraints, and guiding decisions for the Salvo game. It is intended to serve as a shared contract for what we are building, not how it is implemented.

## Game Overview

- Single‑player game: **human player vs. computer opponent**
- Supports **multiple games per session**
- Implemented in **Rust**
- Runs locally for now, but architecture should not preclude future networking

## Rules

- Classic Battleship rules for sinking ships: a ship is sunk only when all of its cells are hit
- Default mode: **one shot per turn**
- Optional mode (**Salvo mode**): number of shots per turn equals the number of ships remaining
- Turn model:
  - One side fires all of its shots for the turn (one or multiple)
  - All shots resolve immediately
  - Control then passes to the other side
- The game ends immediately when one side’s final ship is sunk
- No draw condition; strict turn order determines the winner

## Computer Opponent

- Automatically places ships
- Remembers previous hits
- After a hit, targets adjacent cells intelligently
- No difficulty levels

## Board and Ships

- Board size and ship configuration are **configurable**
- Defaults match the classic Battleship setup

## Ship Placement (Player)

- Player may choose between:
  - Automatic/random placement
  - Manual placement (choosing coordinates and orientation)

## Interface

- Primary interface is a **command‑line TUI**
- Core game logic must be interface‑agnostic to allow future UIs

## Persistence

- Disk‑backed persistence is required
- Persisted data includes:
  - Player statistics (e.g. games played, wins/losses)
  - Game settings/configuration
  - In‑progress games
- Finished games:
  - Are **not reloadable**
  - Must be cleaned up from storage after completion
  - Cleanup requires explicit user confirmation

## Endgame Behavior

- When the game ends:
  - Outcome is recorded in player statistics
  - Both full boards (player and computer) are revealed
  - Control returns to the session flow (e.g. replay or quit)

## Randomness and Testing

- Production gameplay uses real randomness
- Tests must be deterministic
- Code must be structured so that random behavior can be replaced with a deterministic implementation in tests
- Tests should focus on **observable behavior**, not implementation details

## Engineering Preferences

- Bias toward **speed of implementation** over heavy upfront abstraction
- Strict correctness and validation
- Thorough behavior‑level testing
