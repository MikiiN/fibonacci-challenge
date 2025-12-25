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

| Algorithm             | Time Complexity | Location       |
|-----------------------|------------|----------------|
|   naive               | $`O(2^n)`$ |`naive.rs`      |
|  linear               | $`O(n^2)`$ |`linear.rs`     |
| matrix exponentiation | $`O(n^2)`$ |`mat_exp.rs`    |
| reduced matrix exp.   | $`O(n^2)`$ |`r_mat_exp.rs`    |
| binary exponentiation | $`O(log(n))`$ |`binary_exp.rs`    |
| fast doubling         | $`O(log(n))`$ | `fast_doubling.rs` |
<!-- |                       |            |                | -->

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

### Matrix exponentiation
This algorithm is based on the following identity:
```math
\begin{bmatrix}
    1 & 1 \\ 1 & 0
\end{bmatrix}^n =
\begin{bmatrix}
    F_{n+1} & F_n \\ F_n & F_{n-1}
\end{bmatrix}
```
Because the matrices involved are always symmetric, the calculation can be simplified by using only three elements of each matrix.
Starting from the formula:

```math 
\begin{bmatrix} a & b \\ b & c \end{bmatrix} * \begin{bmatrix} d & e \\ e & f \end{bmatrix} = \begin{bmatrix} ad+be & ae+bf \\ ae+bf & be+cf \end{bmatrix}
```

we can represent each symmetric matrix using only three components:
```math
\begin{bmatrix}
    a \\ b \\ c
\end{bmatrix}
```

and rewrite the multiplication as a special operation between these 3-element vectors:

 ```math
\begin{bmatrix}
    a \\ b \\ c
\end{bmatrix} \cdot
\begin{bmatrix}
    d \\ e \\ f
\end{bmatrix}  =
\begin{bmatrix}
    ad+be \\ ae+bf \\ be+cf
\end{bmatrix}
```
Note: \
The operation $`\cdot`$ here does not represent ordinary scalar or matrix multiplication.
It is a custom-defined operation that mimics the effect of multiplying two symmetric 2×2 matrices in this specific use case.

In the end, the algorithm performs iterative matrix multiplications between the result matrix and the base matrix depending on current bit in the given index.

```python
def matrix_exponentiation(n):
    result = Matrix()
    base = Matrix()
    if n < 2:
        return n
    while n > 0:
        if (n & 1) == 1:
            result = result * base
        base = base * base
    return result[0][1]
```


### Reduced matrix exponentiation
Because of the relation between numbers in vector we can reduce it's size to only a two elements.
```math
\begin{bmatrix}
    F_{n} \\ F_{n+1}
\end{bmatrix}
```
With this adjustment the algorithm stay the same, but we need to modify operations for a matrix multiplication and a square. First we calculate intermediate results which will be used to get results.

#### Multiplication:
```math
\begin{bmatrix}
    a_0 \\ b_0
\end{bmatrix}
\cdot
\begin{bmatrix}
    a_1 \\ b_1
\end{bmatrix}
=
\begin{bmatrix}
    p_1 + p_2 \\ p_2 + p_3
\end{bmatrix}
```
where
```math
p_1 = a_0 \cdot a_1 \\
p_2 = b_0 \cdot b_0 \\
p_2 = a_0 \cdot b_1 + b_0 \cdot a_1
```
#### Square
```math
\begin{bmatrix}
    a \\ b
\end{bmatrix}
^2
=
\begin{bmatrix}
    p_1 + p_2 \\ p_2 + p_4
\end{bmatrix}
```
where
```math
p_1 = a \cdot a \\
p_2 = b \cdot b \\
p_3 = a \cdot b \\
p_4 = p_3 + p_3
```

### Binary exponentiation
This algorithm is still a variant of a Matrix exponentiation, but instead of using naive multiplication we split the work using the binary representation of exponent. This will result to using only $`O(log(n))`$ multiplications. 
```python
def binary_exponentiation(n):
    result = Matrix()
    base = Matrix()
    if n < 2:
        return n
    while n > 1:
        if (n % 2) == 1:
            base = base * result
            n -= 1
        result = result * result
        n = n >> 1
    result = result * result
    return result[0][2]
```


### Fast doubling
The Fast doubling use information that just by using $`F_n`$ and $`F_{n+1}`$ we can calculate $`F_{2n}`$. For this it use these two formulas:
```math
F_{2n} = F_n \cdot (2F_{n+1} - F_n) \\
F_{2n+1} = F_{n+1}^2 + F_n^2
```
This allow us to double position in sequence by smaller number of multiplications.

```python
def fast_doubling(n)
    if n == 0:
        return 0, 1
    new_n = n / 2 
    a, b = fast_doubling(new_n)
    a2 = a * a
    b2 = b * b
    c = 2*b - a
    c = a * c
    d = a2 + b2

    if n % 2 == 0:
        return c, d
    else:
        return d, c+d 
```