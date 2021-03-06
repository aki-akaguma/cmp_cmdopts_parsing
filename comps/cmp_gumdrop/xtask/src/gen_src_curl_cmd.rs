use optstr_curl::parse_input_file;
use optstr_curl::OptStr;

pub fn do_gen_src() -> anyhow::Result<()> {
    let (vec_optstr, vec_line) = parse_input_file("comps/common/optstr-curl/src/curl.cmd.txt")?;
    //
    let sss = do_gen_src_help(&vec_optstr, &vec_line)?;
    update_file::update_file(&sss, "comps/cmp_gumdrop/src/curl.cmd.help.rs.txt")?;
    //
    let sss = do_gen_src_match(&vec_optstr)?;
    update_file::update_file(&sss, "comps/cmp_gumdrop/src/curl.cmd.match.rs.txt")?;
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
#[derive(Options, Debug, Default)]
pub struct MyOptions {
    #[options(free)]
    free: Vec<String>,
    //
"#;
    for rec in vec_optstr.iter() {
        sss += "    #[options(\n";
        if rec.sho.is_empty() {
            sss += "        no_short,\n";
        } else {
            sss += &format!("        short = \"{}\",\n", rec.sho);
        }
        sss += &format!("        help = \"{}\",\n", rec._comment);
        if !rec.meta.is_empty() {
            sss += &format!("        meta = \"{}\",\n", rec.meta);
        }
        sss += "    )]\n";
        let gumdrop_field = if rec.field_s.starts_with("flg_") || rec.field_s.starts_with("opt_") {
            &rec.field_s[4..]
        } else {
            &rec.field_s
        };
        sss += &format!("    pub {}: {},\n", gumdrop_field, rec.type_s);
    }
    sss += r#"    //
}
"#;
    //
    Ok(sss)
}

fn do_gen_src_match(vec_optstr: &[OptStr]) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    //
    let s = r"// WARN: This file is auto generated by";
    sss += &format!("{} {}", s, env!("CARGO_PKG_NAME"));
    //
    sss += r#"
{
"#;
    for rec in vec_optstr.iter() {
        match rec.field_s.as_str() {
            "flg_help" => {
                sss += r#"
    if opt.help {
        print_help_and_exit(&conf);
    }
"#;
            }
            "flg_version" => {
                sss += r#"
    if opt.version {
        print_version_and_exit(&conf);
    }
"#;
            }
            _ => {
                let gumdrop_field =
                    if rec.field_s.starts_with("flg_") || rec.field_s.starts_with("opt_") {
                        &rec.field_s[4..]
                    } else {
                        &rec.field_s
                    };
                sss += &format!("    conf.{} = opt.{};\n", rec.field_s, gumdrop_field);
            }
        }
    }
    sss += r#"}
"#;
    //
    Ok(sss)
}
