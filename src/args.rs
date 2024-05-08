pub fn parse() -> Args {
    <Inner as clap::Parser>::parse().into()
}

#[derive(Debug)]
pub struct Args {
    pub verbosity: Verbosity,
    pub port: u16,
}

#[derive(Debug, Copy, Clone)]
pub struct Verbosity {
    pub level: tracing::Level,
}

#[derive(Debug, clap::Parser)]
struct Inner {
    /// Verbosity level
    #[arg(short, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Port on which to serve
    #[arg(short , long, default_value_t = 80, value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,
}

impl From<Inner> for Args {
    fn from(value: Inner) -> Self {
        Self {
            verbosity: value.verbosity.into(),
            port: value.port,
        }
    }
}

impl From<u8> for Verbosity {
    fn from(value: u8) -> Self {
        match value {
            0 => Self {
                level: tracing::Level::ERROR,
            },
            1 => Self {
                level: tracing::Level::WARN,
            },
            2 => Self {
                level: tracing::Level::INFO,
            },
            3 => Self {
                level: tracing::Level::DEBUG,
            },
            _ => Self {
                level: tracing::Level::TRACE,
            },
        }
    }
}
