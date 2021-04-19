use criterion::{criterion_group, criterion_main, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

fn process_one(env_args: &[&str]) -> anyhow::Result<cmp_null_void::curl::CmdOptConf> {
    cmp_null_void::curl::parse_cmdopts("prog", env_args)
}

fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
    #[rustfmt::skip]
    let env_args = vec![
        "-a", "--connect-timeout", "50", "--ftp-pasv", "--http2",
        "--max-time", "100", "--no-alpn", "-N",
        "--socks5-gssapi-service", "name1", "-y", "1000", "--sslv3",
        "http://url1.com"];
    let env_args = &env_args;
    //
    let result_conf = cmp_null_void::curl::CmdOptConf {
        opt_program: "prog".to_string(),
        ..Default::default()
    };
    match process_one(criterion::black_box(env_args)) {
        Ok(conf) => {
            assert_eq!(conf, result_conf);
        }
        Err(err) => {
            eprintln!("{}", err);
            unreachable!();
        }
    }
    //
    c.bench_function("cmp_null_void::curl::", |b| {
        b.iter(|| {
            let _r = process_one(criterion::black_box(env_args));
        })
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = criterion_benchmark);
criterion_main!(benches);
