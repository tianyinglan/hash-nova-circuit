use criterion::{
    criterion_group, criterion_main, measurement::Measurement, BenchmarkGroup,
    Criterion,
};
use hash_chain::Prover;
use std::time::Duration;

const TEST_INIT_VALUE: u64 = 100;

pub fn bench_prover(c: &mut Criterion) {
    let mut group = c.benchmark_group("Prover");
    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(600));

    bench_with_steps(&mut group, 4);
    bench_with_steps(&mut group, 8);
    bench_with_steps(&mut group, 16);

    group.finish();
}

fn bench_with_steps<M: Measurement>(
    group: &mut BenchmarkGroup<M>,
    step_num: usize,
) {
    let prover = Prover::new(TEST_INIT_VALUE, step_num);

    group.bench_function(format!("Prover with {step_num} steps"), |b| {
        b.iter(|| prover.prove().expect("Failed to prove"));
    });
}

criterion_group!(benches, bench_prover);
criterion_main!(benches);
