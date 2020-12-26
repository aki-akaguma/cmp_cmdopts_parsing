use commander::Commander;
use optcolorwhen::OptColorWhen;
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
pub fn parse_cmdopts(
    _program: &str,
    env_args: Vec<String>,
) -> anyhow::Result<CmdOptConf> {
    let cmd = Commander::new()
        .version(env!("CARGO_PKG_VERSION"))
        .exec(env!("CARGO_PKG_NAME"))
        .usage("[options] <input> [<output>]")
        .usage("(-h|--help|-V|--version)")
        //.usage_desc("Copy SOURCE to DEST, or multiple SOURCE(s) to DIRECTORY.")
        //.option("-V, --version", "Print version information", None)
        .option("-d, --debug", "Activate debug mode", None)
        .option_int(
            "--verbose [value]",
            "Verbose mode. --verbose 2 is more verbose",
            None,
        )
        .option_float("-s, --speed [value]", "Set speed", Option::Some(42.0))
        .option_str(
            "--color [when]",
            concat!(
                "Use markers to highlight. ",
                "[when] is \'always\', \'never\', or \'auto\'"
            ),
            Option::Some("auto".to_string()),
        )
        .option_str("-c, --config [path]", "Give a path string argument", None)
        .parse_list_or_exit(env_args);
    //
    if cmd.get("V").is_some() {
        cmd.print_version();
    }
    //
    let mut coa = CmdOptConf {
        opt_speed: 42.0,
        ..Default::default()
    };
    //
    if cmd.get("debug").is_some() {
        coa.flag_debug = true;
    }
    if let Some(i) = cmd.get_int("verbose") {
        coa.cnt_verbose = i as usize;
    }
    if let Some(f) = cmd.get_float("speed") {
        coa.opt_speed = f;
    }
    if let Some(s) = cmd.get_str("color") {
        coa.opt_color = match FromStr::from_str(s.as_str()) {
            Ok(color) => color,
            Err(err) => {
                return Err(From::from(OptParseError::invalid_option_argument(
                    "--color",
                    &err.to_string(),
                )));
            }
        }
    }
    if let Some(s) = cmd.get_str("config") {
        coa.opt_config = Some(s);
    }
    //
    Ok(coa)
}

pub fn parse_cmdopts_str(
    program: &str,
    env_args: Vec<&str>,
) -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<String> = env_args.iter().map(|s| s.to_string()).collect();
    parse_cmdopts(program, env_args)
}

pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let env_args: Vec<String> = std::env::args().collect();
    //let program = env_args.remove(0);
    let program = env_args[0].clone();
    //let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, env_args)
}
