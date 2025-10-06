# calc-rs 🧮  
> A smart terminal calculator with variables, step-by-step RPN evaluation, and Shunting Yard algorithm — all in pure Rust.

[![Crates.io](https://img.shields.io/crates/v/calc-rs?style=flat-square)](https://crates.io/crates/calc-rs)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)

![calc-rs demo](demo.gif)

Ever wondered how `(8 - 3) * 5` becomes `25` inside a calculator?  
**calc-rs shows you every step** — while supporting variables, error recovery, and an interactive REPL.

Built with ❤️ in Rust. Zero unsafe code. Zero bloat.

---

## ✨ Features

- **Variables**: `x = 10; y = x * 2; x + y`
- **Step-by-step RPN tracing**: See how expressions are evaluated using a stack
- **Interactive REPL**: Like Python’s shell — type, assign, compute
- **Shunting Yard Algorithm**: Converts infix → RPN correctly with operator precedence
- **Clear error messages**: Know exactly where your expression went wrong
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Lightweight**: No external dependencies (except `rustyline` for REPL)

---

## 🚀 Install

### From Crates.io (recommended)
```bash
$ cargo install calc-rs
```

### From source
```bash
$ git clone https://github.com/yourusername/calc-rs
$ cd calc-rs
$ cargo install --path .
```
💡 Make sure you have Rust installed.

## 🎮 Usage
### One-off calculation 
```bash 
$ calc "(8 - 3) * 5"
# Output: 25
```

### Interactive REPL (like Python!) 
```bash 
$ calc
```
#### Then:
```bash
>>> x = 5
5
>>> (x + 2) * 3
Step 1: Push 5 → Stack: [5]
Step 2: Push 2 → Stack: [5, 2]
Step 3: Apply '+' → Result: 7 → Stack: [7]
Step 4: Push 3 → Stack: [7, 3]
Step 5: Apply '*' → Result: 21 → Stack: [21]
21
>>> exit
```

### 🛠 How It Works 

* Tokenization: Splits input into numbers, operators, parentheses, and variables.
* Shunting Yard: Converts infix notation (e.g., 2 + 3 * 4) to Reverse Polish Notation (2 3 4 * +).
* RPN Evaluation: Uses a stack to evaluate the expression — and explains each step.
* Error Handling: Detects unmatched parentheses, invalid tokens, division by zero, and undefined variables.

### Examples
```bash
Input: (5 + 1 - 1
Shunting Yard: 5 1 + 1 - (
❌ Error: Invalid token '(' at end of expression
```   

## 📦 Built With
* Rust 🦀 — for memory safety, performance, and correctness
* Shunting Yard Algorithm — Dijkstra’s classic parsing technique
* Reverse Polish Notation (RPN) — stack-based evaluation
* rustyline — for a polished REPL experience (history, arrow keys, line editing)

## 🌟 Why This Project? 
Most calculators hide the magic.
calc-rs reveals it. 

It’s not just a tool — it’s a teaching aid for: 

* Computer science students learning parsing
* Developers exploring Rust’s expressiveness
* Anyone curious about how math expressions are evaluated
     

And it’s built to production standards — with error recovery, clean architecture, and user-friendly UX.

## 📄 License 
MIT © ArshErgon