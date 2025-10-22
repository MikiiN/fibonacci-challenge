# Fibonacci challenge
The goal is to calculate the biggest possible Fibonacci number under one second and I decided to try it using Rust language. In this README will be described my journey through this challenge.

For reminder Fibonacci number is defined by this formula:
```math
F_n = \begin{cases}
    F_{n-1} + F_{n-2}, & n \geq2 \\
    1, & n = 1 \\
    0, & n = 0
\end{cases}
```

## Dependencies
You need **cargo** to run this project and if you want visualize measured results in graph you need to have **python** (tested on version 3.13.7) and install it's dependencies using this command:
```bash
pip install -r requirements.txt
```

## Usage
To run measurements you simply use this command:
```bash
cargo run --release
```
And if you want to display measured data in graph you just run:
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
This algorithm is just rewritten formula in code using recursion.

```python
def naive(n):
    if n < 2:
        return n
    return naive(n-1) + naive(n-2)
```

### Linear
The iterative algorithm that just remember last two numbers, in every iteration process addition over them, forget smaller number and store new number. This algorithm should have linear complexity, but because of the addition over bigger and bigger numbers it has quadratic complexity.  
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