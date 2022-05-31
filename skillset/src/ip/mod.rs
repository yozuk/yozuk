use clap::Parser;
use std::net::IpAddr;
use std::str::FromStr;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"CuDh69d5nMW-WsESpz5mI",
    init: |_| {
        Skill::builder()
            .add_translator(IpTranslator)
            .set_command(IpCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct IpTranslator;

impl Translator for IpTranslator {
    fn generate_command(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        let is_ipaddr = !args.is_empty()
            && args
                .iter()
                .all(|arg| IpAddr::from_str(arg.as_str()).is_ok());
        if is_ipaddr {
            return Some(CommandArgs::new().add_args_iter(args.iter().map(|arg| arg.as_str())));
        }
        None
    }
}

#[derive(Debug)]
pub struct IpCommand;

impl Command for IpCommand {
    fn run(
        &self,
        args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let args = Args::try_parse_from(args.args)?;
        let blocks = args
            .inputs
            .into_iter()
            .map(|input| expand_addrs(&input))
            .map(|addrs| Block::Data(block::Data::new().set_text_data(addrs.join("\n"))));
        Ok(Output::new()
            .set_title("IPAddress converter")
            .add_blocks_iter(blocks))
    }
}

#[derive(Parser)]
#[clap(trailing_var_arg = true)]
struct Args {
    #[clap(multiple_occurrences(true))]
    pub inputs: Vec<String>,
}

fn expand_addrs(s: &str) -> Vec<String> {
    let mut addrs = match IpAddr::from_str(s) {
        Ok(IpAddr::V4(addr)) => vec![IpAddr::V6(addr.to_ipv6_mapped()), IpAddr::V4(addr)],
        Ok(IpAddr::V6(addr)) => addr
            .to_ipv4()
            .into_iter()
            .map(IpAddr::V4)
            .chain([IpAddr::V6(addr)])
            .collect(),
        _ => vec![],
    };
    addrs.sort();
    let mut addrs = addrs
        .into_iter()
        .flat_map(|addr| match addr {
            IpAddr::V4(addr) => vec![addr.to_string()],
            IpAddr::V6(addr) => {
                vec![
                    addr.to_string(),
                    addr.segments()
                        .iter()
                        .map(|s| format!("{:x}", s))
                        .collect::<Vec<_>>()
                        .join(":"),
                    addr.segments()
                        .iter()
                        .map(|s| format!("{:04x}", s))
                        .collect::<Vec<_>>()
                        .join(":"),
                ]
            }
        })
        .collect::<Vec<_>>();

    addrs.dedup();
    addrs
}
