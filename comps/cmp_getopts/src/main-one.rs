const TRY_HELP_MSG: &str = "Try --help for help.";

fn main() {
    match cmp_getopts::one::create_conf() {
        Ok(args) => {
            println!("{:?}", args);
        }
        Err(err) => {
            eprintln!("{}\n{}", err, TRY_HELP_MSG);
            std::process::exit(1);
        }
    }
    std::process::exit(0);
}
