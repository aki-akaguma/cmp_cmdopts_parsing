use clap::{App, Arg, ArgMatches};
use optpaerr_a::OptParseError;

//----------------------------------------------------------------------
include!("curl.cmd.help.rs.txt");

//----------------------------------------------------------------------
fn value_to_string(nm: &str, val: Option<&str>) -> anyhow::Result<String> {
    match val {
        Some(x) => Ok(x.to_string()),
        None => Err(From::from(OptParseError::missing_option_argument(nm))),
    }
}

fn value_to_u32(nm: &str, val: Option<&str>) -> anyhow::Result<u32> {
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

fn value_to_u64(nm: &str, val: Option<&str>) -> anyhow::Result<u64> {
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

fn parse_match(conf: &mut CmdOptConf, matches: &ArgMatches) -> anyhow::Result<()> {
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
    let app = include!("curl.cmd.lex.rs.txt");
    //
    let matches = app.get_matches_from(args);
    parse_match(&mut conf, &matches)?;
    //
    if let Some(x) = matches.value_of("ARG-URL") {
        conf.arg_params.push(x.to_string())
    };
    //
    Ok(conf)
}

//----------------------------------------------------------------------
pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<String> = std::env::args().collect();
    let program = env_args[0].clone();
    //let _program = env_args.remove(0);
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, &env_args)
}
