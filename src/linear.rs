use rug::Integer;
use super::algorithm::Algorithm;

#[derive(Clone)]
pub struct Linear {
    name: String,
}

impl Linear {
    pub fn new() -> Linear {
        Linear { 
            name: String::from("linear"),
        }
    }
}

impl Algorithm for Linear {
    fn fibonacci(&self, n: u64) -> Integer {
        let mut a = Integer::from(1u32);
        let mut b = Integer::from(0u32);

        for _ in 0..n {
            let tmp = Integer::from(&a + &b);
            b = a;
            a = tmp;
        }
        b
    }

    fn fibonacci_measure(&self, n: u64) {
        let _ = self.fibonacci(n);
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}