//
// ref)
//   https://github.com/matklad/cargo-xtask
//
mod gen_src_curl_cmd;

pub fn run(program: &str, env_args: &[&str]) -> anyhow::Result<()> {
    if env_args.is_empty() {
        print_help_and_exit(&program);
    }
    let cmd = env_args[0];
    match cmd {
        "gen-src-curl-cmd" => gen_src_curl_cmd::do_gen_src()?,
        "--help" | "-h" | "-H" | "help" => print_help_and_exit(&program),
        "--version" | "-V" | "-v" => print_version_and_exit(&program),
        _ => {
            eprintln!("Not fount command: {}", cmd);
            unreachable!()
        }
    }
    //
    Ok(())
}

fn print_version_and_exit(_program: &str) {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}

fn print_help_and_exit(program: &str) {
    println!("[usage] {} {{{}}}", program, concat!("gen-src-curl-cmd",));
    std::process::exit(0);
}
