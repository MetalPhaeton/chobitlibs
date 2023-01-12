use criterion::{criterion_group, criterion_main, Criterion};
use chobitlibs::chobit_complex::*;

fn chobit_complex_cos_bench(c: &mut Criterion) {
    let cis_table = CisTable::new();

    let mut total: f32 = 0.0;
    c.bench_function("chobit_complex_cos_bench", |b| b.iter(|| {
        for angle in 0..Complex::full_circle_angle() {
            let radian = Complex::angle_to_radian(angle);
            let a = cis_table[angle];

            total += radian + a.re + a.im;
        }
    }));

    println!("{}", total);
}

fn rust_cos_bench(c: &mut Criterion) {
    let mut total: f32 = 0.0;
    c.bench_function("rust_cos_bench", |b| b.iter(|| {
        for angle in 0..Complex::full_circle_angle() {
            let radian = Complex::angle_to_radian(angle);
            let a = radian.cos() + radian.sin();

            total += radian + a;
        }
    }));

    println!("{}", total);
}

criterion_group!(
    chobit_complex_benches,
    chobit_complex_cos_bench,
    rust_cos_bench
);
criterion_main!(chobit_complex_benches);
