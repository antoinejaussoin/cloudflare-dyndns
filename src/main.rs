mod cloudflare;
mod config;
mod ip;
mod record;

use cloudflare::{needs_update, CloudflareClient, CLOUDFLARE_API_BASE};
use colored::Colorize;
use config::Config;
use ip::get_public_ip;
use record::DnsRecord;
use std::time::Duration;

const IPIFY_URL: &str = "http://api.ipify.org";

fn log(text: &str) {
    let ts = chrono_like_time();
    println!("{}{}", format!("[{ts}] ").blue(), text);
}

/// Approximate Node's `toLocaleTimeString()` for log timestamps (local clock, HH:MM:SS).
fn chrono_like_time() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    // Prefer a simple local-ish display without pulling in chrono.
    // Format matches common en-US 12h style poorly without locale crates;
    // use 24h HH:MM:SS which is readable and stable in Docker logs.
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Best-effort UTC clock; colored banners matter more than exact locale.
    let secs = now % 86400;
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

fn debug_log(cfg: &Config, message: &str, value: &impl std::fmt::Debug) {
    if cfg.debug {
        println!("{message} {value:?}");
    }
}

async fn get_dns_entry(
    cfg: &Config,
    cf: &CloudflareClient,
) -> Result<DnsRecord, cloudflare::CloudflareError> {
    let record = cf.get_dns_record(&cfg.domain, &cfg.record).await?;
    log(&format!(
        "Current DNS IP: {}: {}",
        record.name.green(),
        record.content.red()
    ));
    debug_log(cfg, "Record: ", &record);
    Ok(record)
}

async fn update(
    cfg: &Config,
    cf: &CloudflareClient,
    http: &reqwest::Client,
    ipify_url: &str,
    last_record: &DnsRecord,
) -> anyhow::Result<DnsRecord> {
    let public_ip = get_public_ip(http, ipify_url).await?;
    if needs_update(&public_ip, &last_record.content) {
        log(&format!(
            "🔴 The public IP has changed: {} vs DNS {}",
            public_ip, last_record.content
        ));
        let record = cf
            .update_record(
                &last_record.zone_id,
                &last_record.id,
                &cfg.record,
                &public_ip,
            )
            .await?;
        log(&format!(
            "🟠 The DNS record has been updated to {}",
            record.content.red()
        ));
        debug_log(cfg, "resp: ", &record);
        Ok(record)
    } else {
        log(&format!(
            "✅ The current public IP is the same ({})",
            public_ip.red()
        ));
        Ok(last_record.clone())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", "---------------------------------".yellow());
    println!("{}", "🌍 Cloudflare Dynamic DNS v2.0 🔁 ".white());
    println!("{}", "---------------------------------".yellow());
    println!();

    let cfg = Config::from_env();
    let http = reqwest::Client::new();
    let cf = CloudflareClient::new(http.clone(), CLOUDFLARE_API_BASE, &cfg.cloudflare_api_token);

    log("Initialization");

    let mut last_record = match get_dns_entry(&cfg, &cf).await {
        Ok(r) => r,
        Err(ex) => {
            log(&format!(
                "❗️ {} {}",
                "An error occurred when initializing: ".red(),
                ex
            ));
            return Err(ex.into());
        }
    };

    match update(&cfg, &cf, &http, IPIFY_URL, &last_record).await {
        Ok(r) => last_record = r,
        Err(ex) => {
            log(&format!(
                "❗️ {} {}",
                "An error occurred when initializing: ".red(),
                ex
            ));
            return Err(ex);
        }
    }

    let interval = Duration::from_secs(cfg.check_interval_sec);
    loop {
        tokio::time::sleep(interval).await;
        match async {
            log("Refreshing DNS entry");
            last_record = get_dns_entry(&cfg, &cf).await?;
            last_record = update(&cfg, &cf, &http, IPIFY_URL, &last_record).await?;
            Ok::<(), anyhow::Error>(())
        }
        .await
        {
            Ok(()) => {}
            Err(ex) => {
                log(&format!("❗️ {} {}", "An error occurred: ".red(), ex));
            }
        }
    }
}
