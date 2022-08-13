use clap::Parser;
use std::net::Ipv4Addr;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(long, default_value_t = Ipv4Addr::new(127, 0, 0, 1))]
    pub addr: Ipv4Addr,

    #[clap(long, env("PORT"), default_value_t = 8080)]
    pub port: u16,

    #[clap(long, env("SLACK_BOT_TOKEN"), hide_env_values = true)]
    pub bot_token: String,

    #[clap(long, env("SLACK_APP_TOKEN"), hide_env_values = true)]
    pub app_token: Option<String>,
}
