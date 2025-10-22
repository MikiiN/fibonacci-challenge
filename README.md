# Fibonacci challenge
The goal is to calculate the largest possible Fibonacci number in under one second, and I decided to try it using the Rust programming language. In this README, I will describe my journey through this challenge.

As a reminder, a Fibonacci number is defined by the following formula:
```math
F_n = \begin{cases}
    F_{n-1} + F_{n-2}, & n \geq2 \\
    1, & n = 1 \\
    0, & n = 0
\end{cases}
```

## Dependencies
You need Cargo to run this project, and if you want to visualize the measured results in a graph, you need Python (tested with version 3.13.7) and must install its dependencies using the following command:
```bash
pip install -r requirements.txt
```

## Usage
To run measurements you simply use this command:
```bash
cargo run --release
```
And if you want to display the measured data in a graph, just run:
```bash
python make_graphs.py
```

## Implemented Algorithms

| Algorithm | Complexity | Location       |
|-----------|------------|----------------|
|   naive   | $`O(2^n)`$ |`naive.rs`      |
|  linear   | $`O(n^2)`$ |`linear.rs`     |
|           |            |                |

### Naive
This algorithm is just a rewritten formula in code using recursion.

```python
def naive(n):
    if n < 2:
        return n
    return naive(n-1) + naive(n-2)
```

### Linear
The iterative algorithm remembers only the last two numbers. In each iteration, it adds them together, discards the smaller number, and stores the new one. This algorithm should have linear complexity, but because the additions involve increasingly larger numbers, it actually has quadratic complexity.  
```python
def linear(n):
    a = 1
    b = 0
    for _ in range(n):
        tmp = a + b
        b = a
        a = tmp
    return b
```