use criterion::{criterion_group, criterion_main, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

fn process_one(env_args: &[&str]) -> anyhow::Result<cmp_structopt::curl::CmdOptConf> {
    cmp_structopt::curl::parse_cmdopts("prog", env_args.to_vec())
}

fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
    #[rustfmt::skip]
    let env_args = vec!["prog",
        "-a", "--connect-timeout", "50", "--ftp-pasv", "--http2",
        "--max-time", "100", "--no-alpn", "-N",
        "--socks5-gssapi-service", "name1", "-y", "1000", "--sslv3",
        "http://url1.com"];
    let env_args = &env_args;
    //
    let result_conf = cmp_structopt::curl::CmdOptConf {
        opt_program: "prog".to_string(),
        flg_append: true,
        opt_connect_timeout: 50,
        flg_ftp_pasv: true,
        flg_http2: true,
        opt_max_time: 100,
        flg_no_alpn: true,
        flg_no_buffer: true,
        opt_socks5_gssapi_service: "name1".to_string(),
        opt_speed_time: 1000,
        flg_sslv3: true,
        arg_params: vec!["http://url1.com".to_string()],
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
    c.bench_function("cmp_structopt::curl::", |b| {
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
