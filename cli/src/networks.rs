#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum Network {
    #[default]
    Localhost,
    Devnet,
    Testnet,
    MainnetBeta,
    Custom(String),
}

impl From<Option<&String>> for Network {
    fn from(value: Option<&String>) -> Self {
        if let Some(value_exists) = value {
            match value_exists.as_str() {
                "localhost" => Self::Localhost,
                "devnet" => Self::Devnet,
                "testnet" => Self::Testnet,
                "mainnet-beta" => Self::MainnetBeta,
                _ => Self::Custom(value_exists.clone()),
            }
        } else {
            Network::default()
        }
    }
}
