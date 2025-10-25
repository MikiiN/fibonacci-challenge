use rug::Integer;

pub fn linear_algorithm(n: u64) -> Integer {
    let mut a = Integer::from(1u32);
    let mut b = Integer::from(0u32);

    for _ in 0..n {
        let tmp = Integer::from(&a + &b);
        b = a;
        a = tmp;
    }
    b
}

pub fn linear_algorithm_limit(n: u64) {
    let _ = linear_algorithm(n);
}