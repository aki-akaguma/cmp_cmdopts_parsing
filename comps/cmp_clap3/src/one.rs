use clap::{App, Arg, ColorChoice};
use optcolorwhen::OptColorWhen;
use optcolorwhen::OptColorWhenParseError;
use optpaerr_a::OptParseError;
use std::str::FromStr;

//----------------------------------------------------------------------
//{{{ CmdOptConf
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
#[inline(never)]
pub fn parse_cmdopts(_program: &str, env_args: Vec<&str>) -> anyhow::Result<CmdOptConf> {
    let app = App::new(env!("CARGO_PKG_NAME"))
        .color(ColorChoice::Never)
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::with_name("debug")
                .short('d')
                .long("debug")
                .help("Activate debug mode"),
        )
        .arg(
            Arg::with_name("verbose")
                .short('v')
                .long("verbose")
                //.takes_value(true)
                //.multiple(true)
                .help("Verbose mode. -vv is more verbose"),
        )
        .arg(
            Arg::with_name("speed")
                .short('s')
                .long("speed")
                .value_name("speed")
                .help("Set speed")
                .default_value("42.0")
                .takes_value(true)
                .empty_values(false)
                .validator(|x| match x.parse::<f32>() {
                    Ok(_) => Ok(()),
                    Err(err) => Err(err.to_string()),
                }),
        )
        .arg(
            Arg::with_name("color")
                .long("color")
                .value_name("when")
                .help(concat!(
                    "Use markers to highlight\n",
                    "<when> is \'always\', \'never\',\n",
                    "or \'auto\'"
                ))
                .default_value("auto")
                .takes_value(true)
                .empty_values(false)
                .validator(|x| {
                    let res: Result<OptColorWhen, OptColorWhenParseError> = FromStr::from_str(x);
                    match res {
                        Ok(_) => Ok(()),
                        Err(err) => Err(err.to_string()),
                    }
                }),
        )
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("path")
                .help("Give a path string argument")
                .takes_value(true)
                .empty_values(false),
        )
        .arg(
            Arg::with_name("input")
                .help("Input file")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .help("Output file, stdout if not present")
                .required(false)
                .index(2),
        );
    let matches = app.get_matches_from(env_args);
    //
    let mut args = CmdOptConf::default();
    if matches.is_present("debug") {
        args.flag_debug = true;
    }
    args.cnt_verbose = matches.occurrences_of("verbose") as usize;
    args.opt_speed = match matches.value_of("speed") {
        Some(x) => x.parse::<f32>().unwrap(),
        None => 0.0,
    };
    args.opt_color = match matches.value_of("color") {
        Some(s) => match FromStr::from_str(s) {
            Ok(color) => color,
            Err(err) => {
                return Err(From::from(OptParseError::invalid_option_argument(
                    "--color",
                    &err.to_string(),
                )));
            }
        },
        None => OptColorWhen::Auto,
    };
    args.opt_config = matches.value_of("config").map(|s| s.to_string());
    args.arg_input = match matches.value_of("input") {
        Some(x) => x.to_string(),
        None => "".to_string(),
    };
    args.arg_output = matches.value_of("output").map(|s| s.to_string());
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
