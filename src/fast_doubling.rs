use rug::Integer;
use super::algorithm::Algorithm;

#[derive(Clone)]
pub struct FastDoubling {
    name: String,
}

impl FastDoubling {
    pub fn new() -> FastDoubling {
        FastDoubling { 
            name: String::from("Fast Doubling")
         }
    }

    fn fibonacci_inner(&self, n: u64, res0: &mut Integer, res1: &mut Integer) {
        if n == 0 {
            *res0 = Integer::from(0);
            *res1 = Integer::from(1);
            return;
        }
        let new_idx: u64 = n / 2;
        self.fibonacci_inner(new_idx, res0, res1);
        let a = (*res0).clone();
        let a2 = a.clone() * a.clone();
        let b = (*res1).clone();
        let b2 = b.clone() * b.clone();
        let c = (2 * b) - a.clone();
        let c = a * c;
        let d = a2 + b2;

        if n % 2 == 0 {
            *res0 = c;
            *res1 = d;
        }
        else {
            *res0 = d.clone();
            *res1 = c + d;
        }
    }
}

impl Algorithm for FastDoubling {
    fn fibonacci(&self, n: u64) -> Integer {
        let mut res: Integer = Integer::new();
        let mut tmp: Integer = Integer::new();
        self.fibonacci_inner(n, &mut res, &mut tmp);
        return res;
    }

    fn fibonacci_measure(&self, n: u64) {
        let _ = self.fibonacci(n);
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}