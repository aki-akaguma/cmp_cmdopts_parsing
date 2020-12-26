use argh::FromArgs;
use optcolorwhen::OptColorWhen;
use simple_error::SimpleError;

//----------------------------------------------------------------------
//{{{ CmdOptConf
#[derive(FromArgs, PartialEq, Debug)]
/// this is one program
pub struct CmdOptConf {
    /// activate debug mode
    #[argh(switch, short = 'd', long = "debug")]
    flag_debug: bool,
    //
    /// verbose mode. -vv is more verbose
    #[argh(option, short = 'v', long = "verbose", default = "0u64")]
    cnt_verbose: u64,

    // An argument of type float, with a default value.
    /// set speed
    #[argh(option, short = 's', long = "speed", default = "42f32")]
    opt_speed: f32,
    //
    /// use markers to highlight\n<when> is 'always', 'never' or 'auto'
    #[argh(option, long = "color", default = "OptColorWhen::Auto")]
    opt_color: OptColorWhen,
    //
    /// give a path string argument
    #[argh(option, short = 'c', long = "config")]
    opt_config: Option<String>,
    //
    /// input file
    #[argh(positional)]
    arg_input: String,
    //
    /// output file, stdout if not present
    #[argh(positional)]
    arg_output: Option<String>,
}
//}}} CmdOptConf

#[inline(never)]
pub fn parse_cmdopts(program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let args = CmdOptConf::from_args(&[program], &env_args);
    match args {
        Ok(args) => Ok(args),
        Err(err) => Err(From::from(SimpleError::new(err.output))),
    }
}

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}
