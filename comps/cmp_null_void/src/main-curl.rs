const TRY_HELP_MSG: &str = "Try --help for help.";

fn main() {
    match cmp_null_void::curl::create_conf() {
        Ok(args) => {
            println!("{:?}", args);
        }
        Err(errs) => {
            eprintln!("{}\n{}", errs, TRY_HELP_MSG);
            std::process::exit(1);
        }
    }
    std::process::exit(0);
}
