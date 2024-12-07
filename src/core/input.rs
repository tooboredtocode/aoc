use std::error::Error;

pub trait Input: Sized {
    type ParseError: Error + Send + Sync + 'static;

    async fn from_input(input: String) -> Result<Self, Self::ParseError>;
}