use optcolorwhen::OptColorWhen;
use structopt::StructOpt;

//----------------------------------------------------------------------

//{{{ CmdOptConf
#[allow(dead_code)]
#[derive(StructOpt, Debug)]
//# [structopt(name = "cmp_structopt", author = "", about = "An example of StructOpt usage.")]
#[structopt()]
pub struct CmdOptConf {
    /// A flag, true if used in the command line.
    #[structopt(
        name = "debug",
        short = "d",
        long = "debug",
        help = "Activate debug mode"
    )]
    flag_debug: bool,

    #[structopt(
        name = "verbose",
        short = "v",
        long = "verbose",
        help = "Verbose mode. -vv is more verbose",
        default_value = "0"
    )]
    cnt_verbose: u64,

    /// An argument of type float, with a default value.
    #[structopt(
        name = "speed",
        short = "s",
        long = "speed",
        help = "Set speed",
        default_value = "42"
    )]
    opt_speed: f32,

    #[structopt(
        name = "color",
        long = "color",
        help = "Use markers to highlight\n<when> is \'always\', \'never\',\nor \'auto\'",
        default_value = "auto"
    )]
    opt_color: OptColorWhen,

    #[structopt(
        name = "config",
        short = "c",
        long = "config",
        help = "Give a path string argument"
    )]
    opt_config: Option<String>,

    /// Needed parameter, the first on the command line.
    #[structopt(name = "input", help = "Input file")]
    arg_input: String,

    /// An optional parameter, will be `None` if not present on the
    /// command line.
    #[structopt(name = "output", help = "Output file, stdout if not present")]
    arg_output: Option<String>,
}
//}}} CmdOptConf

#[inline(never)]
pub fn parse_cmdopts(_program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let args = CmdOptConf::from_iter(env_args);
    Ok(args)
}

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<String> = std::env::args().collect();
    //let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(program, env_args)
}
