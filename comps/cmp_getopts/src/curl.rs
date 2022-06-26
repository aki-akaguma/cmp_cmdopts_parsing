use getopts::Matches;
use getopts::Options;
use optpaerr_a::OptParseError;

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

fn value_to_string(nm: &str, val: Option<String>) -> anyhow::Result<String> {
    match val {
        Some(x) => Ok(x),
        None => Err(From::from(OptParseError::missing_option_argument(nm))),
    }
}

fn value_to_u32(nm: &str, val: Option<String>) -> anyhow::Result<u32> {
    match val {
        Some(x) => match x.parse::<u32>() {
            Ok(d) => Ok(d),
            Err(err) => Err(From::from(OptParseError::invalid_option_argument(
                nm,
                &err.to_string(),
            ))),
        },
        None => Err(From::from(OptParseError::missing_option_argument(nm))),
    }
}

fn value_to_u64(nm: &str, val: Option<String>) -> anyhow::Result<u64> {
    match val {
        Some(x) => match x.parse::<u64>() {
            Ok(d) => Ok(d),
            Err(err) => Err(From::from(OptParseError::invalid_option_argument(
                nm,
                &err.to_string(),
            ))),
        },
        None => Err(From::from(OptParseError::missing_option_argument(nm))),
    }
}

fn parse_match(conf: &mut CmdOptConf, matches: &Matches) -> anyhow::Result<()> {
    include!("curl.cmd.match.rs.txt");
    Ok(())
}

pub fn parse_cmdopts(program: &str, args: &[&str]) -> anyhow::Result<CmdOptConf> {
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        ..Default::default()
    };
    //
    let mut opts = Options::new();
    include!("curl.cmd.lex.rs.txt");
    //
    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(err) => {
            return Err(From::from(err));
        }
    };
    parse_match(&mut conf, &matches)?;
    //
    let free = matches.free;
    if !free.is_empty() {
        let iter = free.iter();
        conf.arg_params.extend(iter.map(|s| s.to_string()));
    }
    //
    Ok(conf)
}

//----------------------------------------------------------------------
pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, &env_args)
}
