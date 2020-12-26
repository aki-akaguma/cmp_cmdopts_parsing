use app::{App, Args, Opt};

//----------------------------------------------------------------------
include!("curl.cmd.help.rs.txt");
//----------------------------------------------------------------------
pub fn parse_cmdopts(program: &str, args: &[String]) -> anyhow::Result<CmdOptConf> {
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        ..Default::default()
    };
    //
    let app = include!("curl.cmd.lex.rs.txt");
    let _helper = app.parse(args);
    //
    Ok(conf)
}

//----------------------------------------------------------------------
#[rustfmt::skip]
pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    parse_cmdopts(&program, &env_args)
}
