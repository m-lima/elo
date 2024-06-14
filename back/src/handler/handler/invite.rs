use super::super::{access, model};
use crate::{mailbox, smtp};

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

impl<'a> Invite<'a, access::Regular> {
    pub async fn handle(self, request: model::Invite) -> Result<model::Response, model::Error> {
        let invites = self.handler.store.invites();

        match request {
            model::Invite::Player(model::InvitePlayer { name, email }) => {
                let mailbox =
                    mailbox::Mailbox::new(name, email).map_err(model::Error::InvalidEmail)?;

                let id = invites
                    .invite(self.handler.user.id(), mailbox.name(), mailbox.email())
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
            model::Invite::Cancel(id) => {
                let id = invites
                    .cancel(self.handler.user.id(), id)
                    .await
                    .map_err(model::Error::Store)
                    .and_then(|r| r.ok_or(model::Error::NotFound))?;

                self.handler.broadcaster.send(model::Push::Uninvited(id));

                Ok(model::Response::Done)
            }
            model::Invite::Accept | model::Invite::Reject => Err(model::Error::Forbidden),
        }
    }
}

impl<'a> Invite<'a, access::Pending> {
    pub async fn handle(self, request: model::Invite) -> Result<model::Response, model::Error> {
        let invites = self.handler.store.invites();

        match request {
            model::Invite::Accept => {
                let rating = skillratings::glicko2::Glicko2Rating::new();
                let player = invites
                    .accept(
                        self.handler.user.id(),
                        rating.rating,
                        rating.deviation,
                        rating.volatility,
                    )
                    .await
                    .map_err(model::Error::Store)
                    .and_then(|r| r.ok_or(model::Error::NotFound))?;

                let inviter_id = player.inviter;
                self.handler.broadcaster.send(model::Push::Joined(player));

                if self.handler.smtp.present() {
                    if let Some(inviter_id) = inviter_id {
                        match self.handler.store.players().get(inviter_id).await {
                            Ok(Some(inviter_player)) => {
                                self.handler
                                    .smtp
                                    .send(smtp::Payload::InviteOutcome {
                                        inviter: mailbox::Proto {
                                            name: inviter_player.name,
                                            email: inviter_player.email,
                                        },
                                        invitee: self.handler.user.make_proto(),
                                        accepted: true,
                                    })
                                    .await;
                            }
                            Ok(None) | Err(_) => {
                                tracing::warn!(
                                    "Could not get the original inviter's name and email for acceptance email"
                                );
                            }
                        }
                    }
                }

                Ok(model::Response::Done)
            }
            model::Invite::Reject => {
                let invite = invites
                    .reject(self.handler.user.id())
                    .await
                    .map_err(model::Error::Store)
                    .and_then(|r| r.ok_or(model::Error::NotFound))?;

                self.handler
                    .broadcaster
                    .send(model::Push::Uninvited(invite.id));

                if self.handler.smtp.present() {
                    match self.handler.store.players().get(invite.inviter).await {
                        Ok(Some(inviter_player)) => {
                            self.handler
                                .smtp
                                .send(smtp::Payload::InviteOutcome {
                                    inviter: mailbox::Proto {
                                        name: inviter_player.name,
                                        email: inviter_player.email,
                                    },
                                    invitee: self.handler.user.make_proto(),
                                    accepted: false,
                                })
                                .await;
                        }
                        Ok(None) | Err(_) => {
                            tracing::warn!("Could not get the original inviter's name and email for rejection email");
                        }
                    }
                }

                Ok(model::Response::Done)
            }
            model::Invite::Player(_) | model::Invite::Cancel(_) => Err(model::Error::Forbidden),
        }
    }
}
