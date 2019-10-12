
use std::thread;
use threadpool::ThreadPool;

fn multicore_leibniz(n: u64) -> f64 {
    let mut result: f64 = 4.0
    let cpu_count = num_cpus::get();

    if cpu_count == 0 {
        let cpu_count = 2;
    }
    
    let pool = ThreadPool::new(cpu_count);

    

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
}
