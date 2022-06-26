use argh::FromArgs;
use simple_error::SimpleError;

//----------------------------------------------------------------------
include!("curl.cmd.help.rs.txt");

//----------------------------------------------------------------------
#[inline(never)]
pub fn parse_cmdopts(program: &str, env_args: &[&str]) -> anyhow::Result<CmdOptConf> {
    //
    let mut conf = CmdOptConf {
        opt_program: program.to_string(),
        ..Default::default()
    };
    //
    let opts = match MyOptions::from_args(&[program], env_args) {
        Ok(opts) => opts,
        Err(err) => return Err(From::from(SimpleError::new(err.output))),
    };
    include!("curl.cmd.match.rs.txt");
    //
    conf.arg_params.push(opts.arg_input);
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
