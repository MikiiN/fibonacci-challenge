use std::cmp;
use std::sync::Arc;
use std::fs;
use std::io::{Write, Result};
use std::time::{Instant, Duration};
use std::sync::mpsc;
use std::thread;

use rug::Integer;

pub mod naive;
pub mod linear;
pub mod mat_exp;
pub mod r_mat_exp;
pub mod reversed_mat_exp;

const MAX_TIME_IN_SEC: u64 = 1;
const NUMB_OF_POINTS: u64 = 200;

fn find_limit(alg: Arc<dyn Fn(u64) + Send + Sync + 'static>) -> u64 {
    let mut last_idx = 0u64;
    let mut current_idx = 0u64;
    let mut step = 1u64;
    let limit = Duration::from_millis(MAX_TIME_IN_SEC*1000+10);

    let mut first_fail_flag = false;
    loop {
        println!("idx: {}", current_idx);
        let (tx, rx) = mpsc::channel();
        let f = Arc::clone(&alg);
        thread::spawn(move || {
            f(current_idx);
            let _ = tx.send(current_idx);
        });

        match rx.recv_timeout(limit) {
            Ok(_) => {
                last_idx = current_idx;
                if !first_fail_flag {
                    step = step << 1;
                }
            }
            Err(_) => {
                if !first_fail_flag {
                    step = step >> 1;
                }
                first_fail_flag = true;
                step = step >> 1;
                current_idx = last_idx;
            }
        }
        if step == 0 {
            return current_idx;
        }
        current_idx += step;
    }
}

fn measure_universal(name: &str, alg: &dyn Fn(u64) -> Integer, max_idx: u64) -> Result<()> {
    let limit = Duration::from_secs(MAX_TIME_IN_SEC);
    fs::create_dir_all("data/")?;
    let mut file =  fs::File::create(format!("data/{}.out", name))?; 

    let step = cmp::max(1, max_idx / NUMB_OF_POINTS);

    let mut i = 0u64; 
    loop {
        let begin = Instant::now();
        let _ = alg(i);
        let duration = begin.elapsed();
        if duration >= limit {
            break;
        }
        else {
            let line = format!("{} {} {}", i, 0, duration.as_nanos());
            writeln!(file, "{}", line)?;
        }
        if i == max_idx {
            break;
        }
        i = cmp::min(i+step, max_idx);
    
    }
    Ok(())
}

fn main() -> Result<()> {
    // for n in 0..100 {
    //     let res = r_mat_exp::reduced_matrix_exp_algorithm(n);
    //     let corr = linear::linear_algorithm(n);
    //     if res != corr {
    //         println!("Mistake");
    //         break;
    //     }
    //     println!("{}: {} {}", n, res, corr);
    // }
    
    let f = Arc::new(|x| naive::naive_algorithm_limit(x));
    let lim_naive = find_limit(f);
    println!("limit: {}", lim_naive);
    
    let f = Arc::new(|x| linear::linear_algorithm_limit(x));
    let lim_linear = find_limit(f);
    println!("limit: {}", lim_linear);

    let f = Arc::new(|x| mat_exp::matrix_exp_algorithm_limit(x));
    let lim_exp_mat = find_limit(f);
    println!("limit: {}", lim_exp_mat);
    
    let f = Arc::new(|x| r_mat_exp::reduced_matrix_exp_algorithm_limit(x));
    let lim_r_exp_mat = find_limit(f);
    println!("limit: {}", lim_r_exp_mat);

    let f = Arc::new(|x| reversed_mat_exp::reversed_matrix_exp_algorithm_limit(x));
    let lim_reversed_exp_mat = find_limit(f);
    println!("limit: {}", lim_reversed_exp_mat);

    let algorithms = vec![
        ("naive", lim_naive, &(naive::naive_algorithm as fn(u64) -> Integer)),
        ("linear", lim_linear, &(linear::linear_algorithm as fn(u64) -> Integer)),
        ("exp_matrix", lim_exp_mat, &(mat_exp::matrix_exp_algorithm as fn(u64) -> Integer)),
        ("reduced_exp_matrix", lim_exp_mat, &(r_mat_exp::reduced_matrix_exp_algorithm as fn(u64) -> Integer)),
        ("reversed_exp_matrix", lim_reversed_exp_mat, &(reversed_mat_exp::reversed_matrix_exp_algorithm as fn(u64) -> Integer)),
        ];

    for alg in algorithms {
        measure_universal(alg.0, alg.2, alg.1)?;
    }
    Ok(())
}
