use toml::Table;

pub trait Parseable {
    fn parse(input: Table) -> Result<Box<Self>, ParseError>;
}

#[derive(Debug, Clone, Copy)]
pub struct ParseError {}
