use anyhow::Context;
use std::io::BufRead;

pub fn run(_program: &str, _args: &[&str]) -> anyhow::Result<()> {
    let mut bench_vec = get_bench("z.bench-release.curl.log", "::curl::")?;
    set_size(&mut bench_vec, "z.size-release.curl.log", "-curl")?;
    output(bench_vec)?;
    //
    let mut bench_vec = get_bench("z.bench-release.one.log", "::one::")?;
    set_size(&mut bench_vec, "z.size-release.one.log", "-one")?;
    output(bench_vec)?;
    //
    Ok(())
}

fn output(bench_vec: Vec<BenchStr>) -> anyhow::Result<()> {
    println!(
        "| {:^18} | {:^11} | {:^8} | {:^11} | {:^8} |",
        "`name`", "`bench`", "`.text`", "`Δ bench`", "`Δ .text`"
    );
    println!(
        "|:{:<18}-|-{:>11}:|-{:>8}:|-{:>11}:|-{:>8}:|",
        "-".repeat(18),
        "-".repeat(11),
        "-".repeat(8),
        "-".repeat(11),
        "-".repeat(8)
    );
    for bench in bench_vec {
        if bench.is_cycle {
            println!(
                "| {:<18} | {:>8.3} kc | {:>4} kib | {:>8.3} kc | {:>4} kib |",
                bench.name,
                bench.time / 1000.0,
                bench.size / 1024,
                bench.oh_time / 1000.0,
                bench.oh_size / 1024
            );
        } else {
            println!(
                "| {:<18} | {:>8.3} us | {:>4} kib | {:>8.3} us | {:>4} kib |",
                bench.name,
                bench.time / 0.000001,
                bench.size / 1024,
                bench.oh_time / 0.000001,
                bench.oh_size / 1024
            );
        }
    }
    //
    Ok(())
}

#[rustfmt::skip]
#[derive(Default)]
pub struct BenchStr {
    pub name: String,   // name
    pub time: f64,      // seconds
    pub is_cycle: bool, // cycles
    pub size: u64,      // bytes
    pub oh_time: f64,   // seconds
    pub oh_size: u64,   // bytes
}

fn set_size(bench_vec: &mut Vec<BenchStr>, in_file: &str, suffix: &str) -> anyhow::Result<()> {
    let mut base_time = 0f64;
    let mut base_size = 0u64;
    let re_1 = regex::Regex::new(r"^ *(\d+)\t.*\t([^ ]+)$").unwrap();
    let reader = std::io::BufReader::new(
        std::fs::File::open(in_file)
            .with_context(|| format!("could not open file `{}`", in_file))?,
    );
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re_1.captures(&line) {
            //  934281	  26312	    736	 961329	  eab31	cmp_structopt-curl
            let size_s = &caps[1];
            let name_s = &caps[2];
            let name = if name_s.ends_with(suffix) {
                &name_s[0..(name_s.len() - suffix.len())]
            } else {
                name_s
            };
            let i = match bench_vec.iter().position(|x| x.name == name) {
                Some(i) => i,
                None => {
                    let msg = format!("can not find size: {}", name);
                    return Err(anyhow::Error::msg(msg));
                }
            };
            bench_vec[i].size = size_s.parse::<u64>()?;
            if name == "cmp_null_void" {
                base_time = bench_vec[i].time;
                base_size = bench_vec[i].size;
            }
        }
    }
    //
    for bench in bench_vec {
        bench.oh_time = bench.time - base_time;
        bench.oh_size = bench.size - base_size;
    }
    //
    Ok(())
}

fn get_bench(in_file: &str, suffix: &str) -> anyhow::Result<Vec<BenchStr>> {
    let mut vec_benchstr: Vec<BenchStr> = Vec::new();
    //
    let re_1 =
        regex::Regex::new(r"^([^ ]+) +time: +[\[][^ ]+ [^ ]+ ([^ ]+) ([^ ]+) [^ ]+ [^ ]+[\]]$")
            .unwrap();
    //
    let reader = std::io::BufReader::new(
        std::fs::File::open(in_file)
            .with_context(|| format!("could not open file `{}`", in_file))?,
    );
    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = re_1.captures(&line) {
            // cmp_structopt::curl::   time:   [302.50 us 302.87 us 303.34 us]
            // cmp_structopt::curl::   time:   [714991.6559 cycles 715483.2743 cycles 716029.3928 cycles]
            vec_benchstr.push(BenchStr {
                name: normalize_name(&caps[1], suffix)?,
                time: normalize_time(&caps[2], &caps[3])?,
                is_cycle: if &caps[3] == "cycles" { true } else { false },
                ..BenchStr::default()
            });
        }
    }
    vec_benchstr.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    //
    Ok(vec_benchstr)
}

fn normalize_name(name_s: &str, suffix: &str) -> anyhow::Result<String> {
    fn strip_prefix<'a, 'b>(x: &'a str, prefix: &'b str) -> Option<&'a str> {
        #[cfg(not(has_not_strip_prefix))]
        {
            x.strip_prefix(prefix)
        }
        #[cfg(has_not_strip_prefix)]
        {
            if x.starts_with(&prefix) && x.len() > prefix.len() {
                Some(&x[prefix.len()..])
            } else {
                None
            }
        }
    }
    fn strip_suffix<'a, 'b>(x: &'a str, suffix: &'b str) -> Option<&'a str> {
        #[cfg(not(has_not_strip_prefix))]
        {
            x.strip_suffix(suffix)
        }
        #[cfg(has_not_strip_prefix)]
        {
            if x.ends_with(&suffix) && x.len() > suffix.len() {
                Some(&x[0..(x.len() - suffix.len())])
            } else {
                None
            }
        }
    }
    let name_s = if let Some(x) = strip_suffix(name_s, suffix) {
        x
    } else {
        name_s
    };
    let name = if let Some(x) = strip_prefix(name_s, "cmp_optpa_ut_") {
        format!("cmp_optpa_util_{}", x)
    } else {
        name_s.to_string()
    };
    Ok(name)
}

fn normalize_time(num_s: &str, unit_s: &str) -> anyhow::Result<f64> {
    let num: f64 = num_s.parse::<f64>()?;
    let unit: f64 = match unit_s {
        "ms" => 0.001,
        "us" => 0.000001,
        "µs" => 0.000001,
        "ns" => 0.000000001,
        "ps" => 0.000000000001,
        "cycles" => 1.0,
        _ => {
            let msg = format!("can not convert unit: {}", unit_s);
            return Err(anyhow::Error::msg(msg));
        }
    };
    Ok(num * unit)
}
