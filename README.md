# 🎶 Cadenza Compiler 🎵

This project is a **musical note compiler** that transforms arithmetic expressions using notes from the staff. Imagine blending music and math: that's what we do here!

## 🌟 Features

- **Lexical, Syntactic, and Semantic Analysis**: Your musical input gets broken down and analyzed with precision.
- **Musical Notes**: Work with notes that convert into numbers in base 12.
- **Arithmetic Operations**: Addition, subtraction, multiplication, and division... with a musical twist!

## 🎼 Musical Notes

We use the following notes in our system (in base 12):

- **Notes**: c, c#, d, d#, e, f, f#, g, g#, a, a#, b

### Example Conversion

- **`c#`**: Note "C sharp"
- **`c#c`**: That’s 12! 🎉

## ➕ Arithmetic Operations

The compiler can perform the following arithmetic operations in our musical style:

- **Addition**: `value1 a value2`  
  _(Example: `c#c a c#` -> `12 + 1`)_

- **Subtraction**: `value1 g value2`  
  _(Example: `c#c g c#` -> `12 - 1`)_

- **Multiplication**: `value1 f value2`  
  _(Example: `c#c f c#` -> `12 _ 1`)\*

- **Division**: `value1 e value2`  
  _(Example: `c#c e c#` -> `12 / 1`)_

## 🚀 Requirements

Make sure you have Rust and Cargo installed on your system. If not, it’s time to join the party! 🥳 You can install Rust by following the instructions on [rustup.rs](https://rustup.rs/).

## ⚡ Running the Project

1. **Clone the repository** or **download the project**.
2. **Navigate to the project directory**.
3. Run the following command to compile and execute the program:

   ```bash
   cargo run
   ```
