#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Path does not exist")]
    PathDoesNotExist,
    #[error("Path is not a file")]
    PathNotFile,
}

pub fn parse() -> Args {
    <Inner as clap::Parser>::parse().into()
}

#[derive(Debug)]
pub struct Args {
    pub verbosity: Verbosity,
    pub port: u16,
    pub db: std::path::PathBuf,
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
    #[arg(short, long, value_parser = parse_db)]
    db: std::path::PathBuf,

    #[command(flatten)]
    smtp: SmtpInner,
}

#[derive(Debug)]
pub struct Smtp {
    pub link: hyper::Uri,
    pub from: smtp::Mailbox,
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
    from: Option<smtp::Mailbox>,

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
            db: value.db,
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

fn parse_db(input: &str) -> Result<std::path::PathBuf, Error> {
    let input = input.strip_prefix("sqlite://").unwrap_or(input);
    let path = std::path::PathBuf::from(input);

    if !path.exists() {
        Err(Error::PathDoesNotExist)
    } else if !path.is_file() {
        Err(Error::PathNotFile)
    } else {
        Ok(path)
    }
}
