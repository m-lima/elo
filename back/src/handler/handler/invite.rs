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
            model::Invite::Cancel(id) => {
                let id = invites
                    .cancel(self.handler.user.id, id)
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

impl<'a> Invite<'a, types::PendingUser> {
    pub async fn handle(self, request: model::Invite) -> Result<model::Response, model::Error> {
        let invites = self.handler.store.invites();

        match request {
            // TODO: Send email to inviter
            model::Invite::Accept => {
                let rating = skillratings::glicko2::Glicko2Rating::new();
                let player = invites
                    .accept(
                        self.handler.user.id,
                        &self.handler.user.email,
                        rating.rating,
                        rating.deviation,
                        rating.volatility,
                    )
                    .await
                    .map_err(model::Error::Store)
                    .and_then(|r| r.ok_or(model::Error::NotFound))?;

                self.handler.broadcaster.send(model::Push::Joined(player));

                if self.handler.smtp.present() {
                    match self.handler.store.players().get(player.inviter).await {
                        Ok(Some(inviter)) => {
                            self.handler
                                .smtp
                                .send(smtp::Payload::InviteOutcome {
                                    inviter: mailbox::Proto {
                                        name: inviter.name,
                                        email: inviter.email,
                                    },
                                    invitee: mailbox::Proto {
                                        name: self.handler.user.name.clone(),
                                        email: player.email,
                                    },
                                    accepted: true,
                                })
                                .await
                        }
                        Ok(None) | Err(_) => {
                            tracing::warn!("Could not get the original inviter's name and email");
                        }
                    }
                }

                Ok(model::Response::Done)
            }
            model::Invite::Reject => {
                let id = invites
                    .reject(self.handler.user.id, &self.handler.user.email)
                    .await
                    .map_err(model::Error::Store)
                    .and_then(|r| r.ok_or(model::Error::NotFound))?;

                self.handler.broadcaster.send(model::Push::Uninvited(id));

                if self.handler.smtp.present() {
                    match self.handler.store.players().get(player.inviter).await {
                        Ok(Some(inviter)) => {
                            self.handler
                                .smtp
                                .send(smtp::Payload::InviteOutcome {
                                    inviter: mailbox::Proto {
                                        name: inviter.name,
                                        email: inviter.email,
                                    },
                                    invitee: mailbox::Proto {
                                        name: player.name,
                                        email: player.email,
                                    },
                                    accepted: false,
                                })
                                .await
                        }
                        Ok(None) | Err(_) => {
                            tracing::warn!("Could not get the original inviter's name and email");
                        }
                    }
                }

                Ok(model::Response::Done)
            }
            model::Invite::Player(_) | model::Invite::Cancel(_) => Err(model::Error::Forbidden),
        }
    }
}
