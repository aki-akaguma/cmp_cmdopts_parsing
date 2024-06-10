use exec_target_a::exec_target;

const TARGET_EXE_PATH: &'static str = "../../target/debug/cmp_clap-one";

macro_rules! help_msg {
    () => {
        concat!(
            "cmp_clap 0.1.1\n\n",
            "USAGE:\n",
            "    cmp_clap-one [FLAGS] [OPTIONS] <input> [output]\n\n",
            "FLAGS:\n",
            "    -d, --debug      Activate debug mode\n",
            "    -h, --help       Prints help information\n",
            "    -V, --version    Prints version information\n",
            "    -v, --verbose    Verbose mode. -vv is more verbose\n\n",
            "OPTIONS:\n",
            "        --color <when>     Use markers to highlight\n",
            "                           <when> is \'always\', \'never\',\n",
            "                           or \'auto\' [default: auto]\n",
            "    -c, --config <path>    Give a path string argument\n",
            "    -s, --speed <speed>    Set speed [default: 42.0]\n\n",
            "ARGS:\n",
            "    <input>     Input file\n",
            "    <output>    Output file, stdout if not present\n"
        )
    };
}

macro_rules! try_help_msg {
    () => {
        "For more information try --help\n"
    };
}

macro_rules! version_msg {
    () => {
        "cmp_clap 0.1.1\n"
    };
}

#[test]
fn test_no_options() {
    let oup = exec_target(TARGET_EXE_PATH, &["inp"]);
    assert_eq!(oup.status.success(), true);
    assert_eq!(
        oup.stdout,
        concat!(
            "CmdOptConf {",
            " flag_debug: false,",
            " cnt_verbose: 0,",
            " opt_speed: 42.0,",
            " opt_color: Auto,",
            " opt_config: None,",
            " arg_input: \"inp\",",
            " arg_output: None",
            " }\n"
        )
    );
    assert_eq!(oup.stderr, "");
}

#[test]
fn test_full_options() {
    #[rustfmt::skip]
    let oup = exec_target(
        TARGET_EXE_PATH,
        &["-d", "-vv", "-s", "123", "-c", "file.conf", "inp", "oup"],
    );
    assert_eq!(oup.status.success(), true);
    assert_eq!(
        oup.stdout,
        concat!(
            "CmdOptConf {",
            " flag_debug: true,",
            " cnt_verbose: 2,",
            " opt_speed: 123.0,",
            " opt_color: Auto,",
            " opt_config: Some(\"file.conf\"),",
            " arg_input: \"inp\",",
            " arg_output: Some(\"oup\")",
            " }\n"
        )
    );
    assert_eq!(oup.stderr, "");
}

#[test]
fn test_full_options_long() {
    #[rustfmt::skip]
    let oup = exec_target(
        TARGET_EXE_PATH,
        &[ "--debug", "--verbose", "--verbose", "--speed", "123", "--color",
            "never", "--config", "dir/file.conf", "inp", "oup" ],
    );
    assert_eq!(oup.status.success(), true);
    assert_eq!(
        oup.stdout,
        concat!(
            "CmdOptConf {",
            " flag_debug: true,",
            " cnt_verbose: 2,",
            " opt_speed: 123.0,",
            " opt_color: Never,",
            " opt_config: Some(\"dir/file.conf\"),",
            " arg_input: \"inp\",",
            " arg_output: Some(\"oup\")",
            " }\n"
        )
    );
    assert_eq!(oup.stderr, "");
}

#[test]
fn test_help() {
    let oup = exec_target(TARGET_EXE_PATH, &["-h"]);
    assert_eq!(oup.status.success(), true);
    assert_eq!(oup.stdout, help_msg!());
    assert_eq!(oup.stderr, "");
}

#[test]
fn test_help_long() {
    let oup = exec_target(TARGET_EXE_PATH, &["--help"]);
    assert_eq!(oup.status.success(), true);
    assert_eq!(oup.stdout, help_msg!());
    assert_eq!(oup.stderr, "");
}

#[test]
fn test_version() {
    let oup = exec_target(TARGET_EXE_PATH, &["-V"]);
    assert_eq!(oup.status.success(), true);
    assert_eq!(oup.stdout, version_msg!());
    assert_eq!(oup.stderr, "");
}

#[test]
fn test_version_long() {
    let oup = exec_target(TARGET_EXE_PATH, &["--version"]);
    assert_eq!(oup.status.success(), true);
    assert_eq!(oup.stdout, version_msg!());
    assert_eq!(oup.stderr, "");
}

#[test]
fn test_void_args() {
    let args: [&str; 0] = [];
    let oup = exec_target(TARGET_EXE_PATH, &args);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "error: The following required arguments were not provided:\n",
            "    <input>\n\n",
            "USAGE:\n",
            "    cmp_clap-one <input> --color <when> --speed <speed>\n\n",
            try_help_msg!()
        )
    );
}

#[test]
fn test_invalid_flag() {
    let oup = exec_target(TARGET_EXE_PATH, &["-x", "inp"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "error: Found argument \'-x\' which wasn\'t expected,",
            " or isn\'t valid in this context\n\n",
            "USAGE:\n",
            "    cmp_clap-one [FLAGS] [OPTIONS] <input> [output]\n\n",
            try_help_msg!()
        )
    );
}

#[test]
fn test_invalid_float() {
    let oup = exec_target(TARGET_EXE_PATH, &["-s", "12x", "inp"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!("error: Invalid value for \'--speed <speed>\': invalid float literal\n",)
    );
}

#[test]
fn test_invalid_color() {
    let oup = exec_target(TARGET_EXE_PATH, &["--color", "nev", "inp"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!("error: Invalid value for \'--color <when>\': can not parse \'nev\'\n",)
    );
}

#[test]
fn test_invalid_color2() {
    let oup = exec_target(TARGET_EXE_PATH, &["--color=", "inp"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "error: The argument \'--color <when>\' requires a value but none was supplied\n\n",
            "USAGE:\n",
            "    cmp_clap-one [FLAGS] [OPTIONS] <input> [output]\n\n",
            try_help_msg!()
        )
    );
}

#[test]
fn test_abbreviate_options() {
    #[rustfmt::skip]
    let oup = exec_target(
        TARGET_EXE_PATH,
        &[ "--deb", "--verb", "--verb", "--sp", "123", "--col",
            "never", "--con", "dir/file.conf", "inp", "oup", ],
    );
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "error: Found argument \'--deb\' which wasn\'t expected, or isn\'t valid in this context\n",
            "\tDid you mean \u{1b}[32m--\u{1b}[0m\u{1b}[32mdebug\u{1b}[0m?\n\n",
            "USAGE:\n",
            "    cmp_clap-one <input> --debug\n\n", try_help_msg!())
    );
}

#[test]
fn test_ambiguous_options() {
    #[rustfmt::skip]
    let oup = exec_target(TARGET_EXE_PATH, &["--ver", "--co", "never", "inp", "oup"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "error: Found argument \'--ver\' which wasn\'t expected, or isn\'t valid in this context\n",
            "\tDid you mean \u{1b}[32m--\u{1b}[0m\u{1b}[32mverbose\u{1b}[0m?\n\n",
            "USAGE:\n",
            "    cmp_clap-one <input> --verbose\n\n", try_help_msg!())
    );
}
