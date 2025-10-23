use ibig::UBig;

pub fn matrix_exp_algorithm(n: u64) -> UBig {
    let mut res_mat = SymMat2x2{
        a: UBig::from(1u32),
        b: UBig::from(1u32),
        c: UBig::from(0u32),
    };
    let mul_mat = SymMat2x2{
        a: UBig::from(1u32),
        b: UBig::from(1u32),
        c: UBig::from(0u32),
    };

    if n < 2 {
        return UBig::from(n);
    }

    for _ in 0..n {
        res_mat.mul(&mul_mat);
    }
    res_mat.b
}

pub fn matrix_exp_algorithm_limit(n: u64) {
    let _ = matrix_exp_algorithm(n);
}

struct SymMat2x2 {
    a: UBig,
    b: UBig,
    c: UBig
}

impl SymMat2x2 {
    pub fn mul(&mut self, mat: &SymMat2x2) {
        let p1 = &self.a * &mat.a;
        let p2 = &self.b * &mat.b;
        let p3 = &self.c * &mat.c;
        let p4 = (&self.a + &self.b) + (&mat.a + &mat.b);
        let p5 = (&self.b + &self.c) + (&mat.b + &mat.c);
        let p6 = (&self.a + &self.c) + (&mat.a + &mat.c);

        self.a = &p1 + &p2;
        self.c = &p2 + &p3;
        self.b = (&p4 + &p5 - &p6)/2;
    }
}