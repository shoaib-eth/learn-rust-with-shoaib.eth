# Guessing Game in Rust

Welcome to the **Guessing Game** project! This is a simple number-guessing game built using Rust. The program randomly generates a secret number between 1 and 100, and the player has to guess it. The game provides feedback whether the guess is too high, too low, or correct.

## How It Works

1. The program generates a random secret number between 1 and 100.
2. The user is prompted to input a guess.
3. The program compares the user’s guess with the secret number:
   - If the guess is equal to the secret number, the user wins, and the game ends.
   - If the guess is less than the secret number, the program displays "Too small!".
   - If the guess is greater than the secret number, the program displays "Too big!".
4. The game continues in a loop until the user guesses the correct number.

## Concepts Learned

This project was a hands-on way to introduce several Rust concepts:
- **`let` keyword:** Used to create mutable and immutable variables.
- **`match` statement:** Pattern matching is used to compare the guess with the secret number.
- **Functions:** `main`, `read_line`, and `cmp` functions are used for various operations.
- **External Crates:** The `rand` crate is used to generate a random number.
- **Error Handling:** The program gracefully handles invalid inputs using `match` and `parse()`.

## Prerequisites

Make sure you have Rust installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org/).

## How to Run the Game

1. Clone the repository:
   ```bash
   git clone https://github.com/username/Guessing_Game.git
    ```

2. Navigate to the project directory:
   ```bash
   cd Guessing_Game
    ```

3. Run the game using Cargo:
   ```bash
    cargo run
    ```
    Now you can start guessing the number!