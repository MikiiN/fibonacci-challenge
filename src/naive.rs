use ibig::UBig;

pub fn naive_algorithm(n: u64) -> UBig {
    if n <= 1 {
        UBig::from(n)
    }
    else {
        naive_algorithm(n-1) + naive_algorithm(n-2)
    }
}

pub fn naive_algorithm_limit(n: u64) {
    let _ = naive_algorithm(n);
}