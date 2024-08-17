use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command as ClapCommand};

pub fn get_args() -> ArgMatches {
    ClapCommand::new("hull-rpc")
        .arg(
            clap::arg!(
                -a --account <Base58> "One account to listen to"
            )
            .required(false)
            .value_parser(value_parser!(String)),
        )
        .arg(
            clap::arg!(
                -n --network <Network> "localhost (default), mainnet-beta, testnet, devnet, custom"
            )
            .required(false)
            .value_parser(value_parser!(String)),
        )
        .arg(
            clap::arg!(
                -m --multiple <FILE> "Use a custom config file that lists accounts, transactions etc to listen to"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        ).arg(
            clap::arg!(
                -d --dir <DIRECTORY> "Where to store the application config. Default is /tmp/HullGeyser"
            )
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .get_matches()
}
