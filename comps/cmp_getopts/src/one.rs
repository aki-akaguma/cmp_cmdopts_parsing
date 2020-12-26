use getopts::Options;
use optcolorwhen::OptColorWhen;
use optpaerr_a::OptParseError;
use std::str::FromStr;

//----------------------------------------------------------------------
//{{{ CmdOptConf
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
fn full_usage(program: &str, opts: Options) -> String {
    let brief = format!("Usage: {} [options] <input> [<output>]", program);
    opts.usage(&brief)
}

#[inline(never)]
pub fn parse_cmdopts(program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("V", "version", "print version information");
    opts.optflag("d", "debug", "activate debug mode");
    opts.optflagmulti("v", "verbose", "verbose mode. -vv is more verbose");
    opts.optopt("s", "speed", "set speed (default: 42.0)", "<speed>");
    opts.optopt(
        "",
        "color",
        concat!(
            "Use markers to highlight (default: auto)\n",
            "<when> is \'always\', \'never\', or \'auto\'"
        ),
        "<when>",
    );
    opts.optopt("c", "config", "Give a path string argument", "<path>");
    //
    let matches = match opts.parse(env_args) {
        Ok(m) => m,
        Err(err) => {
            return Err(From::from(err));
        }
    };
    if matches.opt_present("help") {
        let msg = full_usage(&program, opts);
        println!("{}", msg);
        std::process::exit(0);
    }
    if matches.opt_present("version") {
        println!("{} {}", program, env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    //
    let mut args = CmdOptConf::default();
    if matches.opt_present("d") {
        args.flag_debug = true;
    }
    args.cnt_verbose = matches.opt_count("v");
    let speed = matches.opt_str("speed");
    args.opt_speed = match speed {
        Some(x) => match x.parse::<f32>() {
            Ok(d) => d,
            Err(err) => {
                return Err(From::from(OptParseError::invalid_option_argument(
                    "-s",
                    &err.to_string(),
                )));
            }
        },
        None => 42.0,
    };
    let color = matches.opt_str("color");
    args.opt_color = match color {
        Some(s) => match FromStr::from_str(s.as_str()) {
            Ok(color) => color,
            Err(err) => {
                return Err(From::from(OptParseError::invalid_option_argument(
                    "--color",
                    &err.to_string(),
                )));
            }
        },
        None => OptColorWhen::Auto,
    };
    args.opt_config = matches.opt_str("config");
    //
    if !matches.free.is_empty() {
        args.arg_input = matches.free[0].clone();
        args.arg_output = if matches.free.len() > 1 {
            Some(matches.free[1].clone())
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
    parse_cmdopts(&program, env_args)
}
