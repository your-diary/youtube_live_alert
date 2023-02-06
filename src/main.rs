use std::{env, fs, thread, time};
use std::{error::Error, time::Duration};

use log::{error, info};
use reqwest::blocking::{Client, Response};
use serde::Deserialize;
use serde::Serialize;

/*-------------------------------------*/

#[derive(Debug, Deserialize)]
struct Config {
    name: String,
    username: String,
    discord_url: String,
    timeout_sec: u64,
    interval_sec: u64,
}

impl Config {
    fn new(config_file: &str) -> Self {
        let file = fs::read_to_string(config_file).unwrap();
        serde_json::from_str(&file).unwrap()
    }
}

/*-------------------------------------*/

struct YouTube {
    client: Client,
    url: String,
}

impl YouTube {
    fn new(config: &Config) -> Self {
        Self {
            client: Client::builder()
                .timeout(Some(Duration::from_secs(config.timeout_sec)))
                .build()
                .unwrap(),
            url: format!("https://www.youtube.com/{}/live", config.username),
        }
    }

    fn check(&self) -> Result<bool, Box<dyn Error>> {
        let res = self.client.get(&self.url).send()?;
        let status = res.status();
        let body = res.text()?;
        if (!status.is_success()) {
            return Err(body.into());
        }
        Ok(body.contains(r#"name="title""#))
    }
}

/*-------------------------------------*/

#[derive(Debug, Serialize)]
struct Req {
    wait: bool,
    content: String,
}
impl Req {
    fn new(content: &str) -> Self {
        Self {
            wait: true,
            content: content.to_string(),
        }
    }
}

fn discord_notification(url: &str, message: &str, timeout_sec: u64) -> Result<(), Box<dyn Error>> {
    let req = Req::new(message);
    let res: Response = Client::new()
        .post(url)
        .body(serde_json::to_string(&req)?)
        .header("Content-Type", "application/json")
        .timeout(Duration::from_secs(timeout_sec))
        .send()?;
    let status = res.status();
    let body = res.text()?;
    if (!status.is_success()) {
        return Err(body.into());
    }
    Ok(())
}

/*-------------------------------------*/

const CONFIG_FILE: &str = "./config.json";

fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::builder().format_target(false).init();

    let config = Config::new(CONFIG_FILE);
    let youtube = YouTube::new(&config);

    let mut is_live_active_old = false;
    loop {
        thread::sleep(time::Duration::from_secs(config.interval_sec));

        let res = youtube.check();
        if (res.is_err()) {
            error!("{:?}", res);
            continue;
        }
        let is_live_active = res.unwrap();
        if (is_live_active && !is_live_active_old) {
            let res = discord_notification(
                &config.discord_url,
                &format!("{}'s live started.", config.name),
                config.timeout_sec,
            );
            if (res.is_err()) {
                error!("{:?}", res);
                continue;
            }
        }
        info!(
            "{}",
            if (is_live_active) {
                "active"
            } else {
                "inactive"
            }
        );
        is_live_active_old = is_live_active;
    }
}
