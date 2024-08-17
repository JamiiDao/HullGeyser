use std::path::PathBuf;

use clap::{value_parser, ArgMatches, Command as ClapCommand};

use crate::{GeyserConfig, Network};

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

impl TryFrom<ArgMatches> for GeyserConfig {
    type Error = String; //FIXME return better errors

    fn try_from(user_args: ArgMatches) -> Result<Self, Self::Error> {
        let network: Network = user_args.get_one::<String>("network").into();

        let mut parsed_config = GeyserConfig {
            network,
            ..Default::default()
        };

        if let Some(path_exists) = user_args.get_one::<PathBuf>("multiple") {
            parsed_config.extras_location.replace(path_exists.into());
        }

        if let Some(dir) = user_args.get_one::<PathBuf>("dir") {
            parsed_config.dir = dir.into();
        }

        Ok(parsed_config)
    }
}
