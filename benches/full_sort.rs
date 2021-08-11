use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use nanorand::{Rng, WyRand};
use rdst::RadixSort;
use std::time::Duration;
use voracious_radix_sort::RadixSort as Vor;

fn full_sort_u32(c: &mut Criterion) {
    let n = 500_000_000;
    let mut inputs = Vec::with_capacity(n);
    let mut rng = WyRand::new();

    for _ in 0..n {
        inputs.push(rng.generate::<u32>());
    }

    let input_sets: Vec<Vec<u32>> = vec![
        inputs.clone(),
        inputs[..200_000_000].to_vec(),
        inputs[..100_000_000].to_vec(),
        inputs[..50_000_000].to_vec(),
        inputs[..10_000_000].to_vec(),
        inputs[..5_000_000].to_vec(),
        inputs[..2_000_000].to_vec(),
        inputs[..1_000_000].to_vec(),
        inputs[..500_000].to_vec(),
        inputs[..300_000].to_vec(),
        inputs[..200_000].to_vec(),
        inputs[..100_000].to_vec(),
        inputs[..50_000].to_vec(),
        inputs[..10_000].to_vec(),
        inputs[..5_000].to_vec(),
    ];

    drop(inputs);

    let mut group = c.benchmark_group("full_sort_u32");
    group.sample_size(30);
    group.measurement_time(Duration::from_secs(10));
    for set in input_sets.iter() {
        let l = set.len();
        group.throughput(Throughput::Elements(l as u64));
        group.bench_with_input(BenchmarkId::new("rdst", l), set, |bench, set| {
            bench.iter(|| {
                let mut input = set.clone();
                input.radix_sort_unstable();
                black_box(input);
            });
        });

        group.bench_with_input(BenchmarkId::new("voracious", l), set, |bench, set| {
            bench.iter(|| {
                let mut input = set.clone();
                input.voracious_mt_sort(num_cpus::get());
                black_box(input);
            });
        });
    }
    group.finish();
}

fn full_sort_u64(c: &mut Criterion) {
    let n = 200_000_000;
    let mut inputs = Vec::with_capacity(n);
    let mut rng = WyRand::new();

    for _ in 0..n {
        inputs.push(rng.generate::<u64>());
    }

    let input_sets: Vec<Vec<u64>> = vec![
        inputs.clone(),
        inputs[..100_000_000].to_vec(),
        inputs[..50_000_000].to_vec(),
        inputs[..10_000_000].to_vec(),
        inputs[..5_000_000].to_vec(),
        inputs[..2_000_000].to_vec(),
        inputs[..1_000_000].to_vec(),
        inputs[..500_000].to_vec(),
        inputs[..300_000].to_vec(),
        inputs[..200_000].to_vec(),
        inputs[..100_000].to_vec(),
        inputs[..50_000].to_vec(),
        inputs[..10_000].to_vec(),
        inputs[..5_000].to_vec(),
    ];

    drop(inputs);

    let mut group = c.benchmark_group("full_sort_u64");
    group.sample_size(30);
    group.measurement_time(Duration::from_secs(10));
    for set in input_sets.iter() {
        let l = set.len();
        group.throughput(Throughput::Elements(l as u64));
        group.bench_with_input(BenchmarkId::new("rdst", l), set, |bench, set| {
            bench.iter(|| {
                let mut input = set.clone();
                input.radix_sort_unstable();
                black_box(input);
            });
        });

        group.bench_with_input(BenchmarkId::new("voracious", l), set, |bench, set| {
            bench.iter(|| {
                let mut input = set.clone();
                input.voracious_mt_sort(num_cpus::get());
                black_box(input);
            });
        });
    }
    group.finish();
}

criterion_group!(benches, full_sort_u32, full_sort_u64);
criterion_main!(benches);
