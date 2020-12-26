// build.rs

use rustc_version::version;

fn main() {
    let ver = version().unwrap();
    //
    //println!("{:?}", ver);
    assert!(ver.major >= 1);
    if ver.major == 1 {
        if ver.minor < 19 {
            // < 1.19.0
            println!("cargo:rustc-cfg=feature=\"{}\"", "lower_1_19_0");
        }
    }
}
