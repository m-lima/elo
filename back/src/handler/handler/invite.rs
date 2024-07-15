use super::super::{access, model};
use crate::{mailbox, smtp};

#[derive(Debug)]
pub struct Invite<'a, A, S>
where
    A: super::Access,
    S: smtp::Smtp,
{
    handler: &'a mut super::Handler<A, S>,
}

impl<'a, A, S> Invite<'a, A, S>
where
    A: super::Access,
    S: smtp::Smtp,
{
    pub fn new(handler: &'a mut super::Handler<A, S>) -> Self {
        Self { handler }
    }
}

impl<'a, S> Invite<'a, access::Regular, S>
where
    S: smtp::Smtp,
{
    pub async fn handle(
        self,
        request: model::request::Invite,
    ) -> Result<model::Response, model::Error> {
        let invites = self.handler.store.invites();

        match request {
            model::request::Invite::List => invites
                .list()
                .await
                .map_err(model::Error::Store)
                .map(|r| model::Response::Invites(r.into_iter().map(Into::into).collect())),
            model::request::Invite::Player { name, email } => {
                let mailbox =
                    mailbox::Mailbox::new(name, email).map_err(model::Error::InvalidEmail)?;

                let invite = invites
                    .invite(self.handler.user.id(), mailbox.name(), mailbox.email())
                    .await
                    .map_err(model::Error::Store)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Player(model::push::Player::Invited(invite)));

                self.handler.smtp.send(smtp::Payload::Invite(mailbox)).await;

                Ok(model::Response::Done)
            }
            model::request::Invite::Cancel(id) => {
                let invite = invites
                    .cancel(self.handler.user.id(), id)
                    .await
                    .map_err(model::Error::Store)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Player(model::push::Player::Uninvited(invite)));

                Ok(model::Response::Done)
            }
            model::request::Invite::Accept | model::request::Invite::Reject => {
                Err(model::Error::Forbidden)
            }
        }
    }
}

impl<'a, S> Invite<'a, access::Pending, S>
where
    S: smtp::Smtp,
{
    pub async fn handle(
        self,
        request: model::request::Invite,
    ) -> Result<model::Response, model::Error> {
        let invites = self.handler.store.invites();

        match request {
            model::request::Invite::Accept => {
                let rating = skillratings::elo::EloRating::new().rating;
                let (player, initiator) = invites
                    .accept(self.handler.user.id(), rating)
                    .await
                    .map_err(model::Error::Store)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Player(model::push::Player::Joined(player)));

                self.handler
                    .smtp
                    .send(smtp::Payload::InviteOutcome {
                        inviter: mailbox::Proto {
                            name: initiator.name,
                            email: initiator.email,
                        },
                        invitee: self.handler.user.make_proto(),
                        accepted: true,
                    })
                    .await;

                Ok(model::Response::Done)
            }
            model::request::Invite::Reject => {
                let (invite, initiator) = invites
                    .reject(self.handler.user.id())
                    .await
                    .map_err(model::Error::Store)?;

                self.handler
                    .broadcaster
                    .send(model::Push::Player(model::push::Player::Uninvited(invite)));

                self.handler
                    .smtp
                    .send(smtp::Payload::InviteOutcome {
                        inviter: mailbox::Proto {
                            name: initiator.name,
                            email: initiator.email,
                        },
                        invitee: self.handler.user.make_proto(),
                        accepted: false,
                    })
                    .await;

                Ok(model::Response::Done)
            }
            model::request::Invite::List
            | model::request::Invite::Player { .. }
            | model::request::Invite::Cancel(_) => Err(model::Error::Forbidden),
        }
    }
}
