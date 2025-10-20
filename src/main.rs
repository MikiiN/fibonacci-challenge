use std::fs;
use std::io::{Write, Result};
use std::time::{Instant, Duration};

pub mod naive;

fn measure_universal(name: &str, alg: &dyn Fn(u64) -> u64) -> Result<()> {
    let mut i: u64 = 0;
    let max_time = Duration::from_secs(1);
    fs::create_dir_all("data/")?;
    let mut file =  fs::File::create(format!("data/{}.out", name))?; 
    loop {
        let begin = Instant::now();
        let num = alg(i);
        let duration = begin.elapsed();
        let line = format!("{} {} {}", i, num, duration.as_nanos());
        writeln!(file, "{}", line)?;
        if duration >= max_time {
            break;
        }
        i += 1;    
    }
    Ok(())
}

fn main() -> Result<()> {
    let algorithms = vec![
        ("naive", &naive::naive_algorithm)
    ];
    // measure_universal("naive", &naive::naive_algorithm)?;

    for alg in algorithms {
        measure_universal(alg.0, alg.1)?;
    }
    Ok(())
}
