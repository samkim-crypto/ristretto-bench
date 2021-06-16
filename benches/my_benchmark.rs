use criterion::{criterion_group, criterion_main, Criterion};
use ristretto_bench::{ristretto_add, ristretto_decompress, ristretto_mul};
use curve25519_dalek::{
    ristretto::{CompressedRistretto, RistrettoPoint},
    scalar::Scalar,
    traits::Identity,
};

pub fn criterion_benchmark(c: &mut Criterion) {
    // Arithmetic operations on curve25519_dalek are implemented for constant time execution, so
    // the benchmarks on these points will be representative of the other points as well.
    let gen = RistrettoPoint::default();
    let scalar = Scalar::one();
    let compressed_gen = CompressedRistretto::identity();

    c.bench_function("Ristretto Add", |b| b.iter(|| ristretto_add(gen, gen)));
    c.bench_function("Ristretto Decompress", |b| b.iter(|| ristretto_decompress(compressed_gen)));
    c.bench_function("Ristretto Mul", |b| b.iter(|| ristretto_mul(gen, scalar)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
