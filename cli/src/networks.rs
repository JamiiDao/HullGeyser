#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Default)]
pub enum Network {
    #[default]
    Localhost,
    Custom(String),
}

impl Network {
    pub fn to_url(&self) -> &str {
        //FIXME Check if a URL is valid
        match self {
            Self::Localhost => "http://localhost:8899",
            Self::Custom(value) => value.as_str(),
        }
    }
}

impl From<Option<&String>> for Network {
    fn from(value: Option<&String>) -> Self {
        if let Some(value_exists) = value {
            match value_exists.as_str() {
                "localhost" => Self::Localhost,
                _ => Self::Custom(value_exists.clone()),
            }
        } else {
            Network::default()
        }
    }
}
