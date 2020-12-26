mod test_exe_stab_cat {
    use exec_target::args_from;
    use exec_target::exec_target;
    use exec_target::exec_target_with_in;
    const TARGET_EXE: &'static str = "target/debug/exe-stab-cat";
    //
    #[test]
    fn test_invalid_flag() {
        let oup = exec_target(TARGET_EXE, &["-x"]);
        assert_eq!(
            oup.stderr,
            "exe-mock-cat: invalid option -- \'x\'\nTry \'exe-mock-cat --help\' for more information.\n"
        );
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.status.success(), false);
    }
    #[test]
    fn test_valid_cat_in() {
        let oup = exec_target_with_in(TARGET_EXE, args_from(""), b"abcdefg\n" as &[u8]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcdefg\n");
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_valid_cat_in_empty() {
        let oup = exec_target_with_in(TARGET_EXE, &[] as &[&str], b"abcdefg\n");
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "abcdefg\n");
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE, &["--help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            concat!(
                "Usage: exe-mock-cat [OPTION]... [FILE]...\n",
                "Concatenate FILE(s) to standard output.\n\n",
                "With no FILE, or when FILE is -, read standard input.\n\n",
                "  -A, --show-all           equivalent to -vET\n",
                "  -b, --number-nonblank    number nonempty output lines, overrides -n\n",
                "  -e                       equivalent to -vE\n",
                "  -E, --show-ends          display $ at end of each line\n",
                "  -n, --number             number all output lines\n",
                "  -s, --squeeze-blank      suppress repeated empty output lines\n",
                "  -t                       equivalent to -vT\n",
                "  -T, --show-tabs          display TAB characters as ^I\n",
                "  -u                       (ignored)\n",
                "  -v, --show-nonprinting   use ^ and M- notation, except for LFD and TAB\n",
                "      --help     display this help and exit\n",
                "      --version  output version information and exit\n",
            )
        );
        assert_eq!(oup.status.success(), true);
    }
}

mod test_exe_stab_env {
    use exec_target::args_from;
    use exec_target::exec_target_with_env;
    use std::collections::HashMap;
    const TARGET_EXE: &'static str = "target/debug/exe-stab-env";
    //
    #[test]
    fn test_valid_env_env_by_hashmap() {
        let mut env: HashMap<String, String> = HashMap::new();
        env.insert("TEST_TEST_RUST_ENV".to_string(), "abcdef".to_string());
        //
        let oup = exec_target_with_env(TARGET_EXE, args_from(""), env);
        //
        assert_eq!(oup.stderr, "");
        match oup.stdout.find("TEST_TEST_RUST_ENV=abcdef\n") {
            Some(_) => {}
            None => assert!(false, "not found: TEST_TEST_RUST_ENV"),
        }
        match oup.stdout.find("LANG=C\n") {
            Some(_) => {}
            None => assert!(false, "not found: LANG"),
        }
        match oup.stdout.find("PATH=") {
            Some(_) => {}
            None => assert!(false, "not found: PATH"),
        }
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_valid_env_env_by_vec() {
        let oup = exec_target_with_env(
            TARGET_EXE,
            args_from(""),
            vec![("TEST_TEST_RUST_ENV", "abcdef")],
        );
        //
        assert_eq!(oup.stderr, "");
        match oup.stdout.find("TEST_TEST_RUST_ENV=abcdef\n") {
            Some(_) => {}
            None => assert!(false, "not found: TEST_TEST_RUST_ENV"),
        }
        match oup.stdout.find("LANG=C\n") {
            Some(_) => {}
            None => assert!(false, "not found: LANG"),
        }
        match oup.stdout.find("PATH=") {
            Some(_) => {}
            None => assert!(false, "not found: PATH"),
        }
        assert_eq!(oup.status.success(), true);
    }
}

mod test_exe_stab_grep {
    use exec_target::args_from;
    use exec_target::exec_target;
    use exec_target::exec_target_with_env_in;
    const TARGET_EXE: &'static str = "target/debug/exe-stab-grep";
    //
    #[test]
    fn test_valid_grep_color() {
        let oup = exec_target(
            TARGET_EXE,
            &["--color=always", "-e", "exec-target", "Cargo.toml"],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(
            oup.stdout,
            "name = \"\u{1b}[01;31m\u{1b}[Kexec-target\u{1b}[m\u{1b}[K\"\n"
        );
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_valid_grep() {
        let oup = exec_target(
            TARGET_EXE,
            &["--color=never", "-e", "exec-target", "Cargo.toml"],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "name = \"exec-target\"\n");
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_valid_grep_2() {
        let oup = exec_target(
            TARGET_EXE,
            args_from("--color=never -e exec-target Cargo.toml"),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "name = \"exec-target\"\n");
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_valid_grep_env_in() {
        let oup = exec_target_with_env_in(
            TARGET_EXE,
            &["--color=always", "-e", "c"],
            vec![("GREP_COLORS", "ms=01;32")],
            b"abcdefg\n" as &[u8],
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab\u{1b}[01;32m\u{1b}[Kc\u{1b}[m\u{1b}[Kdefg\n");
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_valid_grep_env_in_as_bytes() {
        let oup = exec_target_with_env_in(
            TARGET_EXE,
            &["--color=always", "-e", "c"],
            vec![("GREP_COLORS", "ms=01;32")],
            "abcdefg\n".as_bytes(),
        );
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, "ab\u{1b}[01;32m\u{1b}[Kc\u{1b}[m\u{1b}[Kdefg\n");
        assert_eq!(oup.status.success(), true);
    }
}
