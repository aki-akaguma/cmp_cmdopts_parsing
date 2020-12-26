use anyhow::Context;
use std::io::BufRead;

#[rustfmt::skip]
#[derive(Default, Clone)]
pub struct OptStr {
    pub num: i32,           // number
    pub sho: String,        // short option
    pub lon: String,        // long option
    pub meta: String,       // option's meta
    pub _comment: String,   // option comment
    pub type_s: String,     // type string
    pub enum_s: String,     // enume field string
    pub field_s: String,    // struct field string
}

impl OptStr {
    fn to_enum(&self) -> String {
        let r = &self.lon;
        let v: Vec<_> = r
            .split('-')
            .map(|w| {
                let mut cs: Vec<char> = w.chars().collect();
                cs[0] = cs[0].to_ascii_uppercase();
                let mut s = String::new();
                for c in cs {
                    s.push(if c == '.' { '_' } else { c });
                }
                s
            })
            .collect();
        v.join("")
    }
    fn to_field(&self) -> String {
        let mut s = String::with_capacity(self.lon.len());
        for c in self.lon.chars() {
            #[rustfmt::skip]
            let c = match c { '-' => '_', '.' => '_', _ => c, };
            s.push(c);
        }
        let prefix = if self.meta.is_empty() { "flg_" } else { "opt_" };
        prefix.to_string() + &s
    }
}

pub fn parse_input_file(in_file: &str) -> anyhow::Result<(Vec<OptStr>, Vec<String>)> {
    let mut vec_line: Vec<String> = Vec::new();
    let mut vec_optstr: Vec<OptStr> = Vec::new();
    //
    let re_1 = regex::Regex::new(r"^ *-([^ ]), +--([^ ]+) +(<[^>]+>) +([^ ].*)$").unwrap();
    let re_2 = regex::Regex::new(r"^ *-([^ ]), +--([^ ]+) +([^ ].*)$").unwrap();
    let re_3 = regex::Regex::new(r"^ +--([^ ]+) +(<[^>]+>) +([^ ].*)$").unwrap();
    let re_4 = regex::Regex::new(r"^ +--([^ ]+) +([^ ].*)$").unwrap();
    //
    let mut v_num = 0;
    let reader = std::io::BufReader::new(
        std::fs::File::open(in_file)
            .with_context(|| format!("could not open file `{}`", in_file))?,
    );
    for line in reader.lines() {
        let line = line?;
        if line == "Options:" {
            // nothing todo
        } else if let Some(caps) = re_1.captures(&line) {
            //  -C  --continue-at <offset>        Resumed transfer offset
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: caps[1].to_string(),
                lon: caps[2].to_string(),
                meta: caps[3].to_string(),
                _comment: caps[4].to_string(),
                ..OptStr::default()
            });
        } else if let Some(caps) = re_2.captures(&line) {
            //  -q  --disable             Disable .curlrc
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: caps[1].to_string(),
                lon: caps[2].to_string(),
                meta: "".to_string(),
                _comment: caps[3].to_string(),
                ..OptStr::default()
            });
        } else if let Some(caps) = re_3.captures(&line) {
            //      --data-binary <data>  HTTP POST binary data
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: "".to_string(),
                lon: caps[1].to_string(),
                meta: caps[2].to_string(),
                _comment: caps[3].to_string(),
                ..OptStr::default()
            });
        } else if let Some(caps) = re_4.captures(&line) {
            //      --digest              Use HTTP Digest Authentication
            v_num += 1;
            vec_optstr.push(OptStr {
                num: v_num,
                sho: "".to_string(),
                lon: caps[1].to_string(),
                meta: "".to_string(),
                _comment: caps[2].to_string(),
                ..OptStr::default()
            });
        } else {
            eprintln!("LINE ERROR: {}", line);
            unreachable!();
        }
        vec_line.push(line);
    }
    //
    for v in &mut vec_optstr {
        let v_type = if v.meta.is_empty() { "bool" } else { "String" };
        let v_type = match v.lon.as_str() {
            "connect-timeout" => "u32",
            "continue-at" => "u64",
            "expect100-timeout" => "u32",
            "happy-eyeballs-timeout-ms" => "u64",
            "keepalive-time" => "u32",
            "limit-rate" => "u64",
            "max-filesize" => "u64",
            "max-redirs" => "u32",
            "max-time" => "u32",
            "retry" => "u32",
            "retry-delay" => "u32",
            "retry-max-time" => "u32",
            "speed-limit" => "u64",
            "speed-time" => "u32",
            "tftp-blksize" => "u32",
            _ => v_type,
        };
        //
        v.type_s = v_type.to_string();
        v.enum_s = v.to_enum();
        v.field_s = v.to_field();
    }
    //
    Ok((vec_optstr, vec_line))
}
