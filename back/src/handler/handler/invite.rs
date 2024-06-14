use super::super::model;
use crate::{mailbox, smtp};

#[derive(Debug)]
pub struct Invite<'a> {
    handler: &'a mut super::Handler,
}

impl<'a> Invite<'a> {
    pub fn new(handler: &'a mut super::Handler) -> Self {
        Self { handler }
    }

    pub async fn handle(self, request: model::Invite) -> Result<model::Response, model::Error> {
        let invites = self.handler.store.invites();

        match request {
            model::Invite::Player(model::InvitePlayer { name, email }) => {
                let mailbox =
                    mailbox::Mailbox::new(name, email).map_err(model::Error::InvalidEmail)?;

                let id = invites
                    .invite(self.handler.user_id, mailbox.name(), mailbox.email())
                    .await
                    .map_err(model::Error::Store)
                    .map(model::Response::Id)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Invited(model::InvitePlayer {
                        name: String::from(mailbox.name()),
                        email: String::from(mailbox.email()),
                    }));

                self.handler.smtp.send(smtp::Payload::Invite(mailbox)).await;

                Ok(id)
            }
            model::Invite::Cancel(id) => invites
                .cancel(self.handler.user_id, id)
                .await
                .map_err(model::Error::Store)
                .and_then(|r| r.ok_or(model::Error::NotFound))
                .map(model::Response::Id),
            // TODO: Send email to inviter
            model::Invite::Accept => todo!(),
            // TODO: Send email to inviter
            model::Invite::Reject => todo!(),
        }
    }
}
