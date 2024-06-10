use args_hcc::Args;
use getopts::Occur;
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
fn full_usage(program: &str, args: Args) -> String {
    let brief = format!("Usage: {} [options] <input> [<output>]", program);
    format!("{}\n\n{}", brief, args.usage())
}

#[inline(never)]
pub fn parse_cmdopts(_program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let program = env!("CARGO_PKG_NAME");
    //
    let mut aargs = Args::new(program, "program description.");
    aargs.flag("h", "help", "print this help menu");
    aargs.flag("V", "version", "print version information");
    aargs.flag("d", "debug", "activate debug mode");
    aargs.option(
        "v",
        "verbose",
        "verbose level. -v2 is more verbose",
        "<level>",
        Occur::Optional,
        None,
    );
    aargs.option(
        "s",
        "speed",
        "set speed (default: 42.0)",
        "<speed>",
        Occur::Optional,
        Some("42.0".to_string()),
    );
    aargs.option(
        "",
        "color",
        concat!(
            "Use markers to highlight (default: auto)\n",
            "<when> is \'always\', \'never\', or \'auto\'"
        ),
        "<when>",
        Occur::Optional,
        Some("auto".to_string()),
    );
    aargs.option(
        "c",
        "config",
        "Give a path string argument",
        "<path>",
        Occur::Optional,
        None,
    );
    //
    let matches = match aargs.parse(&env_args) {
        Ok(m) => m,
        Err(err) => {
            return Err(From::from(err));
        }
    };
    let help = aargs.value_of("help")?;
    if help {
        let msg = full_usage(program, aargs);
        println!("{}", msg);
        std::process::exit(0);
    }
    let version = aargs.value_of("version")?;
    if version {
        println!("{} {}", program, env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    //
    let mut args = CmdOptConf::default();
    //
    if let Ok(v) = aargs.value_of("debug") {
        args.flag_debug = v;
    }
    if let Ok(v) = aargs.value_of("verbose") {
        let s: String = v;
        args.cnt_verbose = match s.parse::<usize>() {
            Ok(d) => d,
            Err(err) => {
                return Err(From::from(OptParseError::invalid_option_argument(
                    "-v",
                    &err.to_string(),
                )));
            }
        }
    }
    //
    if let Ok(v) = aargs.value_of("speed") {
        let s: String = v;
        args.opt_speed = match s.parse::<f32>() {
            Ok(d) => d,
            Err(err) => {
                return Err(From::from(OptParseError::invalid_option_argument(
                    "-s",
                    &err.to_string(),
                )));
            }
        }
    }
    //
    if let Ok(v) = aargs.value_of("color") {
        let s: String = v;
        args.opt_color = match FromStr::from_str(s.as_str()) {
            Ok(color) => color,
            Err(err) => {
                return Err(From::from(OptParseError::invalid_option_argument(
                    "--color",
                    &err.to_string(),
                )));
            }
        }
    }
    //
    if let Ok(v) = aargs.value_of("config") {
        args.opt_config = Some(v);
    }
    //
    if !matches.free.is_empty() {
        args.arg_input.clone_from(&matches.free[0]);
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
    let program = env_args.remove(0);
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}
