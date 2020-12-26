fn main() {
    let args: Vec<String> = std::env::args().collect();
    let _program = &args[0];
    let args = &args[1..];
    //println!("{:?}", args);
    for a in args.iter() {
        match a.as_str() {
            "-x" => {
                let msg = concat!(
                    "exe-mock-cat: invalid option -- \'x\'\n",
                    "Try \'exe-mock-cat --help\' for more information.\n"
                );
                eprint_and_exit(msg);
            }
            "--help" => {
                print_help_and_exit();
            }
            _ => {}
        }
    }
    do_proc();
}

fn eprint_and_exit(msg: &str) {
    eprint!("{}", msg);
    std::process::exit(1);
}

fn print_help_and_exit() {
    let help_msg = concat!(
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
    );
    print!("{}", help_msg);
    std::process::exit(0);
}

fn do_proc() {
    let fin = std::io::stdin();
    let mut line = String::new();
    loop {
        match fin.read_line(&mut line) {
            Ok(n) if n > 0 => {
                print!("{}", line);
                line.clear();
            }
            _ => return,
        }
    }
}
