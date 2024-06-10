use optcolorwhen::OptColorWhen;
use optcolorwhen::OptColorWhenParseError;
use rustop::*;
use std::str::FromStr;

//----------------------------------------------------------------------
//{{{ CmdOptConf
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CmdOptConf {
    flag_debug: bool,
    cnt_verbose: usize,
    opt_speed: f32,
    opt_color: OptColorWhen,
    opt_config: Option<String>,
    arg_input: String,
    arg_output: Option<String>,
}
//}}} CmdOptConf

//----------------------------------------------------------------------
//{{{ OptColorRustop
enum OptColorRustop {
    OptColorWhen(OptColorWhen),
}

impl Default for OptColorRustop {
    fn default() -> OptColorRustop {
        OptColorRustop::OptColorWhen(OptColorWhen::Auto)
    }
}

impl ::std::str::FromStr for OptColorRustop {
    type Err = OptColorWhenParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r: Result<OptColorWhen, OptColorWhenParseError> = FromStr::from_str(s);
        match r {
            Ok(c) => Ok(OptColorRustop::OptColorWhen(c)),
            Err(e) => Err(e),
        }
    }
}

impl ::std::fmt::Display for OptColorRustop {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let s = match *self {
            OptColorRustop::OptColorWhen(ref c) => format!("{}", c),
        };
        write!(f, "{}", s)
    }
}

impl From<OptColorRustop> for OptColorWhen {
    fn from(w: OptColorRustop) -> OptColorWhen {
        match w {
            OptColorRustop::OptColorWhen(c) => c,
        }
    }
}
impl rustop::DefaultName for OptColorRustop {
    fn default_name() -> Option<&'static str> {
        Some("<s>")
    }
}
//}}} OptColorRustop

//----------------------------------------------------------------------
#[rustfmt::skip]
#[inline(never)]
pub fn parse_cmdopts(_program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let a = opts! {
        auto_shorts     false;
        command_name    env!("CARGO_PKG_NAME");
        help            true;
        //
        opt version:bool,       short:'V', long:"version",
            desc:"Print version information";
        opt debug:bool,         short:'d', long:"debug",
            desc:"Activate debug mode";
        opt verbose:usize=0,    short:'v', long:"verbose", name:"<level>",
            desc:"Verbose level. -v2 is more verbose";
        opt speed:f32=42.0,     short:'s', long:"speed", name:"<speed>",
            desc:"Set speed";
        opt color:OptColorRustop=OptColorRustop::OptColorWhen(OptColorWhen::Auto),
            long:"color", name:"<when>",
            desc:"Use markers to highlight\n<when> is \'always\', \'never\', or \'auto\'";
        opt config:Option<String>,     short:'c', long:"config", name:"<path>",
            desc:"Give a path string argument";
        //
        param input:String,             name:"<input>",
            desc:"Input file name";
        param output:Option<String>,    name:"<output>",
            desc:"Output file name, stdout if not present";
    //}.parse_or_exit();
    };
    let (args, _) = match a.parse_args(env_args) {
        Ok(a) => a,
        Err(rustop::Error::Help(msg)) => {
            eprintln!("{}", msg);
            std::process::exit(1);
        },
        Err(err) => rustop::error_and_exit(&err),
    };
    //
    if args.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }
    //
    Ok(CmdOptConf {
        flag_debug: args.debug,
        cnt_verbose: args.verbose,
        opt_speed: args.speed,
        opt_color: From::from(args.color),
        opt_config: args.config,
        arg_input: args.input,
        arg_output: args.output,
    })
}

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let _program = env_args.remove(0);
    let program = env!("CARGO_PKG_NAME");
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(program, env_args)
}
