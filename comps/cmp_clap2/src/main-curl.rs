fn main() {
    match cmp_clap2::curl::create_conf() {
        Ok(args) => {
            println!("{:?}", args);
        }
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
    std::process::exit(0);
}
