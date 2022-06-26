use gumdrop::Options;
use optcolorwhen::OptColorWhen;
use optpaerr_a::OptParseError;

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
#[derive(Options, Debug, Default)]
pub struct MyOptions {
    #[options(free)]
    free: Vec<String>,
    //
    #[options(help = "Show this help message")]
    help: bool,
    #[options(short = "V", help = "Show version message")]
    version: bool,
    //
    #[options(help = "Activate debug mode")]
    debug: bool,
    #[options(count, help = "Verbose mode. -vv is more verbose")]
    verbose: u32,
    #[options(help = "Set speed (default: 42.0)", meta = "<speed>")]
    speed: f32,
    #[options(
        no_short,
        help = "Use markers to highlight (default: auto). <when> is \'always\', \'never\', or \'auto\'",
        meta = "<when>"
    )]
    color: OptColorWhen,
    #[options(help = "Give a path string argument", meta = "<path>")]
    config: Option<String>,
}
//----------------------------------------------------------------------
fn full_usage(program: &str) -> String {
    let usage = format!("Usage: {} [options] <input> [<output>]", program);
    let args = concat!(
        "Args:\n",
        "  <input>     Input file\n",
        "  <output>    Output file, stdout if not present"
    );
    format!("{}\nOptions:\n{}\n{}", usage, MyOptions::usage(), args)
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
pub fn parse_cmdopts(_program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let opt = match MyOptions::parse_args_default(&env_args) {
        Ok(o) => o,
        Err(err) => return Err(From::from(err)),
    };
    //
    if opt.help {
        print_help_and_exit();
    }
    if opt.version {
        print_version_and_exit();
    }
    //
    let mut args = CmdOptConf {
        flag_debug: opt.debug,
        cnt_verbose: opt.verbose as usize,
        opt_speed: opt.speed,
        opt_color: opt.color,
        opt_config: opt.config,
        ..CmdOptConf::default()
    };
    //
    if !opt.free.is_empty() {
        args.arg_input = opt.free[0].clone();
        args.arg_output = if opt.free.len() > 1 {
            Some(opt.free[1].clone())
        } else {
            None
        }
    } else {
        return Err(From::from(OptParseError::missing_argument("<input>")));
    }
    //
    Ok(args)
}

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(program, env_args)
}
