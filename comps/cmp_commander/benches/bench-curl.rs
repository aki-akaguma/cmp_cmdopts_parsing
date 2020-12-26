use criterion::{criterion_group, criterion_main, Criterion};
use criterion_cycles_per_byte::CyclesPerByte;

fn process_one(env_args: &[&str]) -> anyhow::Result<cmp_commander::curl::CmdOptConf> {
    cmp_commander::curl::parse_cmdopts_str("prog", env_args)
}

fn criterion_benchmark(c: &mut Criterion<CyclesPerByte>) {
    #[rustfmt::skip]
    let env_args = vec!["prog",
        "-a", "--connecttimeout", "50", "--ftppasv", "--http2",
        "--maxtime", "100", "--noalpn", "-N",
        "--socks5gssapiservice", "name1", "-y", "1000", "--sslv3",
        //"http://url1.com"];
        ];
    let env_args = &env_args;
    //
    let result_conf = cmp_commander::curl::CmdOptConf {
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
        //arg_params: vec!["http://url1.com".to_string()],
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
    c.bench_function("cmp_commander::curl::", |b| b.iter(|| {
        let _r = process_one(criterion::black_box(env_args));
    }));
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = criterion_benchmark);
criterion_main!(benches);
