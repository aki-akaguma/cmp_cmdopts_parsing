use anyhow::Context;
use std::io::Read;
use std::io::Write;

pub fn update_file(sss: &String, file_path: &str) -> anyhow::Result<()> {
    let contents = {
        let mut contents = String::new();
        if let Ok(mut file) = std::fs::File::open(file_path) {
            file.read_to_string(&mut contents)
                .with_context(|| format!("could not read file `{}`", file_path))?;
        }
        contents
    };
    if contents != *sss {
        println!("update: {}", file_path);
        let mut file = std::fs::File::create(file_path)
            .with_context(|| format!("could not create file `{}`", file_path))?;
        write!(file, "{}", sss).with_context(|| format!("could not write file `{}`", file_path))?;
    }
    //
    Ok(())
}
