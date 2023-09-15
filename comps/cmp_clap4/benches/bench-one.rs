use criterion::{criterion_group, criterion_main, Criterion};

#[inline(never)]
fn process_one() {
    let v = vec!["prog", "-d", "-vv", "-s", "123", "inp", "oup"];
    let _r = cmp_clap4::one::parse_cmdopts("prog", v);
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cmp_clap4::one::", |b| b.iter(|| process_one()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
