use criterion::{criterion_group, criterion_main, Criterion};
use exec_target_a::exec_target;

const TARGET_EXE_PATH: &'static str = "target/release/cmp_lapp-one";

#[inline(never)]
fn process_one() {
    let oup = exec_target(
        TARGET_EXE_PATH,
        &["-d", "-v", "2", "-s", "123", "inp", "oup"],
    );
    assert!(oup.status.success())
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("cmp_lapp::", |b| b.iter(|| process_one()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
