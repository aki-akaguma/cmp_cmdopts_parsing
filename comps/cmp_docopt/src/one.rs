use docopt::Docopt;
use serde_derive::*;

//----------------------------------------------------------------------
//{{{ CmdOptConf
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct CmdOptConf {
    flag_debug: bool,
    flag_verbose: usize,
    flag_speed: f32,
    flag_color: Option<OptColor>,
    flag_config: Option<String>,
    arg_input: String,
    arg_output: Option<String>,
}
//}}} CmdOptConf

//----------------------------------------------------------------------
//{{{ OptColor
#[derive(Debug, Deserialize)]
#[allow(non_camel_case_types)]
pub enum OptColor {
    always,
    never,
    auto,
}
//}}} OptColor

//----------------------------------------------------------------------
const USAGE: &str = concat!(
    "Usage:
  cmp_docopt [flags] [options] <input> [<output>]
  cmp_docopt (-h | --help)
  cmp_docopt (-V | --version)

Flags:
  -h --help       Prints help information
  -V --version    Prints version information
  -d --debug      Activate debug mode

Options:
  -s --speed=<speed>      Set speed [default: 42.0]
  --color=<when>          Use markers to highlight
                          <when> is 'always', 'never', or 'auto'
  -v --verbose=<verbose>  Verbose mode. -v2 is more verbose
  -c --config=<path>      Give a path string argument

Args:
  <input>     Input file
  <output>    Output file, stdout if not present
"
);

#[inline(never)]
pub fn parse_cmdopts(_program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let version = format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let args: CmdOptConf = Docopt::new(USAGE)
        .and_then(|d| {
            d.options_first(true)
                .help(true)
                .version(Option::Some(version))
                .argv(env_args)
                .deserialize()
        })
        .unwrap_or_else(|e| e.exit());
    //
    Ok(args)
}

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<String> = std::env::args().collect();
    //let program = env_args.remove(0);
    let program = env_args[0].clone();
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}
