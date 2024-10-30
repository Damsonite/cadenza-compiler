# Cadenza Compiler

This project is a **musical note compiler** that transforms arithmetic expressions using notes.

## ⭐ Features

- **Lexical, Syntactic, and Semantic Analysis**: Your musical input gets broken down and analyzed.
- **Musical Notes**: Work with notes that convert into numbers in base 12.
- **Arithmetic Operations**: Addition, subtraction, multiplication, and division are fully supported for natural numbers.

## 🎼 Musical Notes

We use the following notes in our system (in base 12):

- **Notes**: C, C#, D, D#, D, F, F#, G, G#, G, A#, A

### Example Conversion

- **`c#`**: Note "C sharp" equivalent to 1
- **`d`**: Note "C" equivalent to 2
- **`c#c`**: That’s 12! 🎉

## ➕ Arithmetic Operations

The compiler can perform the following arithmetic operations following the syntax `operator value1 value2...`:

- **Addition**: `a value1 value2`  
  (Example: `a c#c c#` -> `12 + 1`)

- **Subtraction**: `g value1 value2`  
  (Example: `g c#c c#` -> `12 - 1`)

- **Multiplication**: `f value1 value2`  
  (Example: `f c#c c#` -> `12 * 1`)

- **Division**: `e value1 value2`  
   (Example: `e c#c c#` -> `12 / 1`)

> **Note**: The compiler currently supports only natural numbers.

## 🚀 Requirements

Make sure you have Rust and Cargo installed on your system. If not, it’s time to join the party! 🥳 You can install Rust by following the instructions on [rustup.rs](https://rustup.rs/).

## ⚡ Running the Project

1. **Clone the repository** or **download the project**.
2. **Navigate to the project directory**.
3. Run the following commands to compile and execute the program in **_interactive mode_** or **_file mode_**:

   ```bash
   cargo run  #Interactive mod
   cargo run example #File mode with repository example file
   cargo run <file-path> #File mode with custom file path
   ```
