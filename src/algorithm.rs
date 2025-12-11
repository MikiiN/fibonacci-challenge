use rug::Integer;

pub trait Algorithm: {
    fn fibonacci(&self, n: u64) -> Integer;
    fn get_name(&self) -> &str;

    fn fibonacci_measure(&self, n: u64) {
        let _ = self.fibonacci(n);
    }
}