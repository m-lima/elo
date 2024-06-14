use super::super::model;
use crate::{mailbox, smtp, types};

#[derive(Debug)]
pub struct Invite<'a, A>
where
    A: super::Access,
{
    handler: &'a mut super::Handler<A>,
}

impl<'a, A> Invite<'a, A>
where
    A: super::Access,
{
    pub fn new(handler: &'a mut super::Handler<A>) -> Self {
        Self { handler }
    }
}

impl<'a> Invite<'a, types::ExistingUser> {
    pub async fn handle(self, request: model::Invite) -> Result<model::Response, model::Error> {
        let invites = self.handler.store.invites();

        match request {
            model::Invite::Player(model::InvitePlayer { name, email }) => {
                let mailbox =
                    mailbox::Mailbox::new(name, email).map_err(model::Error::InvalidEmail)?;

                let id = invites
                    .invite(self.handler.user.id, mailbox.name(), mailbox.email())
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
                .cancel(self.handler.user.id, id)
                .await
                .map_err(model::Error::Store)
                .and_then(|r| r.ok_or(model::Error::NotFound))
                .map(model::Response::Id),
            model::Invite::Accept | model::Invite::Reject => Err(model::Error::Forbidden),
        }
    }
}

impl<'a> Invite<'a, types::PendingUser> {
    pub async fn handle(self, request: model::Invite) -> Result<model::Response, model::Error> {
        let _invites = self.handler.store.invites();

        match request {
            // TODO: Send email to inviter
            model::Invite::Accept => todo!(),
            // TODO: Send email to inviter
            model::Invite::Reject => todo!(),
            model::Invite::Player(_) | model::Invite::Cancel(_) => Err(model::Error::Forbidden),
        }
    }
}
