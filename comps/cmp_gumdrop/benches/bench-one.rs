use criterion::{criterion_group, criterion_main, Criterion};

#[inline(never)]
fn process_one() {
    let v = vec!["-d", "-v", "2", "-s", "123", "inp", "oup"];
    let _r = cmp_gumdrop::one::parse_cmdopts("prog", v);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cmp_gumdrop::one::", |b| b.iter(|| process_one()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
