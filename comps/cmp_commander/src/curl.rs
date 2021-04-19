use commander::Commander;

//----------------------------------------------------------------------
include!("curl.cmd.help.rs.txt");

//----------------------------------------------------------------------
fn parse_match(conf: &mut CmdOptConf, cmd: &Commander) -> anyhow::Result<()> {
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
    let cmd = include!("curl.cmd.lex.rs.txt");
    parse_match(&mut conf, &cmd)?;
    //
    Ok(conf)
}

pub fn parse_cmdopts_str(program: &str, env_args: &[&str]) -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<String> = env_args.iter().map(|s| s.to_string()).collect();
    parse_cmdopts(program, env_args)
}

//----------------------------------------------------------------------
pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<String> = std::env::args().collect();
    //let program = env_args.remove(0);
    let program = env_args[0].clone();
    //let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}
