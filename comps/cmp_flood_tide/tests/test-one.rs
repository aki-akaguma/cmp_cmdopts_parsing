use exec_target_a::exec_target;

const TARGET_EXE_PATH: &'static str = "../../target/debug/cmp_flood_tide-one";

macro_rules! help_msg {
    () => {
        concat!(
            "Usage: cmp_flood_tide [options] <input> [<output>]\n",
            "Options:\n",
            "    -h, --help          Print this help menu\n",
            "    -V, --version       Print version information\n",
            "    -d, --debug         Activate debug mode\n",
            "    -v, --verbose       Verbose mode. -vv is more verbose\n",
            "    -s, --speed <speed> Set speed (default: 42.0)\n",
            "    --color <when>      Use markers to highlight (default: auto)\n",
            "                        <when> is \'always\', \'never\', or \'auto\'\n",
            "    -c, --config <path> Give a path string argument\n",
            "Args:\n",
            "    <input>             Input file name\n",
            "    [<output>]          Output file name, stdout if not present\n"
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
        "cmp_flood_tide 0.1.1\n"
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
        concat!("Missing argument: <input>\n", try_help_msg!())
    );
}

#[test]
fn test_invalid_flag() {
    let oup = exec_target(TARGET_EXE_PATH, &["-x"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    assert_eq!(oup.stderr, concat!("Invalid option: x\n", try_help_msg!()));
}

#[test]
fn test_invalid_float() {
    let oup = exec_target(TARGET_EXE_PATH, &["-s", "12x"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    #[cfg(feature = "single_error")]
    assert_eq!(
        oup.stderr,
        concat!(
            "Invalid option argument: s: invalid float literal\n",
            try_help_msg!()
        )
    );
    #[cfg(not(feature = "single_error"))]
    assert_eq!(
        oup.stderr,
        concat!(
            "Invalid option argument: s: invalid float literal\n",
            "Missing argument: <input>\n",
            try_help_msg!()
        )
    );
}

#[test]
fn test_invalid_color() {
    let oup = exec_target(TARGET_EXE_PATH, &["--color", "nev"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    #[cfg(feature = "single_error")]
    assert_eq!(
        oup.stderr,
        concat!(
            "Invalid option argument: color: can not parse \'nev\'\n",
            try_help_msg!()
        )
    );
    #[cfg(not(feature = "single_error"))]
    assert_eq!(
        oup.stderr,
        concat!(
            "Invalid option argument: color: can not parse \'nev\'\n",
            "Missing argument: <input>\n",
            try_help_msg!()
        )
    );
}

#[test]
fn test_invalid_color2() {
    let oup = exec_target(TARGET_EXE_PATH, &["--color="]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    #[cfg(feature = "single_error")]
    assert_eq!(
        oup.stderr,
        concat!(
            "Invalid option argument: color: can not parse \'\'\n",
            try_help_msg!()
        )
    );
    #[cfg(not(feature = "single_error"))]
    assert_eq!(
        oup.stderr,
        concat!(
            "Invalid option argument: color: can not parse \'\'\n",
            "Missing argument: <input>\n",
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
    #[cfg(feature = "abbreviate")]
    {
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
    };
    #[cfg(not(feature = "abbreviate"))]
    {
        assert_eq!(oup.status.success(), false);
        assert_eq!(oup.stdout, "");
        #[cfg(feature = "single_error")]
        assert_eq!(
            oup.stderr,
            concat!("Invalid option: deb\n", try_help_msg!())
        );
        #[cfg(not(feature = "single_error"))]
        assert_eq!(
            oup.stderr,
            concat!(
                "Invalid option: deb\n",
                "Invalid option: verb\n",
                "Invalid option: verb\n",
                "Invalid option: sp\n",
                "Invalid option: col\n",
                "Invalid option: con\n",
                try_help_msg!()
            )
        );
    };
}

#[test]
fn test_ambiguous_options() {
    #[rustfmt::skip]
    let oup = exec_target(TARGET_EXE_PATH, &["--ver", "--co", "never", "inp", "oup"]);
    assert_eq!(oup.status.success(), false);
    assert_eq!(oup.stdout, "");
    #[cfg(feature = "abbreviate")]
    {
        #[cfg(feature = "single_error")]
        assert_eq!(
            oup.stderr,
            concat!(
                "Ambiguous option: ver: possibilities: \'--version\' \'--verbose\'\n",
                try_help_msg!()
            )
        );
        #[cfg(not(feature = "single_error"))]
        assert_eq!(
            oup.stderr,
            concat!(
                "Ambiguous option: ver: possibilities: \'--version\' \'--verbose\'\n",
                "Ambiguous option: co: possibilities: \'--color\' \'--config\'\n",
                try_help_msg!()
            )
        );
    }
    #[cfg(not(feature = "abbreviate"))]
    {
        #[cfg(feature = "single_error")]
        assert_eq!(
            oup.stderr,
            concat!("Invalid option: ver\n", try_help_msg!())
        );
        #[cfg(not(feature = "single_error"))]
        assert_eq!(
            oup.stderr,
            concat!(
                "Invalid option: ver\n",
                "Invalid option: co\n",
                try_help_msg!()
            )
        );
    }
}
