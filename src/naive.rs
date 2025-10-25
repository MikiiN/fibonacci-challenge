use rug::Integer;

pub fn naive_algorithm(n: u64) -> Integer {
    if n < 2 {
        Integer::from(n)
    }
    else {
        naive_algorithm(n-1) + naive_algorithm(n-2)
    }
}

pub fn naive_algorithm_limit(n: u64) {
    let _ = naive_algorithm(n);
}