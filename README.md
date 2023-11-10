# Guessing Game

A Simple Terminal/CLI guessing game written in Rust. The program will generate a random integer between 1 and 100. It will then prompt the player to enter a guess. After a guess is entered, the program will indicate whether the guess is too low or too high. If the guess is correct, the game will print a congratulatory message and exit.

## Getting Started

### Prerequisites

Make sure you have [Rust](https://www.rust-lang.org/) installed on your system.

### Running the Game

1. Clone the repository to your local machine:

   ```bash
   git clone https://github.com/obinnafranklinduru/guessing_game
   ```

2. Navigate to the project directory:

   ```bash
   cd guessing_game
   ```

3. Build and run the game:

   ```bash
   cargo run
   ```

## How to Play

1. The game will prompt you to input your guess.
2. Enter a number and press Enter.
3. The program will provide feedback on whether your guess is too small, too big, or correct.
4. Keep guessing until you correctly identify the secret number.

## Features

- Randomly generated secret number within a specified range.
- User input validation for valid number entries.
- Color-coded feedback for a better user experience.

## Contributing

If you'd like to contribute to the project, please follow the [contribution guidelines](https://github.com/obinnafranklinduru/guessing_game/blob/main/CONTRIBUTING.md).

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/obinnafranklinduru/guessing_game/blob/main/LICENSE) file for details.

## Acknowledgments

- [Colored](https://docs.rs/colored/) - A crate for coloring terminal text in Rust.
- [rand](https://docs.rs/rand/) - A crate for random number generation in Rust.

Feel free to fork, modify, and use this code for your own projects. Happy coding!
