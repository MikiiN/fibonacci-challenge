use std::cmp;
use std::fs;
use std::io::{Write, Result};
use std::process::exit;
use std::time::{Instant, Duration};
use std::sync::mpsc;
use std::thread;

use crate::algorithm::Algorithm;

pub mod algorithm;

pub mod naive;
pub mod linear;
pub mod mat_exp;
pub mod r_mat_exp;
pub mod reversed_mat_exp;
pub mod fast_doubling;


const MAX_TIME_IN_SEC: u64 = 1;
const NUMB_OF_POINTS: u64 = 200;


fn find_limit<T>(alg: &T, print_indexes: bool) -> u64
where 
    T: Algorithm + Clone + Send + 'static
{
    let mut last_idx = 0u64;
    let mut current_idx = 0u64;
    let mut step = 1u64;
    let limit = Duration::from_millis(MAX_TIME_IN_SEC*1000+10);

    let mut first_fail_flag = false;
    loop {
        if print_indexes {
            println!("idx: {}", current_idx);
        }
        let (tx, rx) = mpsc::channel();
        let alg_clone = alg.clone();
        thread::spawn(move || {
            alg_clone.fibonacci_measure(current_idx);
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

fn measure_universal<T>(alg: T, print_indexes: bool) -> Result<()>  
where 
    T: Algorithm + Clone + Send + 'static
{
    let limit = Duration::from_secs(MAX_TIME_IN_SEC);
    let max_idx = find_limit(&alg, print_indexes);
    println!("limit for {}: {}", alg.get_name(), max_idx);

    fs::create_dir_all("data/")?;
    let name = alg.get_name();
    let mut file =  fs::File::create(format!("data/{}.out", name))?; 

    let step = cmp::max(1, max_idx / NUMB_OF_POINTS);

    let mut i = 0u64; 
    loop {
        let begin = Instant::now();
        let _ = alg.fibonacci(i);
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

fn main() {
    // code for validity testing
    // let alg_corr = linear::Linear::new();
    // let alg_test = fast_doubling::FastDoubling::new();
    // for n in 0..100 {
    //     let res = alg_test.fibonacci(n);
    //     let corr = alg_corr.fibonacci(n);
    //     println!("{}: {} {}", n, res, corr);
    //     if res != corr {
    //         println!("Mistake");
    //         break;
    //     }
    // }
    // exit(0);
    
    let alg_naive = naive::Naive::new();
    let err = measure_universal(alg_naive, false);
    match err {
        Ok(v) => v,
        Err(_) => {println!("Error occurred"); exit(1)},
    }

    let alg_linear = linear::Linear::new();
    let _ = measure_universal(alg_linear, false);
    match err {
        Ok(v) => v,
        Err(_) => {println!("Error occurred"); exit(1)},
    }

    let alg_exp_mat = mat_exp::ExponentialMatrix::new();
    let err = measure_universal(alg_exp_mat, false);
    match err {
        Ok(v) => v,
        Err(_) => {println!("Error occurred"); exit(1)},
    }

    let alg_reduced_exp_mat = r_mat_exp::ReducedExponentialMatrix::new();
    let err = measure_universal(alg_reduced_exp_mat, false);
    match err {
        Ok(v) => v,
        Err(_) => {println!("Error occurred"); exit(1)},
    }

    let alg_reversed_exp_mat = reversed_mat_exp::ReversedExponentialMatrix::new();
    let err = measure_universal(alg_reversed_exp_mat, false);
    match err {
        Ok(v) => v,
        Err(_) => {println!("Error occurred"); exit(1)},
    }

    let alg_fast_doubling = fast_doubling::FastDoubling::new();
    let err = measure_universal(alg_fast_doubling, false);
    match err {
        Ok(v) => v,
        Err(_) => {println!("Error occurred"); exit(1)},
    }
}
