use optcolorwhen::OptColorWhen;
use optpaerr_a::OptParseError;
use std::iter::Peekable;
use std::str::FromStr;
use std::vec::IntoIter;

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

fn normalize_cmdopts(args: Vec<&str>) -> Vec<String> {
    fn strip_prefix(x: &str, prefix: char) -> Option<&str> {
        #[cfg(not(has_not_strip_prefix))]
        {
            x.strip_prefix(prefix)
        }
        #[cfg(has_not_strip_prefix)]
        {
            let prefix_s: String = prefix.to_string();
            if x.starts_with(&prefix_s) && x.len() > prefix_s.len() {
                Some(&x[prefix_s.len()..])
            } else {
                None
            }
        }
    }
    let mut rargs: Vec<String> = Vec::with_capacity(args.len());
    for x in args {
        if x.starts_with("--") {
            match x.find('=') {
                Some(i) => {
                    rargs.push(x[0..i].to_string());
                    rargs.push(x[i + 1..].to_string());
                }
                None => {
                    rargs.push(x.to_string());
                }
            };
        } else if let Some(xx) = strip_prefix(x, '-') {
            for c in xx.chars() {
                let mut s = String::new();
                s.push('-');
                s.push(c);
                rargs.push(s);
            }
        } else {
            rargs.push(x.to_string());
        }
    }
    rargs.shrink_to_fit();
    rargs
}

fn parse_cmdopts_match(
    cur_s: &str,
    cursor: &mut Peekable<IntoIter<String>>,
    free: &mut Vec<String>,
    args: &mut CmdOptConf,
) -> anyhow::Result<()> {
    match cur_s {
        "--" => {
            free.extend(cursor);
            return Ok(());
        }
        "-h" | "--help" => {
            let s = full_usage(env!("CARGO_PKG_NAME"));
            //let s = full_usage(program.as_str());
            println!("{}", s);
            std::process::exit(0);
        }
        "-V" | "--version" => {
            println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
            std::process::exit(0);
        }
        "-d" | "--debug" => {
            args.flag_debug = true;
        }
        "-v" | "--verbose" => {
            args.cnt_verbose += 1;
        }
        "-s" | "--speed" => {
            let speed = cursor.next();
            args.opt_speed = match speed {
                Some(x) => match x.parse::<f32>() {
                    Ok(d) => d,
                    Err(err) => {
                        return Err(From::from(OptParseError::invalid_option_argument(
                            cur_s,
                            &err.to_string(),
                        )));
                    }
                },
                None => 42.0,
            };
        }
        "--color" => {
            let color_s = cursor.next();
            args.opt_color = match color_s {
                Some(s) => match FromStr::from_str(s.as_str()) {
                    Ok(color) => color,
                    Err(err) => {
                        return Err(From::from(OptParseError::invalid_option_argument(
                            cur_s,
                            &err.to_string(),
                        )));
                    }
                },
                None => OptColorWhen::Auto,
            };
        }
        "-c" | "--config" => {
            let config_s = cursor.next();
            args.opt_config = config_s;
        }
        _ => {
            if !cur_s.starts_with('-') {
                free.push(cur_s.to_string());
            } else {
                return Err(From::from(OptParseError::invalid_option(cur_s)));
            }
        }
    }
    //
    Ok(())
}

#[inline(never)]
pub fn parse_cmdopts(_program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let env_args = normalize_cmdopts(env_args);
    //
    let mut args = CmdOptConf {
        opt_speed: 42.0,
        ..Default::default()
    };
    let mut free: Vec<String> = Vec::new();
    //
    let mut cursor = env_args.into_iter().peekable();
    while let Some(cur) = cursor.next() {
        let cur_s = cur.as_str();
        match parse_cmdopts_match(cur_s, &mut cursor, &mut free, &mut args) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
    }
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

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(program, env_args)
}
