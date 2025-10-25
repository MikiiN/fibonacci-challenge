use rug::Integer;

pub fn matrix_exp_algorithm(n: u64) -> Integer {
    let mut res_mat = SymMat2x2{
        a: Integer::from(1u32),
        b: Integer::from(1u32),
        c: Integer::from(0u32),
    };
    let mul_mat = SymMat2x2{
        a: Integer::from(1u32),
        b: Integer::from(1u32),
        c: Integer::from(0u32),
    };

    if n < 2 {
        return Integer::from(n);
    }

    for _ in 0..(n-1) {
        res_mat.mul(&mul_mat);
    }
    res_mat.b
}

pub fn matrix_exp_algorithm_limit(n: u64) {
    let _ = matrix_exp_algorithm(n);
}

#[derive(Debug)]
struct SymMat2x2 {
    a: Integer,
    b: Integer,
    c: Integer
}

impl SymMat2x2 {
    pub fn mul(&mut self, mat: &SymMat2x2) {
        let p1 = Integer::from(&self.a * &mat.a);
        let p2 = Integer::from(&self.b * &mat.b);
        let p3 = Integer::from(&self.c * &mat.c);
        let p4 = Integer::from(&self.a * &mat.b);
        let p5 = Integer::from(&self.b * &mat.c);

        self.a = Integer::from(&p1 + &p2);
        self.b = Integer::from(&p5 + &p4);
        self.c = Integer::from(&p2 + &p3);
    }
}