use rug::Integer;
use super::algorithm::Algorithm;

#[derive(Clone)]
pub struct BinaryExponentiationMatrix {
    name: String
}

impl BinaryExponentiationMatrix {
    pub fn new() -> BinaryExponentiationMatrix {
        BinaryExponentiationMatrix { 
            name: String::from("Reversed Exponential Matrix") 
        }
    }
}

impl Algorithm for BinaryExponentiationMatrix {
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
        let mut i = n;

        while i > 1 {
            if i % 2 == 1 {
                mul_mat.mul(&res_mat);
                i -= 1;
            }
            res_mat.square();
            i = i >> 1;
        }
        res_mat.mul(&mul_mat);
        res_mat.c
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