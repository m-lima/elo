use super::error::Error;
use super::payload::Payload;
use crate::mailbox;

#[derive(Debug, Clone)]
pub struct Smtp {
    tx: Option<tokio::sync::mpsc::Sender<Payload>>,
}

impl Smtp {
    pub async fn new(
        link: hyper::Uri,
        host: hyper::Uri,
        from: mailbox::Mailbox,
    ) -> Result<Self, Error> {
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

    pub fn present(&self) -> bool {
        self.tx.is_some()
    }
}

struct Worker {
    link: hyper::Uri,
    from: mailbox::Mailbox,
    transport: lettre::AsyncSmtpTransport<lettre::Tokio1Executor>,
    rx: tokio::sync::mpsc::Receiver<Payload>,
}

impl Worker {
    async fn new(
        link: hyper::Uri,
        host: hyper::Uri,
        from: mailbox::Mailbox,
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

            let (span, message) = self.build_message(payload);

            let _span = span.enter();

            let message = match message {
                Ok(message) => message,
                Err(error) => {
                    tracing::error!(%error, "Failed to build email");
                    continue;
                }
            };

            match lettre::AsyncTransport::send(&self.transport, message).await {
                Ok(response) => {
                    if response.is_positive() {
                        tracing::info!("Email sent");
                    } else if let Some(message) = response.first_line() {
                        tracing::error!(%message, "Failed to send email");
                    } else {
                        tracing::error!("Failed to send email");
                    }
                }
                Err(error) => {
                    tracing::error!(%error, "Failed to send email");
                }
            }
        }
    }

    fn build_message(
        &self,
        payload: Payload,
    ) -> (tracing::Span, Result<lettre::Message, BuildError>) {
        match payload {
            Payload::Invite(recipient) => {
                let name = String::from(recipient.name());
                let elo = self.from.name();
                let link = &self.link;

                let span = tracing::info_span!("send", kind = %"Invite", %recipient);

                let message = lettre::Message::builder()
                    .from(self.from.clone().into())
                    .to(recipient.into())
                    .subject(format!("Invitation to join {elo}"))
                    .multipart(
                        lettre::message::MultiPart::alternative()
                            .singlepart(
                                lettre::message::SinglePart::builder()
                                    .header(lettre::message::header::ContentType::TEXT_PLAIN)
                                    .body(format!(
                                        r#"Hi {name}!

You have been invited to join {elo}!
Try it out at {link}

Happy gaming!
"#
                                    )),
                            )
                            .singlepart(
                                lettre::message::SinglePart::builder()
                                    .header(lettre::message::header::ContentType::TEXT_HTML)
                                    .body(format!(
                                        r#"
<!DOCTYPE html>
<html lang = "en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Invitation to join {elo}</title>
</head>
<body>
    <p>Hi {name}!</p>

    <p>
        You have been invited to join {elo}!
        <br>
        Try it out at <a href="{link}">{link}</a>
    </p>

    <p>
        Happy gaming!
    </p>
</body>
</html>
"#
                                    )),
                            ),
                    )
                    .map_err(BuildError::Lettre);

                (span, message)
            }
            Payload::InviteOutcome {
                inviter,
                invitee,
                accepted,
            } => {
                let span = tracing::info_span!("send", kind = %{if accepted { "Accepted" } else { "Rejected" } }, recipient = %inviter);

                let Ok(recipient) = mailbox::Mailbox::try_from(inviter.clone()) else {
                    return (span, Err(BuildError::InvalidEmail(inviter)));
                };

                let outcome = if accepted { "accepted" } else { "rejected" };
                let elo = self.from.name();
                let name = inviter.name;
                let invitee = invitee.name;

                let message = lettre::Message::builder()
                    .from(self.from.clone().into())
                    .to(recipient.into())
                    .subject(format!("Invitation {outcome} by {invitee}"))
                    .body(format!(
                        r#"Hi {name}!

The user {invitee} has {outcome} your invitation to join {elo}.

Happy gaming!
"#
                    ))
                    .map_err(BuildError::Lettre);

                (span, message)
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum BuildError {
    #[error(transparent)]
    Lettre(lettre::error::Error),
    #[error("Could not create mailbox for '{0}'")]
    InvalidEmail(mailbox::Proto),
}
