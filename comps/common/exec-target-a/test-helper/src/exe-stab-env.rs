fn main() {
    let args: Vec<String> = std::env::args().collect();
    let _program = &args[0];
    let _args = &args[1..];
    //println!("{:?}", args);
    do_proc();
}

fn do_proc() {
    for (key, value) in std::env::vars() {
        println!("{}={}", key, value);
    }
}
