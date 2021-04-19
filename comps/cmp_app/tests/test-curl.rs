use exec_target_a::exec_target;

const TARGET_EXE_PATH: &'static str = "../../target/debug/cmp_app-curl";

macro_rules! some_stdout {
    () => {
        concat!(
            "CmdOptConf {",
            " opt_program: \"../../target/debug/cmp_app-curl\",",
            " opt_abstract_unix_socket: \"\",",
            " opt_alt_svc: \"\",",
            " flg_anyauth: false,",
            " flg_append: true,",
            " flg_basic: false,",
            " opt_cacert: \"\",",
            " opt_capath: \"\",",
            " opt_cert: \"\",",
            " flg_cert_status: false,",
            " opt_cert_type: \"\",",
            " opt_ciphers: \"\",",
            " flg_compressed: false,",
            " flg_compressed_ssh: false,",
            " opt_config: \"\",",
            " opt_connect_timeout: 50,",
            " opt_connect_to: \"\",",
            " opt_continue_at: 0,",
            " opt_cookie: \"\",",
            " opt_cookie_jar: \"\",",
            " flg_create_dirs: false,",
            " flg_crlf: false,",
            " opt_crlfile: \"\",",
            " opt_data: \"\",",
            " opt_data_ascii: \"\",",
            " opt_data_binary: \"\",",
            " opt_data_raw: \"\",",
            " opt_data_urlencode: \"\",",
            " opt_delegation: \"\",",
            " flg_digest: false,",
            " flg_disable: false,",
            " flg_disable_eprt: false,",
            " flg_disable_epsv: false,",
            " flg_disallow_username_in_url: false,",
            " opt_dns_interface: \"\",",
            " opt_dns_ipv4_addr: \"\",",
            " opt_dns_ipv6_addr: \"\",",
            " opt_dns_servers: \"\",",
            " opt_doh_url: \"\",",
            " opt_dump_header: \"\",",
            " opt_egd_file: \"\",",
            " opt_engine: \"\",",
            " opt_etag_save: \"\",",
            " opt_etag_compare: \"\",",
            " opt_expect100_timeout: 0,",
            " flg_fail: false,",
            " flg_fail_early: false,",
            " flg_false_start: false,",
            " opt_form: \"\",",
            " opt_form_string: \"\",",
            " opt_ftp_account: \"\",",
            " opt_ftp_alternative_to_user: \"\",",
            " flg_ftp_create_dirs: false,",
            " opt_ftp_method: \"\",",
            " flg_ftp_pasv: true,",
            " opt_ftp_port: \"\",",
            " flg_ftp_pret: false,",
            " flg_ftp_skip_pasv_ip: false,",
            " flg_ftp_ssl_ccc: false,",
            " opt_ftp_ssl_ccc_mode: \"\",",
            " flg_ftp_ssl_control: false,",
            " flg_get: false,",
            " flg_globoff: false,",
            " opt_happy_eyeballs_timeout_ms: 0,",
            " flg_haproxy_protocol: false,",
            " flg_head: false,",
            " opt_header: \"\",",
            " opt_hostpubmd5: \"\",",
            " flg_http0_9: false,",
            " flg_http1_0: false,",
            " flg_http1_1: false,",
            " flg_http2: true,",
            " flg_http2_prior_knowledge: false,",
            " flg_http3: false,",
            " flg_ignore_content_length: false,",
            " flg_include: false,",
            " flg_insecure: false,",
            " opt_interface: \"\",",
            " flg_ipv4: false,",
            " flg_ipv6: false,",
            " flg_junk_session_cookies: false,",
            " opt_keepalive_time: 0,",
            " opt_key: \"\",",
            " opt_key_type: \"\",",
            " opt_krb: \"\",",
            " opt_libcurl: \"\",",
            " opt_limit_rate: 0,",
            " flg_list_only: false,",
            " opt_local_port: \"\",",
            " flg_location: false,",
            " flg_location_trusted: false,",
            " opt_login_options: \"\",",
            " opt_mail_auth: \"\",",
            " opt_mail_from: \"\",",
            " opt_mail_rcpt: \"\",",
            " flg_manual: false,",
            " opt_max_filesize: 0,",
            " opt_max_redirs: 0,",
            " opt_max_time: 100,",
            " flg_metalink: false,",
            " flg_negotiate: false,",
            " flg_netrc: false,",
            " opt_netrc_file: \"\",",
            " flg_netrc_optional: false,",
            " flg_next: false,",
            " flg_no_alpn: true,",
            " flg_no_buffer: true,",
            " flg_no_keepalive: false,",
            " flg_no_npn: false,",
            " flg_no_progress_meter: false,",
            " flg_no_sessionid: false,",
            " opt_noproxy: \"\",",
            " flg_ntlm: false,",
            " flg_ntlm_wb: false,",
            " opt_oauth2_bearer: \"\",",
            " opt_output: \"\",",
            " flg_parallel: false,",
            " flg_parallel_immediate: false,",
            " flg_parallel_max: false,",
            " opt_pass: \"\",",
            " flg_path_as_is: false,",
            " opt_pinnedpubkey: \"\",",
            " flg_post301: false,",
            " flg_post302: false,",
            " flg_post303: false,",
            " opt_preproxy: \"\",",
            " flg_progress_bar: false,",
            " opt_proto: \"\",",
            " opt_proto_default: \"\",",
            " opt_proto_redir: \"\",",
            " opt_proxy: \"\",",
            " flg_proxy_anyauth: false,",
            " flg_proxy_basic: false,",
            " opt_proxy_cacert: \"\",",
            " opt_proxy_capath: \"\",",
            " opt_proxy_cert: \"\",",
            " opt_proxy_cert_type: \"\",",
            " opt_proxy_ciphers: \"\",",
            " opt_proxy_crlfile: \"\",",
            " flg_proxy_digest: false,",
            " opt_proxy_header: \"\",",
            " flg_proxy_insecure: false,",
            " opt_proxy_key: \"\",",
            " opt_proxy_key_type: \"\",",
            " flg_proxy_negotiate: false,",
            " flg_proxy_ntlm: false,",
            " opt_proxy_pass: \"\",",
            " opt_proxy_pinnedpubkey: \"\",",
            " opt_proxy_service_name: \"\",",
            " flg_proxy_ssl_allow_beast: false,",
            " opt_proxy_tls13_ciphers: \"\",",
            " opt_proxy_tlsauthtype: \"\",",
            " opt_proxy_tlspassword: \"\",",
            " opt_proxy_tlsuser: \"\",",
            " flg_proxy_tlsv1: false,",
            " opt_proxy_user: \"\",",
            " opt_proxy1_0: \"\",",
            " flg_proxytunnel: false,",
            " opt_pubkey: \"\",",
            " flg_quote: false,",
            " opt_random_file: \"\",",
            " opt_range: \"\",",
            " flg_raw: false,",
            " opt_referer: \"\",",
            " flg_remote_header_name: false,",
            " flg_remote_name: false,",
            " flg_remote_name_all: false,",
            " flg_remote_time: false,",
            " opt_request: \"\",",
            " flg_request_target: false,",
            " opt_resolve: \"\",",
            " opt_retry: 0,",
            " flg_retry_connrefused: false,",
            " opt_retry_delay: 0,",
            " opt_retry_max_time: 0,",
            " opt_sasl_authzid: \"\",",
            " flg_sasl_ir: false,",
            " opt_service_name: \"\",",
            " flg_show_error: false,",
            " flg_silent: false,",
            " opt_socks4: \"\",",
            " opt_socks4a: \"\",",
            " opt_socks5: \"\",",
            " flg_socks5_basic: false,",
            " flg_socks5_gssapi: false,",
            " flg_socks5_gssapi_nec: false,",
            " opt_socks5_gssapi_service: \"name1\",",
            " opt_socks5_hostname: \"\",",
            " opt_speed_limit: 0,",
            " opt_speed_time: 1000,",
            " flg_ssl: false,",
            " flg_ssl_allow_beast: false,",
            " flg_ssl_no_revoke: false,",
            " flg_ssl_reqd: false,",
            " flg_sslv2: false,",
            " flg_sslv3: true,",
            " flg_stderr: false,",
            " flg_styled_output: false,",
            " flg_suppress_connect_headers: false,",
            " flg_tcp_fastopen: false,",
            " flg_tcp_nodelay: false,",
            " opt_telnet_option: \"\",",
            " opt_tftp_blksize: 0,",
            " flg_tftp_no_options: false,",
            " opt_time_cond: \"\",",
            " opt_tls_max: \"\",",
            " opt_tls13_ciphers: \"\",",
            " opt_tlsauthtype: \"\",",
            " flg_tlspassword: false,",
            " opt_tlsuser: \"\",",
            " flg_tlsv1: false,",
            " flg_tlsv1_0: false,",
            " flg_tlsv1_1: false,",
            " flg_tlsv1_2: false,",
            " flg_tlsv1_3: false,",
            " flg_tr_encoding: false,",
            " opt_trace: \"\",",
            " opt_trace_ascii: \"\",",
            " flg_trace_time: false,",
            " opt_unix_socket: \"\",",
            " opt_upload_file: \"\",",
            " opt_url: \"\",",
            " flg_use_ascii: false,",
            " opt_user: \"\",",
            " opt_user_agent: \"\",",
            " flg_verbose: false,",
            " opt_write_out: \"\",",
            " flg_xattr: false,",
            " flg_help: false,",
            " flg_version: false,",
            " arg_params: [\"http://url1.com\"] }\n",
        )
    };
}

#[test]
fn test_some_args() {
    #[rustfmt::skip]
    let oup = exec_target(
        TARGET_EXE_PATH,
        &["-a", "--connect-timeout", "50", "--ftp-pasv", "--http2",
            "--max-time", "100", "--no-alpn", "-N",
            "--socks5-gssapi-service", "name1", "-y", "1000", "--sslv3",
            "http://url1.com"]
    );
    assert_eq!(oup.status.success(), true);
    assert_eq!(oup.stdout, some_stdout!());
    assert_eq!(oup.stderr, "");
}
