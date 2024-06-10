use rustop::*;

//----------------------------------------------------------------------
include!("curl.cmd.help.rs.txt");
//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[inline(never)]
fn print_version_and_exit(conf: &CmdOptConf) {
    println!("{}", version_message(&conf.opt_program));
    std::process::exit(0);
}

fn value_to_string(_nm: &str, val: Option<String>) -> anyhow::Result<String> {
    match val {
        Some(x) => Ok(x),
        //None => Err(From::from(OptParseError::missing_option_argument(&nm))),
        None => Ok(String::new()),
    }
}

fn value_to_u32(_nm: &str, val: Option<u32>) -> anyhow::Result<u32> {
    match val {
        Some(x) => Ok(x),
        //None => Err(From::from(OptParseError::missing_option_argument(&nm))),
        None => Ok(0),
    }
}

fn value_to_u64(_nm: &str, val: Option<u64>) -> anyhow::Result<u64> {
    match val {
        Some(x) => Ok(x),
        //None => Err(From::from(OptParseError::missing_option_argument(&nm))),
        None => Ok(0),
    }
}

pub fn parse_cmdopts(program: &str, args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        ..Default::default()
    };
    //
    let rop = include!("curl.cmd.lex.rs.txt");
    //
    let (ropa, _) = match rop.parse_args(args) {
        Ok(a) => a,
        Err(rustop::Error::Help(msg)) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        }
        Err(err) => rustop::error_and_exit(&err),
    };
    include!("curl.cmd.match.rs.txt");
    //
    Ok(conf)
}

//----------------------------------------------------------------------
pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(program, env_args)
}
