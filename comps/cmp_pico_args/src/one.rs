use optcolorwhen::OptColorWhen;
use optpaerr_a::OptParseError;
use std::ffi::OsString;
use std::str::FromStr;
//

//----------------------------------------------------------------------
//{{{ CmdOptConf
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CmdOptConf {
    flag_debug: bool,
    cnt_verbose: usize,
    opt_speed: f32,
    opt_color: OptColorWhen,
    opt_config: Option<String>,
    arg_input: String,
    arg_output: Option<String>,
}
//}}} CmdOptConf

//----------------------------------------------------------------------
//----------------------------------------------------------------------
fn full_usage(program: &str) -> String {
    let usage = format!("Usage: {} [options] <input> [<output>]\n", program);
    let opts = concat!(
        "Options:\n",
        "    -h, --help               Print this help menu\n",
        "    -V, --version            Print version information\n",
        "    -d, --debug              Activate debug mode\n",
        "    -v, --verbose <integer>  Verbose mode. -v2 is more verbose\n",
        "    -s, --speed <speed>      Set speed (default: 42.0)\n",
        "    --color <when>           Use markers to highlight (default: auto)\n",
        "                             <when> is \'always\', \'never\', or \'auto\'\n",
        "    -c, --config <path>      Give a path string argument\n",
        "Args:\n",
        "    <input>             Input file name\n",
        "    [<output>]          Output file name, stdout if not present",
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

fn parse_color(s: &str) -> Result<OptColorWhen, String> {
    match FromStr::from_str(s) {
        Ok(color) => Ok(color),
        Err(err) => Err(err.to_string()),
    }
}

#[inline(never)]
pub fn parse_cmdopts(_program: &str, env_args: Vec<OsString>) -> anyhow::Result<CmdOptConf> {
    let mut pico_args = pico_args::Arguments::from_vec(env_args);

    //
    if pico_args.contains(["-h", "--help"]) {
        print_help_and_exit();
    }
    if pico_args.contains(["-V", "--version"]) {
        print_version_and_exit();
    }
    //
    let mut args = CmdOptConf {
        flag_debug: pico_args.contains(["-d", "--debug"]),
        cnt_verbose: pico_args
            .opt_value_from_str(["-v", "--verbose"])?
            .unwrap_or(0),
        opt_speed: pico_args
            .opt_value_from_str(["-s", "--speed"])?
            .unwrap_or(42.0),
        opt_color: pico_args
            .opt_value_from_fn("--color", parse_color)?
            .unwrap_or(OptColorWhen::Auto),
        opt_config: pico_args.opt_value_from_str(["-c", "--config"])?,
        ..CmdOptConf::default()
    };
    //
    let mut free: Vec<String> = Vec::new();
    while let Ok(s) = pico_args.free_from_str() {
        free.push(s)
    }
    //assert_eq!(format!("{:?}", free), "");
    //let free = pico_args.free()?;
    if !free.is_empty() {
        args.arg_input.clone_from(&free[0]);
        args.arg_output = if free.len() > 1 {
            Some(free[1].clone())
        } else {
            None
        }
    } else {
        return Err(From::from(OptParseError::missing_argument("<input>")));
    }
    //
    Ok(args)
}

pub fn parse_cmdopts_str(program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<OsString> = env_args.iter().map(OsString::from).collect();
    parse_cmdopts(program, env_args)
}

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts_str(program, env_args)
}
