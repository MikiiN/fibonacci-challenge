use rug::Integer;

pub fn reduced_matrix_exp_algorithm(n: u64) -> Integer {
    let mut res_mat = SymMat2x2{
        a: Integer::from(0u32),
        b: Integer::from(1u32),
    };
    let mul_mat = SymMat2x2{
        a: Integer::from(0u32),
        b: Integer::from(1u32),
    };

    if n < 2 {
        return Integer::from(n);
    }

    for _ in 0..(n-2) {
        res_mat.mul(&mul_mat);
    }
    res_mat.a + res_mat.b
}

pub fn reduced_matrix_exp_algorithm_limit(n: u64) {
    let _ = reduced_matrix_exp_algorithm(n);
}

#[derive(Debug)]
struct SymMat2x2 {
    a: Integer,
    b: Integer,
}

impl SymMat2x2 {
    pub fn mul(&mut self, mat: &SymMat2x2) {
        let p1 = Integer::from(&self.a * &mat.a);
        let p2 = Integer::from(&self.b * &mat.b);
        let p3 = Integer::from(&self.b * &mat.a) + Integer::from(&self.a * &mat.b);

        self.a = p1 + Integer::from(&p2);
        self.b = Integer::from(&p2) + p3;
    }
}