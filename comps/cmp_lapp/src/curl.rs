
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

fn parse_match(conf: &mut CmdOptConf, aargs: &lapp::Args) -> anyhow::Result<()> {
    include!("curl.cmd.match.rs.txt");
    Ok(())
}

pub fn parse_cmdopts(program: &str, args: Vec<String>) -> anyhow::Result<CmdOptConf> {
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        ..Default::default()
    };
    //
    let mut aargs = lapp::Args::new(OPTIONS_TEXT).start(0);
    if let Err(err) = aargs.parse_spec() {
        let s = format!("{}", err);
        aargs.quit(&s);
    }
    if let Err(err) = aargs.parse_command_line(args) {
        let s = format!("{}", err);
        aargs.quit(&s);
    }
    parse_match(&mut conf, &aargs)?;
    //
    if let Ok(s) = aargs.get_string_result("URL") {
        conf.arg_params.push(s);
    }
    //
    Ok(conf)
}

//----------------------------------------------------------------------
pub fn parse_cmdopts_str(
    program: &str,
    env_args: &[&str],
) -> anyhow::Result<CmdOptConf> {
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
