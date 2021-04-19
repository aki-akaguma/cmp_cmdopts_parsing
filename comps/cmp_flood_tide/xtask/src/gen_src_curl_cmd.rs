use flood_tide_gen::{FixupType, MetaType, Pasc};

//
pub fn do_gen_src() -> anyhow::Result<()> {
    flood_tide_gen::do_gen_src(
        Pasc::Void,
        "comps/common/optstr-curl/src/curl.cmd.txt",
        Some("comps/cmp_flood_tide/src/curl.cmd.help.rs.txt"),
        Some("comps/cmp_flood_tide/src/curl.cmd.match.rs.txt"),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "connect-timeout" => (false, false, MetaType::U32),
                "continue-at" => (false, false, MetaType::U64),
                "expect100-timeout" => (false, false, MetaType::U32),
                "happy-eyeballs-timeout-ms" => (false, false, MetaType::U64),
                "keepalive-time" => (false, false, MetaType::U32),
                "limit-rate" => (false, false, MetaType::U64),
                "max-filesize" => (false, false, MetaType::U64),
                "max-redirs" => (false, false, MetaType::U32),
                "max-time" => (false, false, MetaType::U32),
                "retry" => (false, false, MetaType::U32),
                "retry-delay" => (false, false, MetaType::U32),
                "retry-max-time" => (false, false, MetaType::U32),
                "speed-limit" => (false, false, MetaType::U64),
                "speed-time" => (false, false, MetaType::U32),
                "tftp-blksize" => (false, false, MetaType::U32),
                _ => return None,
            };
            Some(FixupType::from_tuple(tup))
        },
    )
}
