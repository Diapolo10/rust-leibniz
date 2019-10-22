extern crate crossbeam;

use std::thread;
use std::vec::Vec;
use threadpool::ThreadPool;


fn multicore_leibniz(n: u64) -> f64 {
    let mut result: f64 = 4.0;
    let mut cpu_count = num_cpus::get() as u64;
    let thread_results = Vec::<f64>::new();

    if cpu_count == 0 {
        cpu_count = 2;
    }
    
    let pool = ThreadPool::new(cpu_count as usize);

    let threads: Vec::<_> = (1u64..(cpu_count+1 as u64))
        .map(|thread_num| {

            let mut t_queue = thread_results.clone();
            let start = (n / cpu_count) * thread_num+1;
            
            let stop = if cpu_count - 1 == thread_num {
                n
            }
            else {
                (n / cpu_count) * (thread_num+1)
            };

            thread::spawn(move || {
            
                let mut sum: f64 = 0.0;

                for num in (start..stop).step_by(4) {
                    sum += -(4 as f64/num as f64) + 4 as f64/(num+2) as f64;
                }

                t_queue.push(sum);

            })
        })
    .collect();

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Result length: {}", thread_results.len());

    result += thread_results.iter().sum::<f64>();

    
    result
}



fn leibniz(n: u64) -> f64 {
    let mut result: f64 = 4.0;

    for num in (3..n).step_by(4) {
        result += -(4 as f64/num as f64) + 4 as f64/(num+2) as f64;
    }

    if n%2==0 {
        result += 4 as f64 / n as f64;
    }

    result
}

fn main() {
    println!("Leibniz-sequence Pi-calculator");
    let limit = 1_000_000;
    println!("Using {} as limit, pi = {:.16}", limit, leibniz(limit));
    println!("Using {} as limit, pi = {:.16}", limit, multicore_leibniz(limit));
}
