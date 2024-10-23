use divan::{black_box, Bencher};
#[allow(deprecated)]
use hyperion_stats::nd::NdArrayStats;
use hyperion_stats::ParallelStats;
use rand::Rng;

fn main() {
    divan::main();
}

fn generate_test_data(width: usize, updates: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    (0..updates)
        .map(|_| (0..width).map(|_| rng.r#gen()).collect())
        .collect()
}

#[divan::bench(args = [
    4, 8, 16, 32, 64
])]
fn bench_parallel_stats(bencher: Bencher<'_, '_>, width: usize) {
    let updates = 1000;
    let test_data = generate_test_data(width, updates);

    bencher.bench(move || {
        let mut stats = ParallelStats::new(width);
        for values in &test_data {
            stats.update(black_box(values));
        }
        stats
    });
}

#[divan::bench(args = [
    4, 8, 16, 32, 64
])]
fn bench_ndarray_stats(bencher: Bencher<'_, '_>, width: usize) {
    let updates = 1000;
    let test_data = generate_test_data(width, updates);

    bencher.bench(move || {
        let mut stats = NdArrayStats::new(width);
        for values in &test_data {
            stats.update(black_box(values));
        }
        stats
    });
}
