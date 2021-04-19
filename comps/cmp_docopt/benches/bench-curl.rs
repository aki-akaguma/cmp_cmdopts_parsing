use criterion::{criterion_group, criterion_main, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

fn process_one(env_args: &[&str]) -> anyhow::Result<cmp_docopt::curl::CmdOptConf> {
    cmp_docopt::curl::parse_cmdopts("prog", env_args)
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
    let result_conf = cmp_docopt::curl::CmdOptConf {
        //opt_program: "prog".to_string(),
        flag_append: true,
        flag_connect_timeout: 50,
        flag_ftp_pasv: true,
        flag_http2: true,
        flag_max_time: 100,
        flag_no_alpn: true,
        flag_no_buffer: true,
        flag_socks5_gssapi_service: Some("name1".to_string()),
        flag_speed_time: 1000,
        flag_sslv3: true,
        arg_getting_url: "http://url1.com".to_string(),
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
    c.bench_function("cmp_docopt::curl::", |b| {
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
