use clap::Parser;
use std::net::Ipv4Addr;

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Args {
    #[clap(long)]
    pub addr: Ipv4Addr,

    #[clap(long, env("PORT"))]
    pub port: u16,

    #[clap(long, env("SLACK_TOKEN"), hide_env_values = true)]
    pub token: String,
}
