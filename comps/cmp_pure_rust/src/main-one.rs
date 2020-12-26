//3456789a123456789b123456789c123456789d123456789e123456789f123456789g123456789h
//
// pure rust program arguments
// https://rustbyexample.com/std_misc/arg.html
// https://doc.rust-lang.org/book/second-edition/ch12-03-improving-error-handling-and-modularity.html
//

const TRY_HELP_MSG: &str = "Try --help for help.";

fn main() {
    match cmp_pure_rust::one::create_conf() {
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
