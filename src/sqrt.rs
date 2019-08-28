use std::f64;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let count: i64 = 1_000_000_000;
    let sq:f64 = do_sqrt(count);

    let end = start.elapsed();
    println!("{}",sq);
    println!("実行: {}.{}sec", end.as_secs(),end.subsec_nanos() / 1_000_000);

    // Cargo run by MacbookPro i7
    // 21081851051977.78
    // 実行: 115.789sec

    // Build最適化
    // rustrc -O src/main.rs by MacbookPro i7
    // 21081851051977.78
    // 実行: 8.436sec
}

fn do_sqrt(count:i64) -> f64 {
    let mut num: f64 = 0.0;
    for n in 0..count {
        let x: f64 = n as f64;
        num += x.sqrt();
    }
    return num
}