use crate::mailbox;

pub fn parse() -> Args {
    <Inner as clap::Parser>::parse().into()
}

#[derive(Debug)]
pub struct Args {
    pub verbosity: Verbosity,
    pub port: u16,
    pub db: std::path::PathBuf,
    #[cfg(not(feature = "local"))]
    pub init: bool,
    #[cfg(feature = "local")]
    pub init: Option<u16>,
    pub smtp: Option<Smtp>,
}

#[derive(Debug, Copy, Clone)]
pub struct Verbosity {
    pub level: tracing::Level,
    pub internal: bool,
}

#[derive(Debug, clap::Parser)]
struct Inner {
    /// Verbosity level
    #[arg(short, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Port on which to serve
    #[arg(short , long, default_value_t = 80, value_parser = clap::value_parser!(u16).range(1..))]
    port: u16,

    /// Path to databases directory
    #[arg(short, long)]
    db: String,

    /// Initialize an empty database
    #[cfg(not(feature = "local"))]
    #[arg(short, long)]
    init: bool,

    /// Initialize an empty database
    #[cfg(feature = "local")]
    #[arg(short, long)]
    init: Option<u16>,

    #[command(flatten)]
    smtp: SmtpInner,
}

#[derive(Debug)]
pub struct Smtp {
    pub link: hyper::Uri,
    pub from: mailbox::Mailbox,
    #[allow(clippy::struct_field_names)]
    pub smtp: hyper::Uri,
}

#[derive(Debug, clap::Args)]
#[group(required = false)]
struct SmtpInner {
    /// Link to hosted website
    #[arg(short, long, requires = "smtp")]
    link: Option<hyper::Uri>,

    /// Address to send emails from
    ///
    /// Example: Name <user@domain.com>
    #[arg(short, long, requires = "link")]
    from: Option<mailbox::Mailbox>,

    /// SMTP server to send emails
    ///
    /// Example: smtp://example.com:587?tls=required
    #[allow(clippy::doc_markdown)]
    #[arg(short, long, requires = "from")]
    smtp: Option<hyper::Uri>,
}

impl From<Inner> for Args {
    fn from(value: Inner) -> Self {
        let smtp = value.smtp;

        let smtp = match (smtp.link, smtp.from, smtp.smtp) {
            (Some(link), Some(from), Some(smtp)) => Some(Smtp { link, from, smtp }),
            (None, None, None) => None,
            _ => unreachable!(),
        };

        Self {
            verbosity: value.verbosity.into(),
            port: value.port,
            db: value.db.strip_prefix("sqlite://").map_or_else(
                || std::path::PathBuf::from(&value.db),
                std::path::PathBuf::from,
            ),
            init: value.init,
            smtp,
        }
    }
}

impl From<u8> for Verbosity {
    fn from(value: u8) -> Self {
        match value {
            0 => Self {
                level: tracing::Level::ERROR,
                internal: false,
            },
            1 => Self {
                level: tracing::Level::WARN,
                internal: false,
            },
            2 => Self {
                level: tracing::Level::INFO,
                internal: false,
            },
            3 => Self {
                level: tracing::Level::INFO,
                internal: true,
            },
            4 => Self {
                level: tracing::Level::DEBUG,
                internal: true,
            },
            _ => Self {
                level: tracing::Level::TRACE,
                internal: true,
            },
        }
    }
}
