#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to build SMTP sender: {0:?}")]
    Address(#[from] lettre::address::AddressError),
    #[error("Failed to build SMTP sender: {0:?}")]
    Transport(#[from] lettre::transport::smtp::Error),
}

#[derive(Debug, thiserror::Error)]
enum LettreError {
    #[error(transparent)]
    Address(#[from] lettre::address::AddressError),
    #[error(transparent)]
    Transport(#[from] lettre::transport::smtp::Error),
    #[error(transparent)]
    Send(#[from] lettre::error::Error),
}

pub enum Payload {
    Invite {
        name: Option<String>,
        user: String,
        domain: String,
    },
    _Challenge(types::Id),
    _Match(types::Id),
}

#[derive(Debug, Clone)]
pub struct Smtp {
    tx: Option<tokio::sync::mpsc::Sender<Payload>>,
}

impl Smtp {
    pub fn new(
        host: String,
        smtp_host: String,
        smtp_port: u16,
        from_name: String,
        from_user: String,
        from_domain: String,
    ) -> Result<Self, Error> {
        let (tx, rx) = tokio::sync::mpsc::channel(16);
        let worker = Worker::new(
            host,
            smtp_host,
            smtp_port,
            from_name,
            from_user,
            from_domain,
            rx,
        )?;
        tokio::spawn(worker.listen());

        Ok(Self { tx: Some(tx) })
    }

    #[must_use]
    pub fn empty() -> Self {
        Self { tx: None }
    }

    pub fn send(&self, payload: Payload) {
        if let Some(tx) = self.tx.clone() {
            tokio::spawn(Self::send_inner(tx, payload));
        }
    }

    async fn send_inner(tx: tokio::sync::mpsc::Sender<Payload>, payload: Payload) {
        if tx.send(payload).await.is_err() {
            tracing::error!("SMTP channel closed");
        }
    }
}

struct Worker {
    host: String,
    sender: lettre::message::Mailbox,
    transport: lettre::AsyncSmtpTransport<lettre::Tokio1Executor>,
    rx: tokio::sync::mpsc::Receiver<Payload>,
}

impl Worker {
    fn new(
        host: String,
        smtp_host: String,
        smtp_port: u16,
        from_name: String,
        from_user: String,
        from_domain: String,
        rx: tokio::sync::mpsc::Receiver<Payload>,
    ) -> Result<Self, Error> {
        let sender = lettre::Address::new(from_user, from_domain)
            .map(|addr| lettre::message::Mailbox::new(Some(from_name), addr))?;

        let transport =
            lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::starttls_relay(&smtp_host)?
                .port(smtp_port)
                .build();

        tracing::info!(target: "smtp", host = %smtp_host, port = %smtp_port, "Starting SMTP service");

        Ok(Self {
            host,
            sender,
            transport,
            rx,
        })
    }

    async fn listen(mut self) {
        loop {
            let Some(payload) = self.rx.recv().await else {
                break;
            };

            if let Err(error) = self.send(payload).await {
                tracing::error!(target: "smtp", %error, "Failed to send email");
            }
        }
    }

    async fn send(&self, payload: Payload) -> Result<(), LettreError> {
        match payload {
            Payload::Invite { name, user, domain } => {
                if let Some(name) = name.as_ref() {
                    tracing::info!(target: "smtp", %name, %user, %domain, "Sending invite email");
                } else {
                    tracing::info!(target: "smtp", %user, %domain, "Sending invite email");
                }

                let name = name.unwrap_or_else(|| String::from("Player"));
                let recipient = lettre::Address::new(user, domain)
                    .map(|addr| lettre::message::Mailbox::new(Some(name.clone()), addr))?;

                let message = lettre::Message::builder()
                    .from(self.sender.clone())
                    .to(recipient)
                    .subject("Invitation to join PongElo")
                    .multipart(
                        lettre::message::MultiPart::alternative()
                            .singlepart(
                                lettre::message::SinglePart::builder()
                                    .header(lettre::message::header::ContentType::TEXT_PLAIN)
                                    .body(format!(
                                        r#"Hi {name}!

You have been invited to join PongElo!
Try it out at {host}

Happy gaming!
"#,
                                        host = self.host,
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
    <title>Invitation to join PongElo</title>
</head>
<body>
Hi {name}!

You have been invited to join PongElo!
Try it out at <a href={host}>{host}</a>

Happy gaming!
</body>
</html>
"#,
                                        host = self.host,
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
