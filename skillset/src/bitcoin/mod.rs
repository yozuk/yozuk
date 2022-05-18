use bitcoin::{
    network::constants::Network,
    secp256k1::{rand::thread_rng, Secp256k1},
    util::address::Address,
    PrivateKey, PublicKey,
};
use itertools::iproduct;
use yozuk_helper_english::normalized_eq;
use yozuk_sdk::prelude::*;

pub const ENTRY: SkillEntry = SkillEntry {
    model_id: b"3oV85PD5eAdimpfsfCO12",
    config_schema: None,
    init: |_, _| {
        Skill::builder()
            .add_corpus(BitcoinCorpus)
            .add_translator(BitcoinTranslator)
            .set_command(BitcoinCommand)
            .build()
    },
};

#[derive(Debug)]
pub struct BitcoinCorpus;

impl Corpus for BitcoinCorpus {
    fn training_data(&self) -> Vec<Vec<Token>> {
        iproduct!(["generate", "new"], ["bitcoin", "btc"])
            .flat_map(|(verb, name)| {
                vec![
                    tk!([
                        verb,
                        name; "command:bitcoin"
                    ]),
                    tk!([
                        verb,
                        name; "command:bitcoin",
                        "address"
                    ]),
                ]
            })
            .chain(vec![
                tk!(["bitcoin"; "command:bitcoin"]),
                tk!(["btc"; "command:bitcoin"]),
            ])
            .collect()
    }
}

#[derive(Debug)]
pub struct BitcoinTranslator;

impl Translator for BitcoinTranslator {
    fn parse(&self, args: &[Token], _streams: &[InputStream]) -> Option<CommandArgs> {
        if args.iter().any(|arg| {
            arg.tag == "command:bitcoin" && normalized_eq(arg.as_str(), &["bitcoin", "btc"], 0)
        }) {
            return Some(CommandArgs::new());
        }
        None
    }
}

#[derive(Debug)]
pub struct BitcoinCommand;

impl Command for BitcoinCommand {
    fn run(
        &self,
        _args: CommandArgs,
        _streams: &mut [InputStream],
        _i18n: &I18n,
    ) -> Result<Output, CommandError> {
        let s = Secp256k1::new();
        let (privkey, pubkey) = s.generate_keypair(&mut thread_rng());
        let private_key = PrivateKey::new(privkey, Network::Bitcoin);
        let public_key = PublicKey::new(pubkey);
        let address = Address::p2pkh(&public_key, Network::Bitcoin);

        Ok(Output::new().set_title("Bitcoin").add_blocks_iter(vec![
            Block::Comment(block::Comment::new().set_text("Generating a new address")),
            Block::Data(block::Data::new().set_text_data(address.to_string())),
            Block::Spoiler(block::Spoiler::new("Private Key", private_key.to_wif())),
        ]))
    }
}
