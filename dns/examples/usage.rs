use std::{sync::Arc, time::Duration};

use dns::CustomResolver;
use log::LevelFilter;
use reqwest::{Client, ClientBuilder};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            ConfigBuilder::new()
                .set_time_format_rfc3339()
                // .set_time_format_custom(format_description!(
                //     "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond]"
                // )) // 设置日期和时间的格式
                .build(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // WriteLogger::new(LevelFilter::Info, Config::default(), File::create("my_rust_binary.log").unwrap()),
    ])
    .unwrap();

    // 创建自定义的 reqwest 客户端
    let client: Client = ClientBuilder::new()
        .dns_resolver(Arc::new(CustomResolver))
        .timeout(Duration::from_secs(10))
        .build()?;

    // 发起 HTTP 请求
    let response = client.get("https://www.baidu.com").send().await?;
    println!("{}", response.status());

    Ok(())
}
