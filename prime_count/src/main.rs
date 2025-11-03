use std::time::Instant;

fn main() {
    let limit = 5000000;

    let start = Instant::now();
    let result = count_prime(limit);
    let duration = start.elapsed();

    println!("素数の数: {}", result);
    println!("実行時間: {:?}", duration);

    let start = Instant::now();
    let result = count_prime_thread(limit);
    let duration = start.elapsed();

    println!("素数の数 (スレッド版): {}", result);
    println!("実行時間 (スレッド版): {:?}", duration);

    let start = Instant::now();
    let result = count_prime_rayon(limit);
    let duration = start.elapsed();

    println!("素数の数 (Rayon版): {}", result);
    println!("実行時間 (Rayon版): {:?}", duration);

    println!("CPUコア数: {}", num_cpus::get());
}

fn count_prime(limit: u64) -> u64 {
    (2..=limit).filter(|&num| is_prime(num)).count() as u64
}

fn count_prime_thread(limit: u64) -> u64 {
    let num_threads = 16;
    let chunk_size = limit / num_threads;
    let mut handles = vec![];

    for i in 0..num_threads {
        let start = i * chunk_size + 1;
        let end = if i == num_threads - 1 {
            limit
        } else {
            (i + 1) * chunk_size
        };

        let handle = std::thread::spawn(move || {
            (start..=end).filter(|&num| is_prime(num)).count() as u64
        });
        handles.push(handle);
    }

    let mut total_count = 0;
    for handle in handles {
        total_count += handle.join().unwrap();
    }

    total_count
}

fn count_prime_rayon(limit: u64) -> u64 {
    use rayon::prelude::*;
    (2..=limit).into_par_iter().filter(|&num| is_prime(num)).count() as u64
}

// Checks if a number is prime
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

