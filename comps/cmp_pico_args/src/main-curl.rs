const TRY_HELP_MSG: &str = "Try --help for help.";

fn main() {
    //
    match cmp_pico_args::curl::create_conf() {
        Ok(conf) => {
            println!("{:?}", conf);
        }
        Err(err) => {
            eprintln!("{}\n{}", err, TRY_HELP_MSG);
            std::process::exit(1);
        }
    };
    //
    std::process::exit(0);
}
