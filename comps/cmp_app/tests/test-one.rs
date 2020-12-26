use exec_target_a::exec_target;

const TARGET_EXE_PATH: &'static str = "../../target/debug/cmp_app-one";

macro_rules! help_msg {
  () => {
    concat!(
        "cmp_app 0.1.1\n\n",
        "USAGE:\n",
        "   cmp_app [options] <input> [<output>]\n\n",
        "OPTIONS:\n",
        "   -h, --help                            Show the help message\n",
        "   -V, --version                         Show the version message\n",
        "   -c, --config <config>(optional)       Give a path string argument\n",
        "   -d, --debug                           Activate debug mode\n",
        "   -s, --speed <speed>[42]               Set speed\n",
        "   -v, --verbose <verbose>[0]            Verbose mode. -v2 is more verbose\n",
        "   --color <when>[auto]                  Use markers to highlight\n",
        "                                         <when> is \'always\', \'never\', or \'auto\'\n\n",
        "ARGS:\n",
        "   <input>                Input file\n",
        "   <output>(optional)     Output file, stdout if not present\n"
    )
  }
}

macro_rules! version_msg {
    () => {
        "cmp_app 0.1.1\n"
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
            " opt_color: OptColorWhen(Auto),",
            " opt_config: None,",
            " arg_input: [\"inp\"],",
            " arg_output: []",
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
            " opt_color: OptColorWhen(Auto),",
            " opt_config: Some(\"file.conf\"),",
            " arg_input: [\"inp\"],",
            " arg_output: [\"oup\"]",
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
            "never", "--config", "dir/file.conf", "inp", "oup", ],
    );
    assert_eq!(oup.status.success(), true);
    assert_eq!(
        oup.stdout,
        concat!(
            "CmdOptConf {",
            " flag_debug: true,",
            " cnt_verbose: 2,",
            " opt_speed: 123.0,",
            " opt_color: OptColorWhen(Never),",
            " opt_config: Some(\"dir/file.conf\"),",
            " arg_input: [\"inp\"],",
            " arg_output: [\"oup\"]",
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
            "ERROR:\n",
            "\u{1b}[31m   Args(<input>) not provide\u{1b}(B\u{1b}[m\n\n",
            help_msg!()
        )
    );
}

#[test]
fn test_invalid_flag() {
    let oup = exec_target(TARGET_EXE_PATH, &["-x"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "ERROR:\n",
            "\u{1b}[31m   OPTION: \"-x\" is undefined\u{1b}(B\u{1b}[m\n\n",
            help_msg!()
        )
    );
}

#[test]
fn test_invalid_float() {
    let oup = exec_target(TARGET_EXE_PATH, &["-s", "12x"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "ERROR:\n",
            "\u{1b}[31m   OPTION(<speed>) parse<f32> fails: \"12x\"\u{1b}(B\u{1b}[m\n\n",
            help_msg!()
        )
    );
}

#[test]
fn test_invalid_color() {
    let oup = exec_target(TARGET_EXE_PATH, &["--color", "nev"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(
        oup.stderr,
        concat!(
            "ERROR:\n",
            "\u{1b}[31m   OPTION(<when>) parse<OptColor> fails: \"nev\"\u{1b}(B\u{1b}[m\n\n",
            help_msg!()
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
            "ERROR:\n",
            "\u{1b}[31m   OPTION: \"--color=\" is undefined\u{1b}(B\u{1b}[m\n\n",
            help_msg!()
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
            "ERROR:\n",
            "\u{1b}[31m   OPTION: \"--deb\" is undefined\u{1b}(B\u{1b}[m\n\n",
            help_msg!()
        )
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
            "ERROR:\n",
            "\u{1b}[31m   OPTION: \"--ver\" is undefined\u{1b}(B\u{1b}[m\n\n",
            help_msg!()
        )
    );
}
