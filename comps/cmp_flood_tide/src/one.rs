use flood_tide::OptParseError;
#[cfg(not(feature = "single_error"))]
use flood_tide::OptParseErrors;
use optcolorwhen::OptColorWhen;
use std::str::FromStr;

//
use flood_tide::Arg;
use flood_tide::Lex;
use flood_tide::NameVal;
use flood_tide::Opt;

//----------------------------------------------------------------------
//{{{ CmdOptConf
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

//
#[repr(u8)]
#[derive(Debug, PartialEq)]
enum CmdOP {
    Help = 1,
    Version = 2,
    Debug = 3,
    Verbose = 4,
    Speed = 5,
    Color = 6,
    Config = 7,
}
impl std::convert::From<u8> for CmdOP {
    fn from(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }
}

#[inline(never)]
fn mk_invalid_option_argument(nv: &NameVal, err_s: &str) -> OptParseError {
    #[cfg(feature = "was_long")]
    let name = if nv.was_long {
        nv.opt.lon.to_string()
    } else {
        (nv.opt.sho as char).to_string()
    };
    #[cfg(not(feature = "was_long"))]
    let name = nv.opt.lon.to_string();
    OptParseError::invalid_option_argument(&name, err_s)
}

//#[inline(never)]
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    match CmdOP::from(nv.opt.num) {
        CmdOP::Help => {
            print_help_and_exit();
        }
        CmdOP::Version => {
            print_version_and_exit();
        }
        CmdOP::Debug => {
            conf.flag_debug = true;
        }
        CmdOP::Verbose => {
            conf.cnt_verbose += 1;
        }
        CmdOP::Speed => {
            conf.opt_speed = match nv.val {
                Some(x) => match x.parse::<f32>() {
                    Ok(d) => d,
                    Err(err) => return Err(mk_invalid_option_argument(nv, &err.to_string())),
                },
                None => 42.0,
            };
        }
        CmdOP::Color => {
            conf.opt_color = match nv.val {
                Some(s) => match FromStr::from_str(s) {
                    Ok(color) => color,
                    Err(err) => return Err(mk_invalid_option_argument(nv, &err.to_string())),
                },
                None => OptColorWhen::Auto,
            };
        }
        CmdOP::Config => {
            conf.opt_config = match nv.val {
                Some(s) => Some(s.to_string()),
                None => None,
            };
        }
    }
    Ok(())
}

//
#[rustfmt::skip]
const OPT_ARY: [Opt; 7] = [
    Opt { sho: 0u8,  lon: "color",  has: Arg::Yes, num: CmdOP::Color as u8 },
    Opt { sho: b'c', lon: "config",  has: Arg::Yes, num: CmdOP::Config as u8 },
    Opt { sho: b'd', lon: "debug",   has: Arg::No,  num: CmdOP::Debug as u8 },
    Opt { sho: b'h', lon: "help",    has: Arg::No,  num: CmdOP::Help as u8 },
    Opt { sho: b's', lon: "speed",   has: Arg::Yes, num: CmdOP::Speed as u8 },
    Opt { sho: b'v', lon: "verbose", has: Arg::No,  num: CmdOP::Verbose as u8 },
    Opt { sho: b'V', lon: "version", has: Arg::No,  num: CmdOP::Version as u8 },
];

#[rustfmt::skip]
const OPT_ARY_SHO_IDX: [(u8,usize);6] = [
    (b'V',6),(b'c',1),(b'd',2),(b'h',3),(b's',4),(b'v',5),
];

#[cfg(feature = "single_error")]
#[inline(never)]
pub fn parse_cmdopts(_program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    //
    let mut conf = CmdOptConf {
        opt_speed: 42.0,
        ..Default::default()
    };
    //
    {
        //
        let lex = Lex::create_with(&OPT_ARY, &OPT_ARY_SHO_IDX);
        let tokens = match lex.tokens_from(&env_args) {
            Ok(t) => t,
            Err(err) => return Err(From::from(err)),
        };
        //
        for nv in tokens.namevals.iter() {
            match parse_match(&mut conf, &nv) {
                Ok(_) => {}
                Err(err) => return Err(From::from(err)),
            }
        }
        let free = tokens.free;
        if !free.is_empty() {
            conf.arg_input = free[0].to_string();
            conf.arg_output = if free.len() > 1 {
                Some(free[1].to_string())
            } else {
                None
            }
        } else {
            return Err(From::from(OptParseError::missing_argument("<input>")));
        }
    }
    //
    Ok(conf)
}

#[rustfmt::skip]
#[cfg(feature = "single_error")]
pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}

#[cfg(not(feature = "single_error"))]
//#[inline(never)]
pub fn parse_cmdopts(_program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    //
    let mut conf = CmdOptConf {
        opt_speed: 42.0,
        ..Default::default()
    };
    //
    {
        let lex = Lex::create_from(&OPT_ARY).stop_at(&["--"]);
        let tokens = match lex.tokens_from(&env_args) {
            Ok(t) => t,
            Err(errs) => {
                return Err(From::from(errs));
            }
        };
        //
        let mut errs = OptParseErrors::new();
        for nv in tokens.namevals.iter() {
            match parse_match(&mut conf, &nv) {
                Ok(_) => {}
                Err(e) => errs.push(e),
            }
        }
        let free = tokens.free;
        if !free.is_empty() {
            conf.arg_input = free[0].to_string();
            conf.arg_output = if free.len() > 1 {
                Some(free[1].to_string())
            } else {
                None
            }
        } else {
            errs.push(OptParseError::missing_argument("<input>"));
        }
        if !errs.is_empty() {
            return Err(From::from(errs));
        }
    }
    //
    Ok(conf)
}

#[rustfmt::skip]
#[cfg(not(feature = "single_error"))]
pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}

#[cfg(test)]
mod test {
    use flood_tide::check;

    #[test]
    fn check() {
        assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
            &super::OPT_ARY,
            &super::OPT_ARY_SHO_IDX
        ));
    }
}
