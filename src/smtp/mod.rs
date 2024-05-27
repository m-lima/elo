pub mod mailbox;
pub use mailbox::Mailbox;

use crate::types;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to build SMTP sender: {0:?}")]
    Transport(#[from] lettre::transport::smtp::Error),
    #[error("Failed to build SMTP sender: Could not connect")]
    Connection,
}

#[derive(Debug, thiserror::Error)]
enum LettreError {
    #[error(transparent)]
    Transport(#[from] lettre::transport::smtp::Error),
    #[error(transparent)]
    Send(#[from] lettre::error::Error),
}

pub enum Payload {
    Invite(Mailbox),
    _Challenge(types::Id),
    _Match(types::Id),
}

#[derive(Debug, Clone)]
pub struct Smtp {
    tx: Option<tokio::sync::mpsc::Sender<Payload>>,
}

impl Smtp {
    pub async fn new(link: hyper::Uri, host: hyper::Uri, from: Mailbox) -> Result<Self, Error> {
        let (tx, rx) = tokio::sync::mpsc::channel(16);
        let worker = Worker::new(link, host, from, rx).await?;
        tokio::spawn(worker.listen());

        Ok(Self { tx: Some(tx) })
    }

    #[must_use]
    pub fn empty() -> Self {
        Self { tx: None }
    }

    pub async fn send(&mut self, payload: Payload) {
        if let Some(tx) = self.tx.as_ref() {
            if tx.send(payload).await.is_err() {
                tracing::error!("SMTP channel closed");
                self.tx = None;
            }
        }
    }
}

struct Worker {
    link: hyper::Uri,
    from: Mailbox,
    transport: lettre::AsyncSmtpTransport<lettre::Tokio1Executor>,
    rx: tokio::sync::mpsc::Receiver<Payload>,
}

impl Worker {
    async fn new(
        link: hyper::Uri,
        host: hyper::Uri,
        from: Mailbox,
        rx: tokio::sync::mpsc::Receiver<Payload>,
    ) -> Result<Self, Error> {
        let transport = lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::from_url(
            host.to_string().as_str(),
        )?
        .build();

        if !transport.test_connection().await? {
            return Err(Error::Connection);
        }

        tracing::info!(%host, %from, "Starting SMTP worker");

        Ok(Self {
            link,
            from,
            transport,
            rx,
        })
    }

    async fn listen(mut self) {
        loop {
            let Some(payload) = self.rx.recv().await else {
                tracing::error!("Stopping SMTP worker due to lack of senders");
                break;
            };

            if let Err(error) = self.send(payload).await {
                tracing::error!(%error, "Failed to send email");
            }
        }
    }

    async fn send(&self, payload: Payload) -> Result<(), LettreError> {
        match payload {
            Payload::Invite(recipient) => {
                tracing::info!(%recipient, "Sending invite email");

                let invitee = String::from(recipient.name());
                let name = self.from.name();
                let link = &self.link;

                let message = lettre::Message::builder()
                    .from(self.from.clone().into())
                    .to(recipient.into())
                    .subject(format!("Invitation to join {name}"))
                    .multipart(
                        lettre::message::MultiPart::alternative()
                            .singlepart(
                                lettre::message::SinglePart::builder()
                                    .header(lettre::message::header::ContentType::TEXT_PLAIN)
                                    .body(format!(
                                        r#"Hi {invitee}!

You have been invited to join {name}!
Try it out at {link}

Happy gaming!
"#
                                    )),
                            )
                            .singlepart(
                                lettre::message::SinglePart::builder()
                                    .header(lettre::message::header::ContentType::TEXT_HTML)
                                    .body(format!(
                                        r#"<!DOCTYPE html>
<html lang = "en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Invitation to join {name}</title>
</head>
<body>
Hi {invitee}!

You have been invited to join {name}!
Try it out at <a href={link}>{link}</a>

Happy gaming!
</body>
</html>
"#
                                    )),
                            ),
                    )?;

                lettre::AsyncTransport::send(&self.transport, message).await?;
            }
            Payload::_Challenge(_) => todo!(),
            Payload::_Match(_) => todo!(),
        }

        Ok(())
    }
}
