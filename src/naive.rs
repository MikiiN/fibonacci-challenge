use rug::Integer;

use super::algorithm::Algorithm;

#[derive(Clone)]
pub struct Naive {
    name: String,
}

impl Naive {
    pub fn new() -> Naive {
        Naive { 
            name: String::from("naive"),
        }
    }
}

impl Algorithm for Naive {
    fn fibonacci(&self, n: u64) -> Integer {
        if n < 2 {
        Integer::from(n)
        }
        else {
            self.fibonacci(n-1) + self.fibonacci(n-2)
        }
    }

    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}