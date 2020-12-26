use structopt::StructOpt;

//----------------------------------------------------------------------
include!("curl.cmd.help.rs.txt");

//----------------------------------------------------------------------
#[inline(never)]
pub fn parse_cmdopts(program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        ..Default::default()
    };
    //
    let opts = MyOptions::from_iter(env_args);
    include!("curl.cmd.match.rs.txt");
    //
    conf.arg_params.push(opts.arg_input);
    //
    Ok(conf)
}

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<String> = std::env::args().collect();
    //let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}
