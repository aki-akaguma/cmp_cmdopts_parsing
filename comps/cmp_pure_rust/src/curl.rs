use optpaerr_a::OptParseError;
use std::iter::Peekable;
use std::vec::IntoIter;

//----------------------------------------------------------------------
include!("curl.cmd.help.rs.txt");

//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"
transfer a URL, another name is the multiprotocol getter
"#;

const ARGUMENTS_TEXT: &str = r#"Argument:
  <url>                     url to getting, protocol is http or ftp
"#;

const EXAMPLES_TEXT: &str = r#"Examples:
  You  can specify multiple URLs or parts of URLs by writing part sets within braces as in:
    curl "http://site.{one,two,three}.comn"
  you can get sequences of alphanumeric series by using [] as in:
    curl "ftp://ftp.example.com/file[1-100].txt"
"#;
//}}} TEXT

//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    format!("Usage:\n  {} {}", program, "[options] <url>")
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message("curl");
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT,
        ARGUMENTS_TEXT, EXAMPLES_TEXT].join("\n")
}

#[inline(never)]
fn print_help_and_exit(conf: &CmdOptConf) {
    print!("{}", help_message(&conf.opt_program));
    std::process::exit(0);
}

#[inline(never)]
fn print_version_and_exit(conf: &CmdOptConf) {
    println!("{}", version_message(&conf.opt_program));
    std::process::exit(0);
}

//#[inline(never)]
fn value_to_string(cur_s: &str, value: Option<String>) -> Result<String, OptParseError> {
    match value {
        Some(x) => Ok(x),
        None => Err(OptParseError::missing_option_argument(cur_s)),
    }
}

//#[inline(never)]
fn value_to_u32(cur_s: &str, value: Option<String>) -> Result<u32, OptParseError> {
    match value {
        Some(x) => match x.parse::<u32>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                cur_s,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(cur_s)),
    }
}

//#[inline(never)]
fn value_to_u64(cur_s: &str, value: Option<String>) -> Result<u64, OptParseError> {
    match value {
        Some(x) => match x.parse::<u64>() {
            Ok(d) => Ok(d),
            Err(err) => Err(OptParseError::invalid_option_argument(
                cur_s,
                &err.to_string(),
            )),
        },
        None => Err(OptParseError::missing_option_argument(cur_s)),
    }
}

fn normalize_cmdopts(args: &[&str]) -> Vec<String> {
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
    conf: &mut CmdOptConf,
) -> anyhow::Result<()> {
    include!("curl.cmd.match.rs.txt");
    //
    Ok(())
}

#[inline(never)]
pub fn parse_cmdopts(program: &str, env_args: &[&str]) -> anyhow::Result<CmdOptConf> {
    let env_args = normalize_cmdopts(env_args);
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        ..Default::default()
    };
    let mut free: Vec<String> = Vec::new();
    //
    let mut cursor = env_args.into_iter().peekable();
    while let Some(cur) = cursor.next() {
        let cur_s = cur.as_str();
        match parse_cmdopts_match(cur_s, &mut cursor, &mut free, &mut conf) {
            Ok(_) => {}
            Err(err) => return Err(err),
        }
    }
    if !free.is_empty() {
        let iter = free.iter();
        conf.arg_params.extend(iter.map(|s| s.to_string()));
    } else {
        return Err(From::from(OptParseError::missing_argument("<url>")));
    }
    //
    Ok(conf)
}

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(program, &env_args)
}
