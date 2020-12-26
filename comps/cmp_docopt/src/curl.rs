use docopt::Docopt;
use serde_derive::*;

//----------------------------------------------------------------------
include!("docopt.curl.cmd.help.rs.txt");
//----------------------------------------------------------------------
pub fn parse_cmdopts(_program: &str, args: &[&str]) -> anyhow::Result<CmdOptConf> {
    let version = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let conf: CmdOptConf = Docopt::new(HELP_TEXT)
        .and_then(|d| {
            d.options_first(true)
                .help(true)
                .version(Option::Some(version))
                .argv(args)
                .deserialize()
        })
        .unwrap_or_else(|e| e.exit());
    //
    Ok(conf)
}

//----------------------------------------------------------------------
pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<String> = std::env::args().collect();
    //let program = env_args.remove(0);
    let program = env_args[0].clone();
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, &env_args)
}
