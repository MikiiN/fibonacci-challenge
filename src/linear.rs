use ibig::UBig;

pub fn linear_algorithm(n: u64) -> UBig {
    let mut a = UBig::from(1u32);
    let mut b = UBig::from(0u32);

    for _ in 0..n {
        let tmp = &a + &b;
        b = a;
        a = tmp;
    }
    b
}

pub fn linear_algorithm_limit(n: u64) {
    let _ = linear_algorithm(n);
}