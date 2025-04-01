use std::collections::HashSet;
use std::time::{Duration, Instant};
use rand::Rng; // Add rand = "0.8" (or later) to Cargo.toml

/// Performs a mixed workload on a HashSet:
/// - 20% insertion (new value)
/// - 20% removal (attempt to remove a random value)
/// - 60% lookup (check for existence of a random value)
fn mixed_workload_hashset(n: usize, iterations: usize, mut next_val: usize) -> (Duration, usize) {
    // Initialize HashSet with numbers 0 to n-1.
    let mut set: HashSet<usize> = (0..n).collect();
    let mut rng = rand::thread_rng();

    let start = Instant::now();
    for _ in 0..iterations {
        let op = rng.gen_range(0..100); // a value between 0 and 99
        if op < 20 {
            // 20% insertion: insert the next value.
            set.insert(next_val);
            next_val += 1;
        } else if op < 40 {
            // 20% removal: choose a random value from [0, next_val) and remove it.
            let val = rng.gen_range(0..next_val);
            set.remove(&val);
        } else {
            // 60% lookup: check if a random value exists.
            let val = rng.gen_range(0..next_val);
            let _ = set.contains(&val);
        }
    }
    let duration = start.elapsed();
    (duration, next_val)
}

/// Performs a mixed workload on a Vec:
/// - 20% insertion (push a new value)
/// - 20% removal (remove an element at a random index, if non-empty)
/// - 60% lookup (linear search using .contains())
fn mixed_workload_vec(n: usize, iterations: usize, mut next_val: usize) -> (Duration, usize) {
    // Initialize Vec with numbers 0 to n-1.
    let mut vec: Vec<usize> = (0..n).collect();
    let mut rng = rand::thread_rng();

    let start = Instant::now();
    for _ in 0..iterations {
        let op = rng.gen_range(0..100); // a value between 0 and 99
        if op < 20 {
            // 20% insertion: push the next value.
            vec.push(next_val);
            next_val += 1;
        } else if op < 40 {
            // 20% removal: if the vec isn't empty, remove an element at a random index.
            if !vec.is_empty() {
                let index = rng.gen_range(0..vec.len());
                vec.remove(index);
            }
        } else {
            // 60% lookup: check if a random value is in the vector.
            let val = rng.gen_range(0..next_val);
            let _ = vec.contains(&val);
        }
    }
    let duration = start.elapsed();
    (duration, next_val)
}

fn main() {
    let n = 10;
    let iterations = 1_0;
    // Start new values from n.
    let start_next = n;

    let (hashset_duration, next_val_hs) = mixed_workload_hashset(n, iterations, start_next);
    let (vec_duration, next_val_vec) = mixed_workload_vec(n, iterations, start_next);

    println!("Mixed workload ({} iterations) with initial n = {}", iterations, n);
    println!("HashSet final next value: {}", next_val_hs);
    println!("HashSet total time: {:?}", hashset_duration/10);
    println!("Vec     final next value: {}", next_val_vec);
    println!("Vec     total time: {:?}", vec_duration/10);
}
