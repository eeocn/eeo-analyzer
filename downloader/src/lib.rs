use std::cmp::min;

use futures_util::StreamExt;
use reqwest::Client;
use tokio::time::Instant;

pub async fn measure_download_speed(client: &Client, url: &str) -> anyhow::Result<()> {
    let res = client.get(url).send().await.or(Err(anyhow::anyhow!(
        "[MeasureDownloadSpeed] Failed to GET from '{}'",
        &url
    )))?;

    let total_size = res.content_length().ok_or(anyhow::anyhow!(
        "[MeasureDownloadSpeed]  Failed to get content length from '{}'",
        &url
    ))?;

    log::info!(
        "[MeasureDownloadSpeed] starting test download speed use url: {url}, total size:{}MB",
        (total_size / 1048576)
    );

    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    let start_time = Instant::now();
    while let Some(item) = stream.next().await {
        let chunk = item.or(Err(anyhow::anyhow!(
            "[MeasureDownloadSpeed] Error while downloading file"
        )))?;

        let new = min(downloaded + (chunk.len() as u64), total_size);
        print!("\r");
        print!(
            "[MeasureDownloadSpeed] progressing... {:.2}% ",
            (new as f64 / total_size as f64) * 100.0,
        );
        downloaded = new;
    }
    println!("");
    log::info!("[MeasureDownloadSpeed] test download speed completed with {url}");

    let elapsed_time = start_time.elapsed();
    let time_spent = elapsed_time.as_secs();
    let download_speed = (total_size as f64 / 1048576.0) / time_spent as f64;

    log::info!(
        "[MeasureDownloadSpeed] time spent: {time_spent}s, download speed: {download_speed:.2}MB/s",
    );

    return Ok(());
}
