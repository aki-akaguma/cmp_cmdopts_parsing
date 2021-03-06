use optstr_curl::parse_input_file;
use optstr_curl::OptStr;

pub fn do_gen_src() -> anyhow::Result<()> {
    let (vec_optstr, vec_line) = parse_input_file("comps/common/optstr-curl/src/curl.cmd.txt")?;
    //
    let sss = do_gen_src_help(&vec_optstr, &vec_line)?;
    update_file::update_file(&sss, "comps/cmp_structopt/src/curl.cmd.help.rs.txt")?;
    //
    let sss = do_gen_src_match(&vec_optstr, &vec_line)?;
    update_file::update_file(&sss, "comps/cmp_structopt/src/curl.cmd.match.rs.txt")?;
    //
    Ok(())
}

fn do_gen_src_help(vec_optstr: &[OptStr], _vec_line: &[String]) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    //
    let s = r"// WARN: This file is auto generated by";
    sss += &format!("{} {}", s, env!("CARGO_PKG_NAME"));
    sss += r#"
"#;
    //
    sss += r#"
#[derive(Debug, Default, PartialEq)]
pub struct CmdOptConf {
    pub opt_program: String,
    //
"#;
    for rec in vec_optstr.iter() {
        sss += &format!("    pub {}: {},\n", rec.field_s, rec.type_s);
    }
    sss += r#"    //
    pub arg_params: Vec<String>,
}
"#;
    //
    sss += r#"
#[derive(StructOpt, Debug)]
#[structopt()]
pub struct MyOptions {
"#;
    for rec in vec_optstr.iter() {
        if rec.enum_s == "Help" {
            continue;
        }
        if rec.enum_s == "Version" {
            continue;
        }
        let sholon = if !rec.lon.is_empty() {
            rec.lon.clone()
        } else {
            rec.sho.clone()
        };
        sss += "    #[structopt(\n";
        sss += &format!("        name = \"{}\",\n", sholon);
        if !rec.sho.is_empty() {
            sss += &format!("        short = \"{}\",\n", rec.sho);
        }
        sss += &format!("        long = \"{}\",\n", rec.lon);
        sss += &format!("        help = \"{}\",\n", rec._comment);
        sss += "    )]\n";
        if rec.type_s == "bool" {
            sss += &format!("    pub {}: bool,\n", rec.field_s);
        } else {
            sss += &format!("    pub {}: Option<{}>,\n", rec.field_s, rec.type_s);
        }
    }
    sss += r#"
    #[structopt(name = "URL", help = "url")]
    arg_input: String,
}
"#;
    //
    Ok(sss)
}

fn do_gen_src_match(vec_optstr: &[OptStr], _vec_line: &[String]) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    //
    let s = r"// WARN: This file is auto generated by";
    sss += &format!("{} {}", s, env!("CARGO_PKG_NAME"));
    sss += r#"
{
"#;
    //
    for rec in vec_optstr.iter() {
        if rec.enum_s == "Help" {
            continue;
        }
        if rec.enum_s == "Version" {
            continue;
        }
        if rec.type_s == "bool" {
            sss += &format!("        conf.{} = opts.{};\n", rec.field_s, rec.field_s);
        } else {
            sss += &format!("    if let Some(v) = opts.{} {{\n", rec.field_s);
            sss += &format!("        conf.{} = v;\n", rec.field_s);
            sss += "    }\n";
        }
    }
    sss += r#"
}
"#;
    //
    Ok(sss)
}
