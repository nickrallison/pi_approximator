# pi_calculator_parallel

`pi_calculator_parallel` is a Rust-based command-line application that calculates the number of lattice points within a circle and provides an approximation of the value of pi. The application makes use of parallel computation for efficient processing, particularly suitable for large input values.

## Features

- Calculates the number of lattice points inside a circle with a radius derived from the square root of a given number.
- Provides an approximation of pi based on the number of lattice points.
- Utilizes parallel processing for performance optimization.

## Installation

To install and run the `pi_calculator_parallel` application, ensure that you have Rust and Cargo installed on your system. Follow the steps below:

1. Clone the repository:
   ```bash
   git clone https://github.com/nickrallison/pi_calculator_parallel.git
   ```
2. Navigate to the project directory:
   ```bash
   cd pi_calculator_parallel
   ```
3. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

## Usage

To use the `pi_calculator_parallel` program, execute the compiled binary with the required `--num` argument which specifies the number to calculate the lattice points for.

```bash
cargo run --release -- --num <your_number>
```

Replace `<your_number>` with a positive integer to compute the number of lattice points and an estimation of pi.

Example:
```bash
cargo run --release -- --num 25
```

## How It Works

The program determines the number of lattice points inside a circle using a mathematical approach inspired by a YouTube video on similar algorithms. It calculates:
- The number of lattice points using prime factorization and a chi function, leveraging parallel computation for efficient processing.
- An approximation of pi by comparing the lattice points to the circle's defined area.

## Dependencies

This project uses the following Rust crates:

- `clap` for command-line argument parsing.
- `rayon` for parallel iteration and computation.
- `prime_factorization` to handle prime factorization of numbers.
- `rstest` for testing the functionality of the codebase.

All dependencies are listed in the `Cargo.toml` file.

## Acknowledgments

The algorithm implemented in this project is inspired by a YouTube video on calculating lattice points and approximating pi. Check out the video [here](https://www.youtube.com/watch?v=NaL_Cb42WyY).

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

## Contribution

Contributions, issues, and feature requests are welcome! Feel free to check the [issues page](https://github.com/nickrallison/pi_calculator_parallel/issues) and create a pull request.

## Contact

For any inquiries or further information, please contact nickrallison1@gmail.com.
