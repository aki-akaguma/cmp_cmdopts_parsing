//
// ref)
//   https://github.com/matklad/cargo-xtask
//
mod shape_benchmark_results;

fn main() -> anyhow::Result<()> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    if env_args.is_empty() {
        print_help_and_exit(&program);
    }
    let cmd = env_args[0].as_str();
    let program = &program;
    let env_args: Vec<&str> = env_args[1..].iter().map(|s| s.as_str()).collect();
    #[rustfmt::skip]
    match cmd {
        "shape_benchmark_results" => shape_benchmark_results::run(&format!("{} {}", program, cmd), &env_args)?,
        "all_gen_src" => all_cmd(program, &env_args)?,
        //
        "cmp_null_void" => cmp_null_void_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_app" => cmp_app_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_argh" => cmp_argh_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_args" => cmp_args_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_clap" => cmp_clap_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_clap3" => cmp_clap3_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_commander" => cmp_commander_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_docopt" => cmp_docopt_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_flood_tide" => cmp_flood_tide_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_getopts" => cmp_getopts_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_gumdrop" => cmp_gumdrop_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_lapp" => cmp_lapp_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_pico_args" => cmp_pico_args_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_pure_rust" => cmp_pure_rust_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_rustop" => cmp_rustop_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "cmp_structopt" => cmp_structopt_xtask::run(&format!("{} {}", program, cmd), &env_args)?,
        "--help" | "-h" | "-H" | "help" => print_help_and_exit(program),
        "--version" | "-V" | "-v" => print_version_and_exit(program),
        _ => {
            eprintln!("Not fount command: {}", cmd);
            unreachable!()
        }
    };
    //
    Ok(())
}

fn print_version_and_exit(_program: &str) {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}

fn print_help_and_exit(program: &str) {
    println!(
        "[usage] {} {{ {} }}",
        program,
        concat!(
            "shape_benchmark_results|all_gen_src",
            "\n    ",
            "|cmp_null_void|cmp_app|cmp_argh|cmp_args|cmp_clap|cmp_clap3|cmp_commander",
            "\n    ",
            "|cmp_docopt|cmp_flood_tide|cmp_getopts|cmp_gumdrop|cmp_lapp",
            "\n    ",
            "|cmp_pico_args|cmp_pure_rust|cmp_rustop|cmp_structopt",
        )
    );
    std::process::exit(0);
}

pub fn all_cmd(program: &str, env_args: &[&str]) -> anyhow::Result<()> {
    if env_args.is_empty() {
        all_print_help_and_exit(&program);
    }
    let cmd = env_args[0];
    match cmd {
        "gen-src-curl-cmd" => all_run(program, env_args)?,
        "--help" | "-h" | "-H" | "help" => {
            all_print_help_and_exit(&format!("{} {}", program, "all"))
        }
        "--version" | "-V" | "-v" => print_version_and_exit(&format!("{} {}", program, "all")),
        _ => {
            eprintln!("Not fount command: {}", cmd);
            unreachable!()
        }
    }
    //
    Ok(())
}

fn all_print_help_and_exit(program: &str) {
    println!("[usage] {} {{{}}}", program, concat!("gen-src-curl-cmd",));
    std::process::exit(0);
}

pub fn all_run(program: &str, env_args: &[&str]) -> anyhow::Result<()> {
    cmp_null_void_xtask::run(&format!("{} {}", program, "cmp_null_void"), &env_args)?;
    cmp_app_xtask::run(&format!("{} {}", program, "cmp_app"), &env_args)?;
    cmp_argh_xtask::run(&format!("{} {}", program, "cmp_argh"), &env_args)?;
    cmp_args_xtask::run(&format!("{} {}", program, "cmp_args"), &env_args)?;
    cmp_clap_xtask::run(&format!("{} {}", program, "cmp_clap"), &env_args)?;
    cmp_clap3_xtask::run(&format!("{} {}", program, "cmp_clap3"), &env_args)?;
    cmp_commander_xtask::run(&format!("{} {}", program, "cmp_commander"), &env_args)?;
    cmp_docopt_xtask::run(&format!("{} {}", program, "cmp_docopt"), &env_args)?;
    cmp_flood_tide_xtask::run(&format!("{} {}", program, "cmp_docopt"), &env_args)?;
    cmp_getopts_xtask::run(&format!("{} {}", program, "cmp_getopts"), &env_args)?;
    cmp_gumdrop_xtask::run(&format!("{} {}", program, "cmp_gumdrop"), &env_args)?;
    cmp_lapp_xtask::run(&format!("{} {}", program, "cmp_lapp"), &env_args)?;
    cmp_pico_args_xtask::run(&format!("{} {}", program, "cmp_pico_args"), &env_args)?;
    cmp_pure_rust_xtask::run(&format!("{} {}", program, "cmp_pure_rust"), &env_args)?;
    cmp_rustop_xtask::run(&format!("{} {}", program, "cmp_rustop"), &env_args)?;
    cmp_structopt_xtask::run(&format!("{} {}", program, "cmp_structopt"), &env_args)?;
    //
    Ok(())
}
