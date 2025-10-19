use std::time::{Instant, Duration};

pub mod naive;

fn main() {
    let mut i: u64 = 0;
    let max_time = Duration::from_secs(1);
    loop {
        let begin = Instant::now();
        let num = naive::naive_algorithm(i);
        let duration = begin.elapsed();
        println!("{} {} {}", i, num, duration.as_nanos());
        if duration >= max_time {
            break;
        }
        i += 1;    
    }
}
