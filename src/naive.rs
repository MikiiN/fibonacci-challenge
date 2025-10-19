pub fn naive_algorithm(n: u64) -> u64 {
    if n <= 1 {
        n
    }
    else {
        naive_algorithm(n-1) + naive_algorithm(n-2)
    }
}