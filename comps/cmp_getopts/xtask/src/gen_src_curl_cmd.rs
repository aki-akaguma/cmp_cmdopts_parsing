use optstr_curl::parse_input_file;
use optstr_curl::OptStr;

pub fn do_gen_src() -> anyhow::Result<()> {
    let (vec_optstr, vec_line) = parse_input_file("comps/common/optstr-curl/src/curl.cmd.txt")?;
    //
    let sss = do_gen_src_help(&vec_optstr, &vec_line)?;
    update_file::update_file(&sss, "comps/cmp_getopts/src/curl.cmd.help.rs.txt")?;
    //
    let sss = do_gen_src_match(&vec_optstr)?;
    update_file::update_file(&sss, "comps/cmp_getopts/src/curl.cmd.match.rs.txt")?;
    //
    let sss = do_gen_src_lex(&vec_optstr)?;
    update_file::update_file(&sss, "comps/cmp_getopts/src/curl.cmd.lex.rs.txt")?;
    //
    Ok(())
}

fn do_gen_src_help(vec_optstr: &[OptStr], vec_line: &[String]) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    //
    let s = r"// WARN: This file is auto generated by";
    sss += &format!("{} {}", s, env!("CARGO_PKG_NAME"));
    sss += r#"
const OPTIONS_TEXT: &str = r""#;
    for line in vec_line {
        sss += &format!("{}\n", line);
    }
    sss += "\";\n";
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
    Ok(sss)
}

fn do_gen_src_match(vec_optstr: &[OptStr]) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    //
    let s = r"// WARN: This file is auto generated by";
    sss += &format!("{} {}", s, env!("CARGO_PKG_NAME"));
    sss += r#"
{
"#;
    //
    for rec in vec_optstr.iter() {
        let sholon = if !rec.lon.is_empty() {
            rec.lon.clone()
        } else {
            rec.sho.clone()
        };
        match rec.type_s.as_str() {
            "bool" => match rec.enum_s.as_str() {
                "Help" => {
                    sss += r#"
    if matches.opt_present("help") {
        print_help_and_exit(conf);
    }
"#;
                }
                "Version" => {
                    sss += r#"
    if matches.opt_present("version") {
        print_version_and_exit(conf);
    }
"#;
                }
                _ => {
                    sss += &format!(
                        "    conf.{} = matches.opt_present(\"{}\");\n",
                        rec.field_s, sholon
                    );
                }
            },
            "String" => {
                sss += &format!("    if matches.opt_present(\"{}\") {{\n", sholon);
                sss += &format!(
                    "        conf.{} = value_to_string(\"{}\", matches.opt_str(\"{}\"))?;\n",
                    rec.field_s, sholon, sholon
                );
                sss += "    }\n";
            }
            "u32" => {
                sss += &format!("    if matches.opt_present(\"{}\") {{\n", sholon);
                sss += &format!(
                    "        conf.{} = value_to_u32(\"{}\", matches.opt_str(\"{}\"))?;\n",
                    rec.field_s, sholon, sholon
                );
                sss += "    }\n";
            }
            "u64" => {
                sss += &format!("    if matches.opt_present(\"{}\") {{\n", sholon);
                sss += &format!(
                    "        conf.{} = value_to_u64(\"{}\", matches.opt_str(\"{}\"))?;\n",
                    rec.field_s, sholon, sholon
                );
                sss += "    }\n";
            }
            _ => unreachable!(),
        }
    }
    sss += r#"
}
"#;
    //
    Ok(sss)
}

fn do_gen_src_lex(vec_optstr: &[OptStr]) -> anyhow::Result<String> {
    let mut sss = String::with_capacity(4 * 1024);
    //
    let s = r"// WARN: This file is auto generated by";
    sss += &format!("{} {}", s, env!("CARGO_PKG_NAME"));
    //
    sss += r#"
{
"#;
    for rec in vec_optstr.iter() {
        if rec.meta.is_empty() {
            sss += &format!(
                "    opts.optflag(\"{}\", \"{}\", \"{}\");\n",
                rec.sho, rec.lon, rec._comment
            );
        } else {
            sss += &format!(
                "    opts.optopt(\"{}\", \"{}\", \"{}\", \"{}\");\n",
                rec.sho, rec.lon, rec._comment, "<>"
            );
        }
    }
    sss += r#"}
"#;
    //
    Ok(sss)
}
