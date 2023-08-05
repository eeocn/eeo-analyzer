use std::sync::Arc;

use dns::CustomResolver;
use downloader::measure_download_speed;
use log::LevelFilter;
use reqwest::{Client, ClientBuilder};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        ConfigBuilder::new()
            .set_time_format_rfc3339() // 设置日期和时间的格式
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();

    let client: Client = ClientBuilder::new()
        .dns_resolver(Arc::new(CustomResolver))
        .build()?;

    measure_download_speed(
        &client,
        "https://releases.ubuntu.com/jammy/ubuntu-22.04.2-live-server-amd64.iso",
    )
    .await
    .unwrap();
    Ok(())
}
