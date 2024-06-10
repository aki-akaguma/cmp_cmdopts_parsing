use flood_tide::OptParseError;
#[cfg(not(feature = "single_error"))]
use flood_tide::OptParseErrors;

use flood_tide::Arg;
use flood_tide::NameVal;
use flood_tide::Opt;
use flood_tide::OptNum;

use flood_tide::parse_simple_gnu_style;
use flood_tide::HelpVersion;

//----------------------------------------------------------------------
include!("curl.cmd.help.rs.txt");

//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"
transfer a URL, another name is the multiprotocol getter
"#;

const ARGUMENTS_TEXT: &str = r#"Argument:
  <url>                     url to getting, protocol is http or ftp
"#;

const EXAMPLES_TEXT: &str = r#"Examples:
  You  can specify multiple URLs or parts of URLs by writing part sets within braces as in:
    curl "http://site.{one,two,three}.comn"
  you can get sequences of alphanumeric series by using [] as in:
    curl "ftp://ftp.example.com/file[1-100].txt"
"#;
//}}} TEXT

//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    format!("Usage:\n  {} {}", program, "[options] <url>")
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message("curl");
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT,
        ARGUMENTS_TEXT, EXAMPLES_TEXT].join("\n")
}

//----------------------------------------------------------------------
#[inline(never)]
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    include!("curl.cmd.match.rs.txt");
    Ok(())
}

pub fn parse_cmdopts(program: &str, args: &[&str]) -> anyhow::Result<CmdOptConf> {
    match parse_cmdopts_0(program, args) {
        Ok(a) => Ok(a),
        Err(e) => Err(From::from(e)),
    }
}

#[cfg(feature = "single_error")]
type OptErr = OptParseError;
#[cfg(not(feature = "single_error"))]
type OptErr = OptParseErrors;

pub fn parse_cmdopts_0(a_prog_name: &str, args: &[&str]) -> Result<CmdOptConf, OptErr> {
    //
    let mut conf = CmdOptConf {
        prog_name: a_prog_name.to_string(),
        ..Default::default()
    };
    let (opt_free, r_errs) =
        parse_simple_gnu_style(&mut conf, &OPT_ARY, &OPT_ARY_SHO_IDX, args, parse_match);
    //
    #[cfg(feature = "single_error")]
    {
        if conf.is_help() {
            return Err(OptParseError::help_message(&help_message(&conf.prog_name)));
        }
        if conf.is_version() {
            return Err(OptParseError::version_message(&version_message(
                &conf.prog_name,
            )));
        }
    }
    #[cfg(not(feature = "single_error"))]
    {
        if conf.is_help() {
            let mut errs = OptParseErrors::new();
            errs.push(OptParseError::help_message(&help_message(&conf.prog_name)));
            return Err(errs);
        }
        if conf.is_version() {
            let mut errs = OptParseErrors::new();
            errs.push(OptParseError::version_message(&version_message(
                &conf.prog_name,
            )));
            return Err(errs);
        }
    }
    //
    #[cfg(feature = "single_error")]
    {
        r_errs?;
        if let Some(free) = opt_free {
            if !free.is_empty() {
                conf.arg_params = free;
            } else {
                return Err(OptParseError::missing_argument("<url>"));
            }
        }
    }
    #[cfg(not(feature = "single_error"))]
    {
        let mut errs = if let Err(errs) = r_errs {
            errs
        } else {
            OptParseErrors::new()
        };
        let missing_arg = if let Some(free) = opt_free {
            if !free.is_empty() {
                conf.arg_params = free;
                false
            } else {
                true
            }
        } else {
            true
        };
        if missing_arg {
            errs.push(OptParseError::missing_argument("<url>"));
        }
        if !errs.is_empty() {
            return Err(From::from(errs));
        }
    }
    //
    Ok(conf)
}

//----------------------------------------------------------------------
pub fn create_conf() -> anyhow::Result<CmdOptConf> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    let env_args: Vec<&str> = env_args.iter().map(std::string::String::as_str).collect();
    parse_cmdopts(&program, &env_args)
}

#[cfg(test)]
mod test {
    use flood_tide::check;

    #[test]
    fn check() {
        assert!(check::check_sorted_opt_ary_and_sho_idx_ary_with(
            &super::OPT_ARY,
            &super::OPT_ARY_SHO_IDX
        ));
    }
}
