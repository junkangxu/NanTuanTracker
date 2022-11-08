#[derive(Clone, Debug)]
pub enum PollerError {
    Publisher(PublisherError),
    Provider(ProviderError),
    Parser(ParserError)
}

#[derive(Clone, Debug)]
pub enum ParserError {
    Player,
    Guild,
    Match
}

#[derive(Clone, Debug)]
pub enum ProviderError {
    Stratz,
    Dynamo
}

#[derive(Clone, Debug)]
pub enum PublisherError {
    Discord,
    Kook
}

impl std::error::Error for PollerError {}
impl std::error::Error for ParserError {}
impl std::error::Error for ProviderError {}
impl std::error::Error for PublisherError {}

impl std::fmt::Display for PollerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Publisher(publisher_error) => write!(f, "{}", publisher_error),
            Self::Provider(provider_error) => write!(f, "{}", provider_error),
            Self::Parser(parser_error) => write!(f, "{}", parser_error)
        }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Player => write!(f, "{}", "ParserError.Player"),
            Self::Guild => write!(f, "{}", "ParserError.Guild"),
            Self::Match => write!(f, "{}", "ParserError.Match")
        }
    }
}

impl std::fmt::Display for ProviderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stratz => write!(f, "{}", "ProviderError.Player"),
            Self::Dynamo => write!(f, "{}", "ProviderDyanmo")
        }
    }
}

impl std::fmt::Display for PublisherError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Discord => write!(f, "{}", "PublisherError.Discord"),
            Self::Kook => write!(f, "{}", "PublisherError.Kook")
        }
    }
}