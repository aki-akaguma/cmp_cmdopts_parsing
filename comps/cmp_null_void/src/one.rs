use optcolorwhen::OptColorWhen;

//----------------------------------------------------------------------
//{{{ CmdOptConf
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CmdOptConf {
    pub flag_debug: bool,
    cnt_verbose: usize,
    opt_speed: f32,
    opt_color: OptColorWhen,
    opt_config: Option<String>,
    arg_input: String,
    arg_output: Option<String>,
}
//}}} CmdOptConf

//----------------------------------------------------------------------
fn full_usage(program: &str) -> String {
    let usage = format!("Usage: {} [options] <input> [<output>]", program);
    let opts = concat!(
        "Options:\n",
        "    -h, --help          Print this help menu\n",
        "    -V, --version       Print version information\n",
        "    -d, --debug         Activate debug mode\n",
        "    -v, --verbose       Verbose mode. -vv is more verbose\n",
        "    -s, --speed <speed> Set speed (default: 42.0)\n",
        "    --color <when>      Use markers to highlight (default: auto)\n",
        "                        <when> is \'always\', \'never\', or \'auto\'\n",
        "    -c, --config <path> Give a path string argument\n",
        "Args:\n",
        "    <input>             Input file name\n",
        "    [<output>]          Output file name, stdout if not present"
    );
    format!("{}\n{}", usage, opts)
}

#[inline(never)]
fn print_help_and_exit() {
    let s = full_usage(env!("CARGO_PKG_NAME"));
    //let s = full_usage(program.as_str());
    println!("{}", s);
    std::process::exit(0);
}

#[inline(never)]
fn print_version_and_exit() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}

#[inline(never)]
pub fn parse_cmdopts(_program: &str, args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    //
    let conf = CmdOptConf {
        opt_speed: 42.0,
        ..Default::default()
    };
    if !args.is_empty() {
        match args[0] {
            "--help" => print_help_and_exit(),
            "--version" => print_version_and_exit(),
            _ => {}
        }
    }
    //
    Ok(conf)
}

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}
