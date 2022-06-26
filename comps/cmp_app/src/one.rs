use app::{App, Args, Opt, OptTypo};
use optcolorwhen::OptColorWhen;
use optcolorwhen::OptColorWhenParseError;

use std::str::FromStr;

//----------------------------------------------------------------------
//{{{ CmdOptConf
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct CmdOptConf {
    flag_debug: bool,
    cnt_verbose: usize,
    opt_speed: f32,
    opt_color: OptColorApp,
    opt_config: Option<String>,
    arg_input: Vec<String>,
    arg_output: Vec<String>,
    /*
    arg_input: String,
    arg_output: Option<String>,
    */
}
//}}} CmdOptConf

//----------------------------------------------------------------------
//{{{ OptColorApp
#[derive(Debug)]
enum OptColorApp {
    OptColorWhen(OptColorWhen),
}

impl Default for OptColorApp {
    fn default() -> OptColorApp {
        OptColorApp::OptColorWhen(OptColorWhen::Auto)
    }
}

impl ::std::str::FromStr for OptColorApp {
    type Err = OptColorWhenParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r: Result<OptColorWhen, OptColorWhenParseError> = FromStr::from_str(s);
        match r {
            Ok(c) => Ok(OptColorApp::OptColorWhen(c)),
            Err(e) => Err(e),
        }
    }
}

impl ::std::fmt::Display for OptColorApp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        let s = match *self {
            OptColorApp::OptColorWhen(ref c) => format!("{}", c),
        };
        write!(f, "{}", s)
    }
}

impl From<OptColorApp> for OptColorWhen {
    fn from(w: OptColorApp) -> OptColorWhen {
        match w {
            OptColorApp::OptColorWhen(c) => c,
        }
    }
}

impl<'app, 's: 'app> app::OptValueParse<'app> for &'s mut OptColorApp {
    fn into(self) -> app::OptValue<'app> {
        app::OptValue::new(Box::from(self))
    }
    fn is_bool(&self) -> bool {
        false
    }
    fn default(&self) -> Option<String> {
        Option::Some(String::from("auto"))
    }
    fn parse(
        &mut self,
        opt_name: &str,
        msg: &str,
        count: &mut usize,
        typo: &mut OptTypo,
    ) -> Result<(), String> {
        if *count == 0 || typo.is_covered() || typo.is_multiple() {
            **self = msg.trim().parse::<OptColorApp>().map_err(|_| {
                format!(
                    "OPTION(<{}>) parse<{}> fails: \"{}\"",
                    opt_name,
                    stringify!(OptColor),
                    msg
                )
            })?;
        } else if typo.is_single() {
            return Err(format!(
                "OPTION(<{}>) can only occurs once, but second: {:?}",
                opt_name, msg
            ));
        }
        Ok(())
    }
    fn check(
        &self,
        opt_name: &str,
        optional: &bool,
        count: &usize,
        _typo: &OptTypo,
    ) -> Result<(), String> {
        if !optional && *count == 0 && self.default().is_none() {
            return Err(format!("OPTION(<{}>) missing", opt_name));
        }
        Ok(())
    }
}
//}}} OptColorApp

//----------------------------------------------------------------------
#[inline(never)]
pub fn parse_cmdopts(_program: &str, env_args: &[String]) -> anyhow::Result<CmdOptConf> {
    let mut args = CmdOptConf {
        opt_speed: 42.0,
        opt_color: OptColorApp::OptColorWhen(OptColorWhen::Auto),
        ..Default::default()
    };
    let app = {
        App::new(env!("CARGO_PKG_NAME"))
            .version(env!("CARGO_PKG_VERSION"))
            /*
            .opt(Opt::new("help", &mut args.flag_help)
                           .short('h')
                           .long("help")
                           .help("print this help menu"))
            .opt(Opt::new("version", &mut args.flag_version)
                           .short('V')
                           .long("version")
                           .help("print version information"))
            */
            .opt(
                Opt::new("debug", &mut args.flag_debug)
                    .short('d')
                    .long("debug")
                    .typo(OptTypo::Single)
                    .help("Activate debug mode"),
            )
            .opt(
                Opt::new("verbose", &mut args.cnt_verbose)
                    .short('v')
                    .long("verbose")
                    .help("Verbose mode. -v2 is more verbose"),
            )
            .opt(
                Opt::new("speed", &mut args.opt_speed)
                    .short('s')
                    .long("speed")
                    .help("Set speed"),
            )
            .opt(
                Opt::new("when", &mut args.opt_color)
                    .long("color")
                    .help(concat!(
                        "Use markers to highlight\n",
                        "<when> is \'always\', \'never\', or \'auto\'"
                    )),
            )
            .opt(
                Opt::new("config", &mut args.opt_config)
                    .optional()
                    .short('c')
                    .long("config")
                    .help("Give a path string argument"),
            )
            .args(
                Args::new("input", &mut args.arg_input)
                    .len(1usize)
                    .help("Input file"),
            )
            .args(
                Args::new("output", &mut args.arg_output)
                    .len(1usize)
                    .optional()
                    .help("Output file, stdout if not present"),
            )
            .build_helper()
    };
    let _ = app.parse(env_args);
    //
    Ok(args)
}

#[rustfmt::skip]
#[inline(never)]
pub fn parse_cmdopts_str(program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<String> = env_args.iter().map(|s| s.to_string()).collect();
    parse_cmdopts(program, &env_args)
}

#[rustfmt::skip]
pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    parse_cmdopts(&program, &env_args)
}
