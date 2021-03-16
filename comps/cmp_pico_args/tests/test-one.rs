use exec_target_a::exec_target;

const TARGET_EXE_PATH: &'static str = "../../target/debug/cmp_pico_args-one";

macro_rules! help_msg {
    () => {
        concat!(
            "Usage: cmp_pico_args [options] <input> [<output>]\n",
            "\n",
            "Options:\n",
            "    -h, --help               Print this help menu\n",
            "    -V, --version            Print version information\n",
            "    -d, --debug              Activate debug mode\n",
            "    -v, --verbose <integer>  Verbose mode. -v2 is more verbose\n",
            "    -s, --speed <speed>      Set speed (default: 42.0)\n",
            "    --color <when>           Use markers to highlight (default: auto)\n",
            "                             <when> is \'always\', \'never\', or \'auto\'\n",
            "    -c, --config <path>      Give a path string argument\n",
            "Args:\n",
            "    <input>             Input file name\n",
            "    [<output>]          Output file name, stdout if not present",
            "\n",
        )
    };
}

macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

macro_rules! version_msg {
    () => {
        "cmp_pico_args 0.1.0\n"
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
        &[ "-d", "-v", "2", "-s", "123", "-c", "file.conf", "inp", "oup" ],
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
        &[ "--debug", "--verbose", "2", "--speed", "123", "--color",
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
    let oup = exec_target(TARGET_EXE_PATH, &["-V", "inp"]);
    assert_eq!(oup.status.success(), true);
    assert_eq!(oup.stdout, version_msg!());
    assert_eq!(oup.stderr, "");
}

#[test]
fn test_version_long() {
    let oup = exec_target(TARGET_EXE_PATH, &["--version", "inp"]);
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
        concat!("Missing argument: <input>\n", try_help_msg!())
    );
}
/*
#[test]
fn test_invalid_flag() {
    let oup = exec_target(TARGET_EXE_PATH, &["-x", "inp"]);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!("unused arguments left: -x\n", try_help_msg!())
    );
    assert_eq!(oup.status.success(), false);
}
*/
#[test]
fn test_invalid_float() {
    let oup = exec_target(TARGET_EXE_PATH, &["-s", "12x", "inp"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "failed to parse \'12x\' cause invalid float literal\n",
            try_help_msg!()
        )
    );
}

#[test]
fn test_invalid_color() {
    let oup = exec_target(TARGET_EXE_PATH, &["--color", "nev", "inp"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "failed to parse \'nev\' cause can not parse \'nev\'\n",
            try_help_msg!()
        )
    );
}

#[test]
fn test_invalid_color2() {
    let oup = exec_target(TARGET_EXE_PATH, &["--color="]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "the \'--color\' option doesn\'t have an associated value\n",
            try_help_msg!()
        )
    );
}
/*
#[test]
fn test_abbreviate_options() {
    #[rustfmt::skip]
    let oup = exec_target(
        TARGET_EXE_PATH,
        &[ "--deb", "--verb", "--verb", "--sp", "123", "--col",
            "never", "--con", "dir/file.conf", "inp", "oup", ],
    );
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "unused arguments left: --deb, --verb, --verb, --sp, --col, --con\n",
            try_help_msg!()
        )
    );
    assert_eq!(oup.status.success(), false);
}
*/
/*
#[test]
fn test_ambiguous_options() {
    #[rustfmt::skip]
    let oup = exec_target(TARGET_EXE_PATH, &["--ver", "--co", "never", "inp", "oup"]);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!("unused arguments left: --ver, --co\n", try_help_msg!())
    );
    assert_eq!(oup.status.success(), false);
}
*/
