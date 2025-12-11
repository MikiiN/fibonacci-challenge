use rug::Integer;
use super::algorithm::Algorithm;

#[derive(Clone)]
pub struct ExponentialMatrix {
    name: String
}

impl ExponentialMatrix {
    pub fn new() -> ExponentialMatrix {
        ExponentialMatrix { name: String::from("Exponential Matrix") }
    }
}

impl Algorithm for ExponentialMatrix {
    fn fibonacci(&self, n: u64) -> Integer {
        let mut res_mat = SymMat2x2{
            a: Integer::from(1u32),
            b: Integer::from(1u32),
            c: Integer::from(0u32),
        };
        let mut mul_mat = SymMat2x2{
            a: Integer::from(1u32),
            b: Integer::from(1u32),
            c: Integer::from(0u32),
        };

        if n < 2 {
            return Integer::from(n);
        }

        let mut i = n-1;
        while i > 0 {
            let last_bit = i & 1;
            if last_bit == 1 {
                res_mat.mul(&mul_mat);
            }
            mul_mat.square();
            i = i >> 1;
        } 
        res_mat.b
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

#[derive(Debug)]
struct SymMat2x2 {
    a: Integer,
    b: Integer,
    c: Integer
}

impl SymMat2x2 {
    pub fn mul(&mut self, mat: &SymMat2x2) {
        let p1 = Integer::from(&self.b * &mat.b);
        let p2 = Integer::from(&self.c * &mat.c);
        let p3 = Integer::from(&self.a * &mat.b);
        let p4 = Integer::from(&self.b * &mat.c);

        self.b = p4 + p3;
        self.c = p1 + p2;
        self.a = Integer::from(&self.c + &self.b);
    }
    
    pub fn square(&mut self) {
        let p1 = Integer::from(&self.b * &self.b);
        let p2 = Integer::from(&self.c * &self.c);
        let p3 = Integer::from(&self.a * &self.b);
        let p4 = Integer::from(&self.b * &self.c);

        self.b = p4 + p3;
        self.c = p1 + p2;
        self.a = Integer::from(&self.c + &self.b);
    }
}