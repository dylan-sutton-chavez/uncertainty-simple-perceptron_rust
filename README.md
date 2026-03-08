## Perceptron

A binary perceptron implemented in Rust with production tooling: static typing, integrated CI, and reproducible testing.

## Why Rust

Previous iterations of this model were written in Python. This implementation explores the same algorithm with a systems-level language, gaining:

- Compiled, no runtime overhead
- Weight/input mismatches caught at compile time
- Memory safety without a garbage collector
- CI/CD, integration tests, and reproducible builds out of the box

## Model

The perceptron computes a weighted sum of inputs plus a bias, then applies a step function to produce a binary label.

$$
f(x) = sum(w_i * x_i) + b
$$

## Structure

```
src/
├── main.rs
└── model.rs
tests/
├── integration-tests.rs
└── cases.json
.github/
└── workflows/
    └── ci.yml
Cargo.toml
```

## Usage

```bash
cargo run # Inferenece.
cargo test # Run integration tests.
```

## Requirements

```txt
cargo
```
