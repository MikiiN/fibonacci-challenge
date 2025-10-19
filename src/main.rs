use std::time::{Instant, Duration};

pub mod naive;

fn measure_universal(alg: &dyn Fn(u64) -> u64) {
    let mut i: u64 = 0;
    let max_time = Duration::from_secs(1);
    loop {
        let begin = Instant::now();
        let num = alg(i);
        let duration = begin.elapsed();
        println!("{} {} {}", i, num, duration.as_nanos());
        if duration >= max_time {
            break;
        }
        i += 1;    
    }
}

fn main() {
    measure_universal(&naive::naive_algorithm);
}
