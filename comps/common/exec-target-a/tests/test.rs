mod test_args {
    use exec_target::args_from;
    //
    #[test]
    fn test_args_from() {
        let v = args_from("--color=never -e exec-target  Cargo.toml");
        assert_eq!(v[0], "--color=never");
        assert_eq!(v[1], "-e");
        assert_eq!(v[2], "exec-target");
        assert_eq!(v[3], "Cargo.toml");
    }
    //
    #[test]
    fn test_args_from_q() {
        let v = args_from("-e 'exec-target'  Cargo.toml");
        assert_eq!(v[0], "-e");
        assert_eq!(v[1], "exec-target");
        assert_eq!(v[2], "Cargo.toml");
    }
    //
    #[test]
    fn test_args_from_q2() {
        let v = args_from("-e 'exec_\"target'  Cargo.toml");
        assert_eq!(v[0], "-e");
        assert_eq!(v[1], "exec_\"target");
        assert_eq!(v[2], "Cargo.toml");
    }
    //
    #[test]
    fn test_args_from_qq() {
        let v = args_from("-e \"exec-target\"  Cargo.toml");
        assert_eq!(v[0], "-e");
        assert_eq!(v[1], "exec-target");
        assert_eq!(v[2], "Cargo.toml");
    }
    //
    #[test]
    fn test_args_from_qq2() {
        let v = args_from("-e \"exec_'target\"  Cargo.toml");
        assert_eq!(v[0], "-e");
        assert_eq!(v[1], "exec_'target");
        assert_eq!(v[2], "Cargo.toml");
    }
    //
    #[test]
    fn test_args_from_back_slash() {
        let v = args_from("-e exec_\\\\target  Cargo.toml");
        assert_eq!(v[0], "-e");
        assert_eq!(v[1], "exec_\\target");
        assert_eq!(v[2], "Cargo.toml");
    }
}
