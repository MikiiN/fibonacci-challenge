// use rug::Integer;

// pub fn reversed_matrix_exp_algorithm(n: u64) -> Integer {
//     let mut res_mat = SymMat2x2{
//         a: Integer::from(0u32),
//         b: Integer::from(1u32),
//     };
//     let mul_mat = SymMat2x2{
//         a: Integer::from(0u32),
//         b: Integer::from(1u32),
//     };

//     if n < 2 {
//         return Integer::from(n);
//     }

//     let mut len = bit_len(n)-1;

//     loop {
//         let bit = n & (1 << len);
        
//         println!("Before {:?}", res_mat);
//         res_mat.square();
//         if bit == 1 {
//             res_mat.mul(&mul_mat);
//         }
        
//         println!("After {:?}", res_mat);
//         if len == 0 {
//             break;
//         }
//         len -= 1;
//     }
//     res_mat.a + res_mat.b
// }

// pub fn reversed_matrix_exp_algorithm_limit(n: u64) {
//     let _ = reversed_matrix_exp_algorithm(n);
// }

// fn bit_len(n: u64) -> u64 {
//     let mut i = 0;
//     while n >= (1 << i) {
//         i += 1;
//     }
//     i
// }

// #[derive(Debug)]
// struct SymMat2x2 {
//     a: Integer,
//     b: Integer,
// }

// impl SymMat2x2 {
//     pub fn mul(&mut self, mat: &SymMat2x2) {
//         let p1 = Integer::from(&self.a * &mat.a);
//         let p2 = Integer::from(&self.b * &mat.b);
//         let p3 = Integer::from(&self.a * &mat.b) + Integer::from(&self.b * &mat.a);

//         self.a = Integer::from(&p1 + &p2);
//         self.b = Integer::from(&p2 + &p3);
//     }

//     pub fn square(&mut self) {
//         let p1 = Integer::from(&self.a * &self.a);
//         let p2 = Integer::from(&self.b * &self.b);
//         let p3 = Integer::from(&self.a * &self.b);
//         let p4 = Integer::from(&p3 + &p3);

//         self.a = Integer::from(&p1 + &p2);
//         self.b = Integer::from(&p2 + &p4);
//     }
// }

use rug::Integer;

pub fn reversed_matrix_exp_algorithm(n: u64) -> Integer {
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
    res_mat.b
}

pub fn reversed_matrix_exp_algorithm_limit(n: u64) {
    let _ = reversed_matrix_exp_algorithm(n);
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