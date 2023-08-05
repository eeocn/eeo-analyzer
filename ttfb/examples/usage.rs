use simplelog::{
    ColorChoice, CombinedLogger, ConfigBuilder, LevelFilter, TermLogger, TerminalMode,
};

fn main() -> anyhow::Result<()> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_format_rfc3339() // 设置日期和时间的格式
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let r = ttfb::ttfb(
        "https://releases.ubuntu.com/jammy/ubuntu-22.04.2-live-server-amd64.iso".to_owned(),
        false,
    )?;

    if r.dns_duration_rel().is_some() {
        log::info!(
            "[MeasureTTFB] [DNS Lookup] real time: {rel_time}, abs time: {abs_time}",
            rel_time = r.dns_duration_rel().unwrap().as_secs_f64() * 1000.0,
            abs_time = r.dns_duration_rel().unwrap().as_secs_f64() * 1000.0,
        );
        log::info!(
            "[MeasureTTFB] [TCP Connect] real time: {rel_time}, abs time: {abs_time}",
            rel_time = r.tcp_connect_duration_rel().as_secs_f64() * 1000.0,
            abs_time = r.tcp_connect_duration_abs().as_secs_f64() * 1000.0,
        );
        log::info!(
            "[MeasureTTFB] [TLS Handshake] real time: {rel_time}, abs time: {abs_time}",
            rel_time = r.tls_handshake_duration_rel().unwrap().as_secs_f64() * 1000.0,
            abs_time = r.tls_handshake_duration_abs().unwrap().as_secs_f64() * 1000.0,
        );
        log::info!(
            "[MeasureTTFB] [HTTP GET Req] real time: {rel_time}, abs time: {abs_time}",
            rel_time = r.http_get_send_duration_rel().as_secs_f64() * 1000.0,
            abs_time = r.http_get_send_duration_abs().as_secs_f64() * 1000.0,
        );
        log::info!(
            "[MeasureTTFB] [HTTP Resp TTFB] real time: {rel_time}, abs time: {abs_time}",
            rel_time = r.http_ttfb_duration_rel().as_secs_f64() * 1000.0,
            abs_time = r.http_ttfb_duration_abs().as_secs_f64() * 1000.0,
        );
    }
    Ok(())
}
