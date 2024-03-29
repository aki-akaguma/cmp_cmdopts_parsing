use optstr_curl::parse_input_file;
use optstr_curl::OptStr;

pub fn do_gen_src() -> anyhow::Result<()> {
    let (vec_optstr, vec_line) = parse_input_file("comps/common/optstr-curl/src/curl.cmd.txt")?;
    //
    let sss = do_gen_src_help(&vec_optstr, &vec_line)?;
    update_file::update_file(&sss, "comps/cmp_clap4/src/curl.cmd.help.rs.txt")?;
    //
    let sss = do_gen_src_match(&vec_optstr)?;
    update_file::update_file(&sss, "comps/cmp_clap4/src/curl.cmd.match.rs.txt")?;
    //
    let sss = do_gen_src_lex(&vec_optstr)?;
    update_file::update_file(&sss, "comps/cmp_clap4/src/curl.cmd.lex.rs.txt")?;
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
        let name = if !rec.lon.is_empty() {
            rec.lon.clone()
        } else {
            rec.sho.clone()
        };
        match rec.type_s.as_str() {
            "bool" => match rec.enum_s.as_str() {
                "Help" => {}
                "Version" => {}
                _ => {
                    sss += &format!("    if matches.contains_id(\"{}\") {{\n", name);
                    sss += &format!(
                        "        conf.{} = matches.get_flag(\"{}\");\n",
                        rec.field_s, name
                    );
                    sss += "    }\n";
                }
            },
            "String" => {
                sss += &format!("    if matches.contains_id(\"{}\") {{\n", name);
                sss += &format!(
                    "        conf.{} = value_to_string(\"{}\", matches.get_one::<String>(\"{}\"))?;\n",
                    rec.field_s, name, name
                );
                sss += "    }\n";
            }
            "u32" => {
                sss += &format!("    if matches.contains_id(\"{}\") {{\n", name);
                sss += &format!(
                    "        conf.{} = value_to_u32(\"{}\", matches.get_one::<String>(\"{}\"))?;\n",
                    rec.field_s, name, name
                );
                sss += "    }\n";
            }
            "u64" => {
                sss += &format!("    if matches.contains_id(\"{}\") {{\n", name);
                sss += &format!(
                    "        conf.{} = value_to_u64(\"{}\", matches.get_one::<String>(\"{}\"))?;\n",
                    rec.field_s, name, name
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
    Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
"#;
    for rec in vec_optstr.iter() {
        if rec.lon == "help" || rec.lon == "version" {
            continue;
        }
        let name = if !rec.lon.is_empty() {
            rec.lon.clone()
        } else {
            rec.sho.clone()
        };
        if rec.meta.is_empty() {
            if !rec.sho.is_empty() && !rec.lon.is_empty() {
                sss += &format!(
                    "    .arg(Arg::new(\"{}\").short(\'{}\').long(\"{}\").help(\"{}\").action(ArgAction::SetTrue))\n",
                    name, rec.sho, rec.lon, rec._comment
                );
            } else if !rec.sho.is_empty() {
                sss += &format!(
                    "    .arg(Arg::new(\"{}\").short(\'{}\').help(\"{}\").action(ArgAction::SetTrue))\n",
                    name, rec.sho, rec._comment
                );
            } else {
                sss += &format!(
                    "    .arg(Arg::new(\"{}\").long(\"{}\").help(\"{}\").action(ArgAction::SetTrue))\n",
                    name, rec.lon, rec._comment
                );
            }
        } else if !rec.sho.is_empty() && !rec.lon.is_empty() {
            sss += &format!("    .arg(Arg::new(\"{}\").short(\'{}\').long(\"{}\").help(\"{}\").num_args(0..=1).required(false))\n", name, rec.sho, rec.lon, rec._comment);
        } else if !rec.sho.is_empty() {
            sss += &format!("    .arg(Arg::new(\"{}\").short(\'{}\').help(\"{}\").num_args(0..=1).required(false))\n", name, rec.sho, rec._comment);
        } else {
            sss += &format!("    .arg(Arg::new(\"{}\").long(\"{}\").help(\"{}\").num_args(0..=1).required(false))\n", name, rec.lon, rec._comment);
        }
    }
    sss += r#"    .arg(Arg::new("ARG-URL").help("url").required(true).index(1))
"#;
    sss += r#"
}
"#;
    //
    Ok(sss)
}
