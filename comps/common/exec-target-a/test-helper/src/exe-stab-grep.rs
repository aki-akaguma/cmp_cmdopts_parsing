fn main() {
    let args: Vec<String> = std::env::args().collect();
    let _program = &args[0];
    let args = &args[1..];
    //println!("{:?}", args);
    if args_eq(
        &args,
        &["--color=always", "-e", "exec-target", "Cargo.toml"],
    ) {
        let msg = "name = \"\u{1b}[01;31m\u{1b}[Kexec-target\u{1b}[m\u{1b}[K\"\n";
        print_and_exit(msg);
    }
    if args_eq(&args, &["--color=never", "-e", "exec-target", "Cargo.toml"]) {
        let msg = "name = \"exec-target\"\n";
        print_and_exit(msg);
    }
    match std::env::var("GREP_COLORS") {
        Ok(ref s) if s.as_str() == "ms=01;32" => {
            if args_eq(&args, &["--color=always", "-e", "c"]) {
                let msg = "ab\u{1b}[01;32m\u{1b}[Kc\u{1b}[m\u{1b}[Kdefg\n";
                print_and_exit(msg);
            }
        }
        _ => (),
    }
    print_and_exit("not support\n");
}

fn args_eq(args: &[String], sm: &[&str]) -> bool {
    if args.len() < sm.len() {
        return false;
    }
    for i in 0..sm.len() {
        if args[i].as_str() != sm[i] {
            return false;
        }
    }
    return true;
}

fn print_and_exit(msg: &str) {
    print!("{}", msg);
    std::process::exit(0);
}
