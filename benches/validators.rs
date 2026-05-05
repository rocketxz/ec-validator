use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_cedula_valid(c: &mut Criterion) {
    c.bench_function("cedula_valid", |b| {
        b.iter(|| {
            let _ = ec_validator::cedula::validate(black_box("1713175071"));
        });
    });
}

fn bench_cedula_invalid(c: &mut Criterion) {
    c.bench_function("cedula_invalid", |b| {
        b.iter(|| {
            let _ = ec_validator::cedula::validate(black_box("0000000000"));
        });
    });
}

fn bench_ruc_natural(c: &mut Criterion) {
    c.bench_function("ruc_natural", |b| {
        b.iter(|| {
            let _ = ec_validator::ruc::validate(black_box("1713175071001"));
        });
    });
}

fn bench_ruc_juridical(c: &mut Criterion) {
    c.bench_function("ruc_juridical", |b| {
        b.iter(|| {
            let _ = ec_validator::ruc::validate(black_box("1790085783001"));
        });
    });
}

fn bench_iban_valid(c: &mut Criterion) {
    c.bench_function("iban_valid", |b| {
        b.iter(|| {
            let _ = ec_validator::iban::validate(black_box("EC8912345678901234567890"));
        });
    });
}

criterion_group!(
    benches,
    bench_cedula_valid,
    bench_cedula_invalid,
    bench_ruc_natural,
    bench_ruc_juridical,
    bench_iban_valid
);
criterion_main!(benches);