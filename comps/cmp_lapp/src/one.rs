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
#[inline(never)]
pub fn parse_cmdopts(_program: &str, env_args: Vec<String>) -> anyhow::Result<CmdOptConf> {
    //let mut aargs = lapp::parse_args(concat!(
    let mut aargs = lapp::Args::new(concat!(
        "Usage: ",
        env!("CARGO_PKG_NAME"),
        " [OPTIONS] <input> [<output>]\n\n",
        "Options:\n",
        "  -V, --version                  Print version information\n",
        "  -d, --debug                    Activate debug mode\n",
        "  -v, --verbose (integer)        Verbose mode. -v2 is more verbose\n",
        "  -s, --speed (default 42.0)     Set speed\n",
        "  --color (default auto)         Use markers to highlight\n",
        "                                 \'always\', \'never\', or \'auto\'\n",
        "  -c, --config (string)          Give a path string argument\n",
        "  -h, --help                     Show this help message.\n\n",
        "Parameters:\n",
        "  <input>     (string)           Input file name\n",
        "  <output>    (string)           Output file name, stdout if not present\n"
    ))
    .start(0);
    if let Err(err) = aargs.parse_spec() {
        let s = format!("{}", err);
        aargs.quit(&s);
    }
    if let Err(err) = aargs.parse_command_line(env_args) {
        let s = format!("{}", err);
        aargs.quit(&s);
    }
    //
    if let Ok(b) = aargs.get_bool_result("version") {
        if b {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
    }
    //
    /*
    let s: String = match aargs.get_string_result("output") {
        Ok(s) => s,
        Err(err) => {println!("AAA// {:?}",err); String::new() }
    };
    let s: String = aargs.get_string("input");
    */
    //
    Ok(CmdOptConf {
        flag_debug: if let Ok(b) = aargs.get_bool_result("debug") {
            b
        } else {
            false
        },
        cnt_verbose: match aargs.get_integer_result("verbose") {
            Ok(i) => i as usize,
            Err(_) => 0,
        },
        opt_speed: match aargs.get_float_result("speed") {
            Ok(f) => f,
            Err(err) => {
                let s = format!("{}", err);
                aargs.quit(&s);
            }
        },
        opt_color: match aargs.get_string_result("color") {
            Ok(s) => match FromStr::from_str(s.as_str()) {
                Ok(color) => color,
                Err(err) => {
                    return Err(From::from(OptParseError::invalid_option_argument(
                        "color",
                        &err.to_string(),
                    )));
                }
            },
            Err(_) => OptColorWhen::Auto,
        },
        opt_config: match aargs.get_string_result("config") {
            Ok(s) => Some(s),
            Err(_) => None,
        },
        arg_input: aargs.get_string("input"),
        arg_output: match aargs.get_string_result("output") {
            Ok(s) => Some(s),
            Err(_) => None,
        },
    })
}

pub fn parse_cmdopts_str(program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<String> = env_args.iter().map(|s| s.to_string()).collect();
    parse_cmdopts(program, env_args)
}

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    //let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}
